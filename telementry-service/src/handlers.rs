use create::db;
use create::error::AppError;
use create::models::{TelementryPayload, TelementryRecord};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

#[tracing::instrument(skip(pool, payload), fields(device_id = %payload.device_id))]
pub async fn insert_telementry_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<TelementryRecord>,
) -> Result<(StatusCode, Json<Telementryrecord>), AppError> {
    // Validate boundries
    payload.validate().map_err(AppError::ValidationError)?;

    // Persist via async connection pool
    let record = db::insert_telementry(&pool, &payload).await?;
    tracing::info!("Telementry ingested successfully for ID: {}", record.id);

    Ok((StatusCode::CREATED, Json(record)))
}

#[tracing::instrument(skip(pool))]
pub async fn get_telementry_handler(
    State(pool): State<PgPool>,
    Path(device_id): Path<String>,
) -> Result<(Json<TelementryRecord>), AppError> {
    let record = db::get_latest_telementry(&pool, &device_id).await?;
    Ok(Json(record))
}

pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Telementry service is healthy")
}
