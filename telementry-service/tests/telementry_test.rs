use chrono::Utc;
use telementry_service::models::TelementryPayload;

#[test]
fn test_payload_validation_rejects_impossible_battery() {
    let invalid_payload = TelementryPayload {
        device_id: "VEHICLE_001".into(),
        timestamp: Utc::now(),
        battery_percentage: 150, // Invalid!
        speed_kmh: 45.0,
        temperature_celsius: 28.5,
        latitude: 12.9716,
        longitude: 77.5946,
        firmware_version: "v1.2.0".into(),
    };
    assert!(invalid_payload.validate().is_err());
}

#[test]
fn test_payload_validation_accepts_valid_telementry() {
    let valid_payload = TelementryPayload {
        device_id: "VEHICLE_001".into(),
        timestamp: Utc::now(),
        battery_percentage: 82,
        speed_kmh: 60.5,
        temperature_celsius: 24.0,
        latitude: 12.9716,
        longitude: 77.5946,
        firmware_version: "v1.2.0".into(),
    };
    assert!(valid_payload.validate().is_ok());
}
