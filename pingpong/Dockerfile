FROM rustlang/rust:nightly-buster@sha256:ea6209a005404e7098feff7af1cd5b340fbf20a420ed05000cad21c0b714d044 as builder
WORKDIR /usr/src/pingpong
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
WORKDIR /usr/app/pingpong
COPY --from=builder /usr/src/pingpong/target/release/pingpong /usr/app/pingpong/
ENV ROCKET_PORT 8000
ENV ROCKET_ENV prod
CMD ["./pingpong"]
