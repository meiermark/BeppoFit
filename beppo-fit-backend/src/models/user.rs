use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
    pub is_verified: bool,
    #[serde(skip)]
    #[allow(dead_code)]
    pub verification_token: Option<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub verification_token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub reset_password_token: Option<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub reset_password_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}
