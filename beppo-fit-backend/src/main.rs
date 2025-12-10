use std::net::SocketAddr;
use beppo_fit_backend::app;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize database connection and run migrations
    let pool = beppo_fit_backend::db::init_pool().await;

    let app = app(pool).await;

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

