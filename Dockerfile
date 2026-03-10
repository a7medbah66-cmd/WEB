FROM rust:1.70-slim as builder
WORKDIR /app

# Copy the workspace manifest
COPY Cargo.toml Cargo.lock* ./

# Copy ALL folders so the workspace is complete
COPY engine/ ./engine/
COPY engine_server/ ./engine_server/
COPY browser_ui/ ./browser_ui/

# Build the server
RUN cargo build --release -p engine_server

FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/engine_server .
EXPOSE 8080
CMD ["./engine_server"]