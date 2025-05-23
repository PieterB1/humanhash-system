FROM rust:1.80.1 AS builder
WORKDIR /usr/src/biometric
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/biometric/target/release/humanhash-biometric /usr/local/bin/biometric
EXPOSE 8000
CMD ["biometric"]
