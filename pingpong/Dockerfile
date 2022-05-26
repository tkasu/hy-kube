FROM rust:1.61.0-buster as builder
WORKDIR /usr/src/pingpong
COPY . .
RUN cargo install --locked --path .

FROM debian:buster
WORKDIR /usr/app/pingpong
RUN apt-get update && apt-get install libpq5 -y
COPY --from=builder /usr/src/pingpong/target/release/pingpong /usr/app/pingpong/
CMD ["./pingpong"]
