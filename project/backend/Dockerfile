FROM rustlang/rust:nightly-buster@sha256:ca626ba169b1b720fcdad3c9ebdba1a97ed4cc0cc2ad53f70c456ca4ccd96537 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
WORKDIR /usr/app/backend
COPY --from=builder /usr/src/backend/target/release/backend .
COPY --from=builder /usr/src/backend/static/ ./static/
COPY --from=builder /usr/src/backend/Rocket.toml ./Rocket.toml
RUN apt-get update && apt-get install -y openssl
ENV ROCKET_PORT 8000
ENV ROCKET_ENV prod
CMD ["./backend"]