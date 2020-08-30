FROM rustlang/rust:nightly-buster@sha256:ca626ba169b1b720fcdad3c9ebdba1a97ed4cc0cc2ad53f70c456ca4ccd96537 as builder
WORKDIR /usr/src/mainapp
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/src/mainapp /usr/local/bin/mainapp
ENTRYPOINT ["/usr/local/bin/mainapp/target/release/mainapp"]
CMD [""]