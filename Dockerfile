FROM rust:1.82-slim-bookworm as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/next_learn_api_loco_v0101-cli /usr/app/next_learn_api_loco_v0101-cli

EXPOSE 8080

ENTRYPOINT ["/usr/app/next_learn_api_loco_v0101-cli"]
CMD [ "start" ]
