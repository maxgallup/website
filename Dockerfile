FROM rust:latest

WORKDIR /website

COPY . /website

RUN cargo build --release

EXPOSE 8000

CMD cargo run --release
