use sqlx::PgPool;
use crate::services::email::EmailService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub email_service: Arc<EmailService>,
}
