FROM rust:1-bullseye

WORKDIR /app

COPY . .

RUN cargo build --release

