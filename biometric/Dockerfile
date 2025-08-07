# Build stage
   FROM rust:1.82 AS builder
   WORKDIR /usr/src/biometric
   COPY . .
   RUN cargo build --release

   # Runtime stage
   FROM debian:bullseye-slim
   WORKDIR /app
   COPY --from=builder /usr/src/biometric/target/release/humanhash-biometric .
   COPY --from=builder /usr/src/biometric/bitcoin/testnet3/wallets/testwallet/wallet.dat /app/bitcoin/testnet3/wallets/testwallet/wallet.dat
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   CMD ["./humanhash-biometric"]
   HEALTHCHECK --interval=30s --timeout=3s CMD curl --fail http://localhost:8080 || exit 1
