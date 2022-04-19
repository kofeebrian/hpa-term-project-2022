FROM rust:1.60 AS builder

WORKDIR /myapp
COPY . .

RUN cargo build -j8 --release

# FROM debian:buster-slim
FROM debian:buster-slim 

COPY --from=builder /myapp/target/release/prog /usr/local/bin/prog

WORKDIR /usr/local/bin