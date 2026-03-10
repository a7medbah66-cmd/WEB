# Stage 1: Builder
FROM rust:1.70-slim as builder

# Install build dependencies for Rust
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy EVERYTHING in the current directory to the container
COPY . .

# Build only the headless engine server
RUN cargo build --release -p engine_server

# Stage 2: Runtime
FROM debian:bullseye-slim
WORKDIR /app

# Install SSL certs so your engine can hit HTTPS websites
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/engine_server .

# Railway uses PORT 8080 by default
EXPOSE 8080

CMD ["./engine_server"]