use axum::{
    extract::{Query, State},
    Json,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use validator::Validate;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    http::header::AUTHORIZATION,
    async_trait,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    error::AppError,
    models::user::{AuthResponse, LoginRequest, RegisterRequest, User},
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct VerifyTokenQuery {
    token: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    token: String,
    new_password: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::Unauthorized("Missing Authorization Header".into()))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization Header".into()))?;

        if !auth_str.starts_with("Bearer ") {
             return Err(AppError::Unauthorized("Invalid Bearer Token".into()));
        }

        let token = &auth_str[7..];
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid Token".into()))?;

        Ok(token_data.claims)
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let existing_user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
        .fetch_optional(&state.pool)
        .await?;

    if let Some(existing) = existing_user {
        if existing.is_verified {
            return Err(AppError::Conflict("A user with this email address already exists".into()));
        } else {
             // User exists but unverified. Resend token.
            let new_token = Uuid::new_v4().to_string();
            let user = sqlx::query_as!(
                User,
                "UPDATE users SET verification_token = $1, verification_token_expires_at = NOW() + INTERVAL '24 hours' WHERE id = $2 RETURNING *",
                new_token,
                existing.id
            )
            .fetch_one(&state.pool)
            .await?;

            if let Err(e) = state.email_service.send_verification_email(&user.email, &new_token) {
                 tracing::error!("Failed to resend verification email: {:?}", e);
            }

            let token = generate_token(&user.id.to_string())?;
            return Ok(Json(AuthResponse { token, user }));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    let verification_token = Uuid::new_v4().to_string();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, verification_token, verification_token_expires_at)
        VALUES ($1, $2, $3, NOW() + INTERVAL '24 hours')
        RETURNING *
        "#,
        payload.email,
        password_hash,
        verification_token
    )
    .fetch_one(&state.pool)
    .await?;

    // Send verification email (non-blocking in a real app, but blocking here for simplicity)
    // We log the error but don't fail the registration if email fails (or we could fail it)
    if let Err(e) = state.email_service.send_verification_email(&user.email, &verification_token) {
        tracing::error!("Failed to send verification email: {:?}", e);
    }

    let token = generate_token(&user.id.to_string())?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("Unknown e-mail".into()))?;

    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or(AppError::Unauthorized("Account uses Google Login".into()))?;

    let parsed_hash = PasswordHash::new(password_hash)?;
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AppError::Unauthorized("Wrong password".into()));
    }

    // Removed check for is_verified to allow account management access
    // if !user.is_verified { ... }

    let token = generate_token(&user.id.to_string())?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyTokenQuery>,
) -> Result<Json<&'static str>, AppError> {
    let result = sqlx::query!(
        "UPDATE users SET is_verified = TRUE, verification_token = NULL, verification_token_expires_at = NULL WHERE verification_token = $1 AND verification_token_expires_at > NOW()",
        query.token
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::BadRequest("Invalid or expired token".into()));
    }

    Ok(Json("Email verified successfully"))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<&'static str>, AppError> {
    let token = Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

    let result = sqlx::query!(
        "UPDATE users SET reset_password_token = $1, reset_password_expires_at = $2 WHERE email = $3",
        token,
        expires_at,
        payload.email
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() > 0 {
        if let Err(e) = state.email_service.send_password_reset_email(&payload.email, &token) {
             tracing::error!("Failed to send reset email: {:?}", e);
             return Err(AppError::InternalServerError);
        }
    }

    // Always return OK to prevent email enumeration
    Ok(Json("If an account exists, a reset email has been sent"))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<&'static str>, AppError> {
    let user = sqlx::query!(
        "SELECT id FROM users WHERE reset_password_token = $1 AND reset_password_expires_at > $2",
        payload.token,
        chrono::Utc::now()
    )
    .fetch_optional(&state.pool)
    .await?;

    if user.is_none() {
        return Err(AppError::BadRequest("Invalid or expired token".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.new_password.as_bytes(), &salt)?
        .to_string();

    sqlx::query!(
        "UPDATE users SET password_hash = $1, reset_password_token = NULL, reset_password_expires_at = NULL WHERE reset_password_token = $2",
        password_hash,
        payload.token
    )
    .execute(&state.pool)
    .await?;

    Ok(Json("Password reset successfully"))
}

pub fn generate_token(user_id: &str) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::JwtError)
}

pub async fn delete_account(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<&'static str>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized("Invalid User ID in token".into()))?;
    
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&state.pool)
        .await?;

    Ok(Json("Account deleted successfully"))
}
