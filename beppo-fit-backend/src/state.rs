use crate::services::email::EmailService;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub email_service: Arc<EmailService>,
}
