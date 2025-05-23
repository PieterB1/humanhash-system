FROM rust:1.80.1 AS builder
WORKDIR /usr/src/system
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/system/target/release/humanhash-system /usr/local/bin/system
EXPOSE 3000
CMD ["system"]
