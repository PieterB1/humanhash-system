FROM rust:1.80.1 AS builder
WORKDIR /usr/src/popchain
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/popchain/target/release/humanhash-popchain /usr/local/bin/popchain
EXPOSE 3002
CMD ["popchain"]
