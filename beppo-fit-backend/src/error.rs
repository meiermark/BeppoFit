use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    SqlxError(sqlx::Error),
    PasswordHashError(argon2::password_hash::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Unauthorized(ref msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            AppError::Conflict(ref msg) => (StatusCode::CONFLICT, msg.as_str()),
            AppError::SqlxError(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database Error")
            }
            AppError::PasswordHashError(ref e) => {
                tracing::error!("Password hash error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Password Processing Error",
                )
            }
            AppError::JwtError(ref e) => {
                tracing::error!("JWT error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Token Error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::SqlxError(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err)
    }
}
