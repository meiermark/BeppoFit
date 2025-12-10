use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use beppo_fit_backend::app;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn test_verification_token_expiration(pool: PgPool) {
    let app = app(pool.clone()).await;

    // 1. Register a user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/register")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "email": "expire_test@example.com",
                        "password": "Password123!"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Get the user from DB to find the token
    let user_row = sqlx::query!(
        "SELECT verification_token FROM users WHERE email = 'expire_test@example.com'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let token = user_row.verification_token.unwrap();

    // 2. Manually expire the token (set to yesterday)
    sqlx::query!("UPDATE users SET verification_token_expires_at = NOW() - INTERVAL '1 day' WHERE email = 'expire_test@example.com'")
        .execute(&pool)
        .await
        .unwrap();

    // 3. Try to verify
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/auth/verify?token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 4. Expect BadRequest (or whatever the logic returns for expired/invalid)
    // Based on `verify_email` handler: if token checks fail, it usually returns BadRequest ("Invalid or expired verification token")
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body_json["error"], "Invalid or expired token");
}
