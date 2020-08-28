FROM rust:1.45-buster as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/src/backend /usr/local/bin/backend
ENTRYPOINT ["/usr/local/bin/backend/target/release/backend"]
CMD [""]