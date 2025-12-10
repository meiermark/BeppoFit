use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod state;

use crate::{services::email::EmailService, state::AppState};

use sqlx::PgPool;

pub async fn app(pool: PgPool) -> Router {
    let email_service = Arc::new(EmailService::new());

    let state = AppState {
        pool,
        email_service,
    };

    Router::new()
        .route("/", get(root))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/verify", get(handlers::auth::verify_email))
        .route(
            "/auth/forgot-password",
            post(handlers::auth::forgot_password),
        )
        .route("/auth/reset-password", post(handlers::auth::reset_password))
        .route("/auth/google", get(handlers::oauth::google_login))
        .route(
            "/auth/google/callback",
            get(handlers::oauth::google_callback),
        )
        .route(
            "/auth/me",
            axum::routing::delete(handlers::auth::delete_account),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, BeppoFit!"
}
