FROM rust:latest

WORKDIR /usr/src/website

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD cargo run --release

