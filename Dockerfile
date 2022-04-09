FROM rust:latest AS builder

WORKDIR /website

COPY . /website

RUN cargo build --release


FROM debian:latest
WORKDIR /app
COPY --from=builder /website/target/release/website .

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./website"]



