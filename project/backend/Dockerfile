FROM rust:1.61.0-buster as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
WORKDIR /usr/app/backend
COPY --from=builder /usr/src/backend/target/release/backend .
COPY --from=builder /usr/src/backend/public/ ./public/
COPY --from=builder /usr/src/backend/Rocket.toml ./Rocket.toml
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
RUN apt-get update && apt-get install libpq5 -y
CMD ["./backend"]