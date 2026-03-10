# Use the official Rust image as the base image
FROM rust:1.70-slim as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY engine/ ./engine/
COPY browser_ui/ ./browser_ui/
COPY engine_server/ ./engine_server/

# Build the engine_server binary
RUN cargo build --release -p engine_server

# Create a new stage for the runtime image
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/engine_server /usr/local/bin/engine_server

# Expose the port that the app runs on
EXPOSE 3000

# Set the default command to run the binary
CMD ["engine_server"]