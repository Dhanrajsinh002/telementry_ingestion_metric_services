mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Inititalize structured logging
    let default_log = if cfg!(debug_assertions) {
        "telementry_service=debug,tower_http=debug"
    } else {
        "telementry_service=info,tower_http=info"
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_log.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL env must be set");
    let max_conn = std::env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "20".to_string())
        .parse::<u32>()
        .unwrap_or(20);
    let conn_timeout = std::env::var("DATABASE_CONNECT_TIMEOUT_SECS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<u64>()
        .unwrap_or(8);

    // Configure conenction pool
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .acquire_timeout(std::time::Duration::from_secs(conn_timeout))
        .connect(&database_url)
        .await?;

    tracing::info!("Connected to PostgreSQL database pool");

    // Build Axum Router
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/v1/telementry", post(handlers::insert_telementry_handler))
        .route("/api/v1/devices/:device_id/latest", get(handlers::get_telementry_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be valid u16 number");

    let addr = format("{}:{}", host, port).parse().expect("Invalid HOST or PORT address");
    tracing::info!("Telementry ingestion service listing on {}", addr);

    let listner = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listner, app).await?;

    Ok(())
}
