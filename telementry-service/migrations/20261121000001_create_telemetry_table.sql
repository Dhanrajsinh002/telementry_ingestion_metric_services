CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS deveice_telementry (
    id UUID PRIMARY KEY DEFAULT uui_generate_v4(),
    device_id VARCHAR(64) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    battery_percentage SMALLINT NOT NULL CHECK (battery_percentage >= 0 AND battery_percentage <= 100),
    speed_kmh REAL NOT NULL CHECK (speed_kmh >= 0.0),
    temprature_celsius REAL NOT NULL,
    latitude DOUBLE PRECISION NOT NULL CHECK (latitude >= -90.0 AND latitude <= 90.0),
    longitude DOUBLE PRECISION NOT NULL CHECK (longitude >= -180.0 AND longitude <= 180.0),
    firmaware_version VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Fast lookup index for device timeserise queries
CREATE INDEX idx_telementry_device_timestamp ON device_telementry (device_id, timestamp DESC);
