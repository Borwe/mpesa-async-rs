FROM rust:1-buster

WORKDIR /usr/src/mpesa-async
COPY . .
RUN cd ./mpesa-async-server/ && cargo install --path ./
CMD ["mpesa-async-server"]
