# Builder stage
FROM rust:1.84.0-slim-bullseye AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY shop/Cargo.toml ./shop/
COPY management/Cargo.toml ./management/

RUN mkdir -p shop/src management/src && \
    echo "fn main() {}" > shop/src/main.rs && \
    echo "pub fn init() {}" > management/src/lib.rs && \
    cargo build --release && \
    rm -rf shop/src management/src

# Copy actual source code
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/shop /app/
COPY migrations /app/migrations

CMD ["./shop"] 