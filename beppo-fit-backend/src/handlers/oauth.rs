use axum::{
    extract::{Query, State},
    response::{Redirect, IntoResponse},
};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenUrl, TokenResponse, reqwest::async_http_client,
};
use serde::Deserialize;
use std::env;

use crate::{
    error::AppError,
    models::user::User,
    state::AppState,
    handlers::auth::generate_token,
};

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    #[allow(dead_code)]
    state: String,
}

#[derive(Deserialize)]
struct GoogleUser {
    id: String,
    email: String,
    verified_email: bool,
}

pub async fn google_login() -> Result<impl IntoResponse, AppError> {
    let client = oauth_client()?;
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let client = oauth_client()?;
    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("OAuth token exchange failed: {:?}", e);
            AppError::InternalServerError
        })?;

    let client = reqwest::Client::new();
    let google_user: GoogleUser = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|_| AppError::InternalServerError)?
        .json()
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if !google_user.verified_email {
        return Err(AppError::Unauthorized("Google email not verified".into()));
    }

    // Upsert user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, google_id, is_verified)
        VALUES ($1, $2, TRUE)
        ON CONFLICT (email) DO UPDATE
        SET google_id = $2, is_verified = TRUE
        RETURNING *
        "#,
        google_user.email,
        google_user.id
    )
    .fetch_one(&state.pool)
    .await?;

    let token = generate_token(&user.id.to_string())?;

    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4200".to_string());
    let redirect_url = format!("{}/auth/google/callback?token={}", frontend_url, token);

    Ok(Redirect::to(&redirect_url))
}

fn oauth_client() -> Result<BasicClient, AppError> {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET");
    let redirect_url = env::var("GOOGLE_REDIRECT_URL").expect("Missing GOOGLE_REDIRECT_URL");

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|_| AppError::InternalServerError)?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .map_err(|_| AppError::InternalServerError)?;

    Ok(
        BasicClient::new(ClientId::new(client_id), Some(ClientSecret::new(client_secret)), auth_url, Some(token_url))
            .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|_| AppError::InternalServerError)?)
    )
}
