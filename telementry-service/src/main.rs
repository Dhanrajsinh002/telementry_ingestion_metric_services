mod config;
mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
}
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv.ok();

    // Inititalize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "telementry_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let dayabase_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/telementry_db".into());

    // Configure conenction pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    tracing::info!("Connected to PostgreSQL database pool");

    // Build Axum Router
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/v1/telementry", post(handlers::ingest_telementry_handler))
        .route("/api/v1/devices/:device_id/latest", get(handlers::get_telementry_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr = SocketAddr::from(([0,0,0,0], 8080));
    tracing::info!("Telementry ingestion service listing on {}", addr);

    let listner = tokio::net::TcpListner::bind(addr).await?;
    axum::serve(listner, app).await?;

    Ok(())
}
