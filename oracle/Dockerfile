# Build stage
   FROM rust:1.82 AS builder
   WORKDIR /usr/src/oracle
   COPY . .
   RUN cargo build --release

   # Runtime stage
   FROM debian:bullseye-slim
   WORKDIR /app
   COPY --from=builder /usr/src/oracle/target/release/humanhash-oracle .
   COPY --from=builder /usr/src/oracle/bitcoin/testnet3/wallets/testwallet/wallet.dat /app/bitcoin/testnet3/wallets/testwallet/wallet.dat
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   CMD ["./humanhash-oracle"]
   HEALTHCHECK --interval=30s --timeout=3s CMD curl --fail http://localhost:3003 || exit 1
