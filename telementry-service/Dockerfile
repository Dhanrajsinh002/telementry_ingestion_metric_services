# ==============================================================================
# Multi-Stage Dockerfile for High-Performance Rust Microservice
# ==============================================================================

# ------------------------------------------------------------------------------
# Stage 1: Build Environment (Debian Bookworm + Rust toolchain)
# ------------------------------------------------------------------------------
FROM rust:1.80-bookworm AS builder

WORKDIR /usr/src/app

# Install build dependencies for TLS and C bindings
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Optimize layer caching: Copy manifest first
COPY Cargo.toml ./

# Create dummy source tree to pre-compile and cache external crate dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy real source code and migrations
COPY src ./src
COPY migrations ./migrations

# Touch main.rs to invalidate dummy build and compile release binary
RUN touch src/main.rs && cargo build --release

# ------------------------------------------------------------------------------
# Stage 2: Minimal Production Runtime
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install runtime shared libraries and curl for health checks
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create isolated non-root system user
RUN useradd -m -u 1001 -U -s /bin/sh appuser

WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /usr/src/app/target/release/telementry-service /usr/local/bin/telementry-service

# Copy migration files for startup schema migration execution
COPY --from=builder /usr/src/app/migrations /app/migrations

# Ensure permissions
RUN chown -R appuser:appuser /app

USER appuser

# Expose HTTP port
EXPOSE 3000

# Container liveness check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Launch the service
CMD ["telementry-service"]
