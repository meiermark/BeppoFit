# Builder stage
FROM rust:1.83-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/beppo-fit-backend /usr/local/bin/beppo-fit-backend

# Set environment variable for port (optional, but good practice)
ENV PORT=8080
EXPOSE 8080

CMD ["beppo-fit-backend"]
