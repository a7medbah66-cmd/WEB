# Use the official Rust image as a builder
FROM rust:1.70-slim as builder

WORKDIR /app

# 1. Copy the workspace configuration first
COPY Cargo.toml Cargo.lock ./

# 2. Copy the actual source folders 
# Ensure these folder names match your 'ls' output EXACTLY (case-sensitive)
COPY engine/ ./engine/
COPY engine_server/ ./engine_server/
# We skip browser_ui for the server build to save time/space
# COPY browser_ui/ ./browser_ui/ 

# 3. Build the headless server
RUN cargo build --release -p engine_server

# Final Stage: Use a tiny runtime image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/engine_server .

# Set the start command
CMD ["./engine_server"]