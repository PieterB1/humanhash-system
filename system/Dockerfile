# Build stage
   FROM rust:1.82 AS builder
   WORKDIR /usr/src/system
   COPY . .
   RUN cargo build --release

   # Runtime stage
   FROM debian:bullseye-slim
   WORKDIR /app
   COPY --from=builder /usr/src/system/target/release/humanhash-system .
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   CMD ["./humanhash-system"]
   HEALTHCHECK --interval=30s --timeout=3s CMD curl --fail http://localhost:8081 || exit 1
