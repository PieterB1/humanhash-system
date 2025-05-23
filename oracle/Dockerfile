FROM rust:1.80.1 AS builder
WORKDIR /usr/src/oracle
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/oracle/target/release/humanhash-oracle /usr/local/bin/oracle
EXPOSE 3003
CMD ["oracle"]
