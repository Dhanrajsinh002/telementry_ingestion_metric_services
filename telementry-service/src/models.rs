use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelementryPayload {
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub battery_percentage: i16,
    pub speed_kmh: f32,
    pub temprature_celsius: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub firmware_version: String,
}

impl TelementryPayload {
    /// Validate sensor data censtraints before db insertion
    pub fn validate(&self) -> Result<(), String> {
        if self.device_id.is_empty() {
            return Err("device_id cannot be empty".to_string());
        }
        if self.battery_percentage < 0 || self.battery_percentage > 100 {
            return Err("battery_percentage must be between 0 and 100".to_string());
        }
        if self.speed_kmh < 0.0 || self.speed_kmh > 400.0 {
            return Err("speed_kmh exceeds plausible vehicle boundries (0-400)".to_string());
        }
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err("latitude must be between -90 and 90".to_string());
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err("longitude must be between -180 and 180".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TelementryRecord {
    pub id: Uuid,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub battery_percentage: i16,
    pub speed_kmh: f32,
    pub temprature_celsius: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub firmware_version: String,
    pub created_at: DateTime<Utc>,
}
