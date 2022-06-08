FROM rust:1-buster

WORKDIR /usr/src/mpesa-async-server
COPY . .
RUN cargo install --path .
CMD ["mpesa-async-server"]
