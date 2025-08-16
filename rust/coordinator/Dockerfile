# Build stage
FROM rust:1.88-bookworm AS builder

# Install dependencies in a separate layer
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy workspace Cargo.toml and all crate Cargo files for dependency caching
COPY Cargo.toml ./
COPY coordinator/Cargo.toml ./coordinator/
COPY coordinator-common/Cargo.toml ./coordinator-common/
COPY common/Cargo.toml ./common/


# Copy source files

COPY coordinator/src ./coordinator/src/
COPY coordinator-common/src ./coordinator-common/src/
COPY common/src ./common/src/

# Copy Cargo.lock for reproducible builds
COPY Cargo.lock ./

# Build the application
RUN cargo build --release -p sessio-coordinator && \
    strip target/release/sessio-coordinator

# Runtime stage
FROM debian:bookworm-slim

# Accept build arguments for configuration
ARG DATABASE_URL
ARG RUST_LOG="info"
ARG LISTEN_ADDR="0.0.0.0:2223"
ARG USE_HTTP="true"
ARG CERT_FILE="/app/certs/fullchain.pem"
ARG KEY_FILE="/app/certs/privkey.pem"

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash coordinator

# Copy binary from builder
COPY --from=builder /app/target/release/sessio-coordinator /usr/local/bin/

# Create necessary directories
RUN mkdir -p /app/certs && \
    chown -R coordinator:coordinator /app

USER coordinator
WORKDIR /app

# Expose coordinator port
EXPOSE 2223

# Set environment variables from build args
ENV RUST_LOG=${RUST_LOG}
ENV DATABASE_URL=${DATABASE_URL}
ENV LISTEN_ADDR=${LISTEN_ADDR}
ENV USE_HTTP=${USE_HTTP}
ENV CERT_FILE=${CERT_FILE}
ENV KEY_FILE=${KEY_FILE}

# Use JSON format CMD with sh -c for conditional logic
CMD ["/bin/sh", "-c", "if [ \"$USE_HTTP\" = \"true\" ]; then exec sessio-coordinator --listen \"$LISTEN_ADDR\" --database-url \"$DATABASE_URL\" --dangerously-use-http; else exec sessio-coordinator --listen \"$LISTEN_ADDR\" --database-url \"$DATABASE_URL\" --cert-file \"$CERT_FILE\" --key-file \"$KEY_FILE\"; fi"]