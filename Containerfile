# Build stage
FROM docker.io/rust:latest as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build application
RUN cargo build --release

# Expose API port
EXPOSE 3000

# Default to API mode
CMD ["target/release/taskprinter", "--api", "--api-port", "3000"]
