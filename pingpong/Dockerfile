FROM rustlang/rust:nightly-buster@sha256:ca626ba169b1b720fcdad3c9ebdba1a97ed4cc0cc2ad53f70c456ca4ccd96537 as builder
WORKDIR /usr/src/pingpong
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
WORKDIR /usr/app/pingpong
COPY --from=builder /usr/src/pingpong/target/release/pingpong /usr/app/pingpong/
ENV ROCKET_PORT 8000
ENV ROCKET_ENV prod
CMD ["./pingpong"]
