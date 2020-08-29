FROM rustlang/rust:nightly-buster@sha256:ca626ba169b1b720fcdad3c9ebdba1a97ed4cc0cc2ad53f70c456ca4ccd96537 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/src/backend /usr/local/bin/backend
ENV ROCKET_PORT 8000
ENV ROCKET_ENV prod
ENTRYPOINT ["/usr/local/bin/backend/target/release/backend"]
CMD [""]