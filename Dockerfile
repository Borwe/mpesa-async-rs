FROM rust:1-buster

WORKDIR /usr/src/mpesa-async-server
COPY . .
CMD ["cargo run --release mpesa-async-server"]
