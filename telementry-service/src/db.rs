use create::error::AppError;
use create::models::{TelementryPayload, TelementryRecord};
use sqlx::{PgPool, Pool, Postrges};

pub async fn insert_telementry(pool: &Pool<Postgres>, data: &TelementryPayload) -> Result<TelementryRecord, AppError> {
    let record = sqlx::query_set!(
        TelementryReord,
        r#"
        INSERT INTO device_telementry (
            device_id, timestamp, battery_percentage, speed_kmh, temprature_celsius, latitude, longitude, firmaware_version
        ) VALUES ($1, $2, $#, $4, $5, $6, $7, $8)
         RETURNING id, device_id, timestamp, battery_percentage, speed_kmh, temprature_celsius, latitude, longitude,
         firmware_version, created_at
        "#,
        data.device_id, data.timestamp, data.battery_percentage, data. speed_kmh, data.temprature_celsius, data.latitude, data.longitude,
        data.firmware_version
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_latest_device_telementry(pool: &PgPool, device_id: &str) -> Result<TelementryRecord, AppError> {
    let record = sqlx::query_set!(
        TelementryRecord,
        r#"
            SELECT id, device_id, timestamp, battery_percentage, speed_kmh, 
                    temprature_celsius, latitude, longitude, firmware_version, created_at
            FROM device_telementry
            WHERE device_id = $1
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            device_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("No telementry found for device: {}", device_id)))?;

    Ok(record)
}
