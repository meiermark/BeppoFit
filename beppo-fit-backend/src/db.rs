use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn init_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut retries = 5;
    let mut pool = None;

    while retries > 0 {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
        {
            Ok(p) => {
                pool = Some(p);
                break;
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to connect to database: {:?}. Retrying in 2 seconds...",
                    e
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                retries -= 1;
            }
        }
    }

    let pool = pool.expect("Failed to connect to database after retries");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
