# Stage 1: Builder
FROM rust:1.70-slim as builder

# Set the working directory FIRST
WORKDIR /app

# Copy the manifest files
# Using a wildcard (*) for the lockfile makes it optional during COPY

# ... (inside the builder stage)
COPY engine/ ./engine/
COPY engine_server/ ./engine_server/
COPY browser_ui/ ./browser_ui/  # <--- Add this line
# ...

COPY Cargo.toml Cargo.lock* ./

# Copy only the folders needed for the server
COPY engine/ ./engine/
COPY engine_server/ ./engine_server/

# Build the headless server (this will generate the lockfile if missing)
RUN cargo build --release -p engine_server

# Stage 2: Runtime
FROM debian:bullseye-slim
WORKDIR /app

# Install SSL certificates (needed for engine/network.rs to hit HTTPS sites)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/engine_server .

# Expose the port Railway uses
EXPOSE 8080

CMD ["./engine_server"]