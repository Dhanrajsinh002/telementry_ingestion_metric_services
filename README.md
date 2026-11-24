# 🚀 Telemetry Ingestion & Metrics Service

[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Async%20Runtime-Tokio%201.38-black?style=flat-square&logo=tokio)](https://tokio.rs/)
[![Axum](https://img.shields.io/badge/Web%20Framework-Axum%200.7-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![SQLx](https://img.shields.io/badge/Database-SQLx%200.7%20(PostgreSQL)-336791?style=flat-square&logo=postgresql&logoColor=white)](https://github.com/launchbadge/sqlx)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

A high-performance, memory-safe, and asynchronous backend microservice built in **Rust** designed for ingesting, validating, and persisting high-frequency telemetry and sensor metrics from connected vehicles and IoT hardware fleets.

Engineered with **Tokio**, **Axum**, and **SQLx**, this service provides sub-millisecond serialization, compile-time verified database interactions, structured observability, and resilient error recovery under heavy I/O loads.

---

## 📑 Table of Contents

- [Overview](#-overview)
- [Key Features](#-key-features)
- [System Architecture](#-system-architecture)
- [Technology Stack](#-technology-stack)
- [Repository Structure](#-repository-structure)
- [API Reference](#-api-reference)
- [Database Schema & Indexing](#-database-schema--indexing)
- [Getting Started](#-getting-started)
  - [Prerequisites](#prerequisites)
  - [Environment Setup](#environment-setup)
  - [Database Initialization](#database-initialization)
  - [Running the Service](#running-the-service)
  - [Running Tests](#running-tests)
- [Engineering Highlights](#-engineering-highlights)
- [Author & Contact](#-author--contact)

---

## 🌐 Overview

In connected vehicle ecosystems (such as fleet management, battery health monitoring, and remote diagnostics), field devices stream continuous telemetry packets containing GPS coordinates, speed, battery percentage, motor temperatures, and firmware versions.

This service acts as the ingestion gateway:
1. **Validates Telemetry Packets**: Enforces physical sensor boundary constraints (e.g., latitude/longitude limits, speed thresholds, realistic temperature ranges).
2. **Asynchronous Connection Pooling**: Leverages non-blocking PostgreSQL pools via `SQLx` to maximize throughput without thread-starvation.
3. **Optimized Time-Series Lookups**: Utilizes composite indices `(device_id, timestamp DESC)` to guarantee $O(\log N)$ latest-state queries across millions of records.
4. **Structured Tracing**: Instruments every inbound request with distributed contextual spans for end-to-end debugging.

---

## ⚡ Key Features

- **Blazing Fast Ingestion**: Built on `Tokio`'s work-stealing multi-threaded runtime and `Axum`'s zero-copy routing.
- **Type-Safe Payloads & Strict Boundaries**: Custom domain validation prevents corrupted sensor data from reaching the persistence tier.
- **Compile-Time Verified Queries**: Uses `sqlx::query!` macros to ensure SQL queries are checked against the live database schema at build time.
- **Granular Error Handling**: Custom `AppError` enum using `thiserror` mapping domain anomalies directly into standardized HTTP status codes (`400`, `422`, `404`, `500`).
- **Distributed Observability**: Integrated with `tracing` and `tracing-subscriber` for JSON-formatted logs with dynamic request fields (`device_id`).

---

## 🏗️ System Architecture

```mermaid
flowchart TD
    subgraph IoT_Fleet ["Connected Fleet / Edge Devices"]
        D1["Vehicle ECU / OBD-II Tracker"]
        D2["Battery Management Unit (BMS)"]
        D3["Firmware Diagnostics Node"]
    end

    subgraph Gateway ["Rust Ingestion Layer (Axum + Tokio)"]
        Router["Axum HTTP Router (:3000)"]
        Validation["Serde Deserializer & Sensor Boundary Validator"]
        Tracing["Tracing Middleware (Span per device_id)"]
        ErrHandler["Central Error Handler (thiserror -> IntoResponse)"]
    end

    subgraph Storage ["Persistence Layer (PostgreSQL)"]
        Pool["SQLx Async Connection Pool"]
        DB[("PostgreSQL Database
        Table: device_telemetry
        Index: (device_id, timestamp DESC)")]
    end

    subgraph Consumers ["Downstream Workflows"]
        FleetDash["Fleet Dashboard / Analytics"]
        Alerts["Remote Diagnostics / Alerting Service"]
    end

    D1 -->|"POST /api/v1/telemetry"| Router
    D2 -->|"POST /api/v1/telemetry"| Router
    D3 -->|"POST /api/v1/telemetry"| Router

    Router --> Tracing
    Tracing --> Validation
    Validation -->|"Invalid Payload"| ErrHandler
    Validation -->|"Validated Batch"| Pool

    Pool -->|"Async Batch Insert"| DB

    FleetDash -->|"GET /api/v1/devices/{id}/latest"| Router
    Alerts -->|"GET /health"| Router
    DB -->|"Index Scan ($O(\log N)$)"| Pool
```

---

## 🛠️ Technology Stack

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Language** | **Rust (Edition 2024)** | Memory safety, zero-cost abstractions, data-race prevention |
| **Async Runtime** | **Tokio 1.38** | Asynchronous execution, thread scheduling, timers, non-blocking I/O |
| **Web Framework** | **Axum 0.7** | Ergonomic, high-speed HTTP routing and state extraction |
| **Middleware** | **Tower / Tower-HTTP** | Trace logging, timeout management, CORS enforcement |
| **Database Client** | **SQLx 0.7** | Pure async PostgreSQL driver with compile-time SQL verification |
| **Database** | **PostgreSQL 16** | Relational & time-series storage with composite B-Tree indexes |
| **Serialization** | **Serde & Serde JSON** | High-performance JSON deserialization with field renaming |
| **Observability** | **Tracing & Tracing-Subscriber** | Structured logging with scoped span context |
| **Error Handling** | **Thiserror & Anyhow** | Idiomatic error modeling implementing Axum `IntoResponse` |
| **Date & Time** | **Chrono (UTC)** | ISO-8601 compliant timestamps |

---

## 📂 Repository Structure

```text
telementry_ingestion_metric_services/
├── README.md                                # Root documentation & architecture overview
├── RUST_TELEMETRY_PROJECT_GUIDE.md          # Complete implementation blueprint & interview guide
└── telementry-service/                      # Main Rust binary crate
    ├── Cargo.toml                           # Crate manifest & pinned dependencies
    ├── docker-compose.yml                   # Containerized PostgreSQL service
    ├── .env.example                         # Environment configuration template
    ├── migrations/                          # Database migration scripts
    │   └── 20261121000001_create_telemetry_table.sql
    ├── src/
    │   ├── main.rs                          # Runtime bootstrapper, DB pool init & server listener
    │   ├── models.rs                        # Data models & sensor physical boundary validation
    │   ├── db.rs                            # Async SQLx queries (insert & latest record lookup)
    │   ├── handlers.rs                      # Axum HTTP route handlers
    │   └── error.rs                         # Custom AppError enum & HTTP response serialization
    └── tests/
        └── telemetry_test.rs                # Integration & unit test fixtures
```

---

## 📡 API Reference

### 1. Ingest Telemetry Data
- **Endpoint:** `POST /api/v1/telemetry`
- **Content-Type:** `application/json`
- **Description:** Receives and validates real-time telemetry from an edge device, persisting it into PostgreSQL.

#### Request Body:
```json
{
  "device_id": "VH-EV-9081",
  "timestamp": "2026-11-24T10:45:00Z",
  "battery_percentage": 87,
  "speed_kmh": 64.5,
  "temprature_celsius": 32.8,
  "latitude": 12.9716,
  "longitude": 77.5946,
  "firmware_version": "v2.4.1"
}
```

#### Success Response (`201 Created`):
```json
{
  "id": "b73a4b0d-773a-493f-9eb9-6a9ec1f2518e",
  "device_id": "VH-EV-9081",
  "timestamp": "2026-11-24T10:45:00Z",
  "battery_percentage": 87,
  "speed_kmh": 64.5,
  "temprature_celsius": 32.8,
  "latitude": 12.9716,
  "longitude": 77.5946,
  "firmware_version": "v2.4.1",
  "created_at": "2026-11-24T10:45:01.120Z"
}
```

#### Validation Error Response (`422 Unprocessable Entity`):
```json
{
  "error": "Validation failed: speed_kmh exceeds plausible vehicle boundaries (0-400)"
}
```

---

### 2. Fetch Latest Device Telemetry
- **Endpoint:** `GET /api/v1/devices/{device_id}/latest`
- **Description:** Retrieves the single most recent telemetry packet recorded for the specified device ID using the indexed timestamp lookup.

#### Success Response (`200 OK`):
```json
{
  "id": "b73a4b0d-773a-493f-9eb9-6a9ec1f2518e",
  "device_id": "VH-EV-9081",
  "timestamp": "2026-11-24T10:45:00Z",
  "battery_percentage": 87,
  "speed_kmh": 64.5,
  "temprature_celsius": 32.8,
  "latitude": 12.9716,
  "longitude": 77.5946,
  "firmware_version": "v2.4.1",
  "created_at": "2026-11-24T10:45:01.120Z"
}
```

#### Not Found Response (`404 Not Found`):
```json
{
  "error": "No telemetry found for device: VH-UNKNOWN-0000"
}
```

---

### 3. Service Health Check
- **Endpoint:** `GET /health`
- **Description:** Liveness probe for container orchestrators (Docker / Kubernetes).

#### Success Response (`200 OK`):
```text
Telemetry service is healthy
```

---

## 🗄️ Database Schema & Indexing

The schema is optimized for write-heavy IoT workloads and fast temporal retrieval:

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS device_telemetry (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    device_id VARCHAR(64) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    battery_percentage SMALLINT NOT NULL CHECK (battery_percentage >= 0 AND battery_percentage <= 100),
    speed_kmh REAL NOT NULL CHECK (speed_kmh >= 0.0),
    temprature_celsius REAL NOT NULL,
    latitude DOUBLE PRECISION NOT NULL CHECK (latitude >= -90.0 AND latitude <= 90.0),
    longitude DOUBLE PRECISION NOT NULL CHECK (longitude >= -180.0 AND longitude <= 180.0),
    firmware_version VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Composite B-Tree Index for sub-millisecond latest-record lookups:
CREATE INDEX IF NOT EXISTS idx_telemetry_device_timestamp 
ON device_telemetry (device_id, timestamp DESC);
```

### Why this index matters:
Without this index, finding the latest telemetry for a device requires a full table scan across millions of rows ($O(N)$). With `(device_id, timestamp DESC)`, PostgreSQL performs an index range scan directly locating the matching `device_id` leaf and picking the top entry ($O(\log N)$ complexity).

---

## 🚀 Getting Started

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (`rustc 1.78+` / `cargo`)
- [Docker & Docker Compose](https://www.docker.com/)
- [SQLx CLI](https://github.com/launchbadge/sqlx):
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

---

### Environment Setup

Navigate to the service directory and configure the environment:
```bash
cd telementry-service
cp .env.example .env
```

Ensure your `.env` contains:
```env
PORT=3000
DATABASE_URL=postgres://telemetry_user:telemetry_pass@localhost:5432/telemetry_db
RUST_LOG=info,telementry_service=debug,tower_http=debug
```

---

### Database Initialization

1. **Start PostgreSQL via Docker**:
   ```bash
   docker compose up -d
   ```

2. **Execute Migrations**:
   ```bash
   sqlx database create
   sqlx migrate run
   ```

---

### Running the Service

Start the application with cargo:
```bash
cargo run
```

The service will bind to `http://0.0.0.0:3000` with structured logging enabled.

To test ingestion via `curl`:
```bash
curl -X POST http://localhost:3000/api/v1/telemetry \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "TEST-VEHICLE-01",
    "timestamp": "2026-11-24T12:00:00Z",
    "battery_percentage": 92,
    "speed_kmh": 55.4,
    "temprature_celsius": 29.5,
    "latitude": 12.9716,
    "longitude": 77.5946,
    "firmware_version": "v1.0.0"
  }'
```

Query the latest telemetry:
```bash
curl http://localhost:3000/api/v1/devices/TEST-VEHICLE-01/latest
```

---

### Running Tests

Execute unit and integration tests:
```bash
cargo test
```

---

## 💡 Engineering Highlights

### 1. Robust Domain Validation (Fail-Fast)
Rather than executing redundant database round-trips for invalid sensor readings, validation occurs immediately upon deserialization:
- `battery_percentage` strictly in `[0, 100]`
- `speed_kmh` bounded to realistic vehicle dynamics `[0.0, 400.0]`
- `latitude` / `longitude` verified against WGS-84 coordinate limits `[-90.0, 90.0]` and `[-180.0, 180.0]`

### 2. Zero-Cost Compile-Time SQL Verification
By utilizing `sqlx::query!`, syntax errors, missing columns, or type mismatches between Rust structs and PostgreSQL columns are caught at **compile time**, completely eliminating whole classes of runtime SQL crashes.

### 3. Non-Blocking Async Architecture
All database calls are scheduled asynchronously on the `Tokio` threadpool. Connection acquisition timeout limits and pool size constraints prevent cascade failures during sudden ingestion spikes.

---

## 👨‍💻 Author

**Dhanrajsinh Parmar**  
- **GitHub**: [github.com/Dhanrajsinh12](https://github.com/Dhanrajsinh12)  
- **LinkedIn**: [linkedin.com/in/dhanrajsinh-parmar](https://www.linkedin.com/in/dhanrajsinh-parmar-96348325a/)  
