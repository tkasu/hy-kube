FROM rust:1.45-buster as builder
WORKDIR /usr/src/mainapp
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/src/mainapp /usr/local/bin/mainapp
ENTRYPOINT ["/usr/local/bin/mainapp/target/release/mainapp"]
CMD [""]