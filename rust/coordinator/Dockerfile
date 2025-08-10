# Build stage
FROM rust:1.88-bookworm AS builder

# Install dependencies in a separate layer
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy coordinator, coordinator-common, and common Cargo files for dependency caching
COPY coordinator/Cargo.toml ./coordinator/
COPY coordinator-common/Cargo.toml ./coordinator-common/
COPY common/Cargo.toml ./common/

# Create a minimal workspace Cargo.toml including coordinator-common
RUN echo '[workspace]' > Cargo.toml && \
    echo 'members = ["coordinator", "coordinator-common", "common"]' >> Cargo.toml && \
    echo 'resolver = "2"' >> Cargo.toml && \
    echo '' >> Cargo.toml && \
    echo '[workspace.package]' >> Cargo.toml && \
    echo 'version = "0.4.0"' >> Cargo.toml && \
    echo 'edition = "2021"' >> Cargo.toml && \
    echo 'authors = ["Sessio Contributors"]' >> Cargo.toml && \
    echo 'license = "MIT"' >> Cargo.toml && \
    echo 'repository = "https://github.com/0xC0ffee1/sessio"' >> Cargo.toml && \
    echo '' >> Cargo.toml && \
    echo '[workspace.dependencies]' >> Cargo.toml && \
    echo 'homedir = "=0.3.3"' >> Cargo.toml && \
    echo 'base64ct = "=1.6.0"' >> Cargo.toml


# Copy source files

COPY coordinator/src ./coordinator/src/
COPY coordinator-common/src ./coordinator-common/src/
COPY common/src ./common/src/

# Copy only the Cargo.lock from the real workspace
COPY Cargo.lock ./

# Build the actual application (keep using the minimal workspace)
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