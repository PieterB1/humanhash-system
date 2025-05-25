FROM rust:1.82
WORKDIR /usr/src/biometric
COPY . .
RUN cargo build --release
CMD ["./target/release/humanhash-biometric"]

