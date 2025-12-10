use lettre::{
    message::header::ContentType, Message,
    SmtpTransport, Transport,
};


use std::env;

use crate::error::AppError;

pub struct EmailService {
    mailer: SmtpTransport,
    frontend_url: String,
}

impl EmailService {
    pub fn new() -> Self {
        let host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".to_string())
            .parse::<u16>()
            .expect("Invalid SMTP_PORT");

        // For Mailhog, we don't strictly need auth, but structure is here
        let mailer = SmtpTransport::builder_dangerous(&host).port(port).build();
        let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:80".to_string());

        Self { mailer, frontend_url }
    }

    pub fn send_verification_email(&self, to_email: &str, token: &str) -> Result<(), AppError> {
        let subject = "Verify your BeppoFit account";
        let body = format!(
            "Welcome to BeppoFit! Click here to verify your account: http://localhost:8080/auth/verify?token={}",
            token
        );

        self.send_email(to_email, subject, &body)
    }

    pub fn send_password_reset_email(&self, to_email: &str, token: &str) -> Result<(), AppError> {
        let subject = "Reset your BeppoFit password";
        let body = format!(
            "Click here to reset your password: {}/auth/reset-password?token={}",
            self.frontend_url, token
        );

        self.send_email(to_email, subject, &body)
    }

    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppError> {
        let email = Message::builder()
            .from("BeppoFit <noreply@beppofit.com>".parse().unwrap())
            .to(to.parse().map_err(|_| AppError::BadRequest("Invalid email address".into()))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(body))
            .map_err(|_e| AppError::InternalServerError)?;

        self.mailer
            .send(&email)
            .map_err(|e| {
                tracing::error!("Failed to send email: {:?}", e);
                AppError::InternalServerError
            })?;

        Ok(())
    }
}
