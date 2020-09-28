FROM rust:slim AS builder
MAINTAINER Laura Demkowicz-Duffy <edward70891@gmail.com>

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

WORKDIR /source
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runner

RUN apt-get update
RUN apt-get install -y libssl-dev

ENV RUST_LOG=info
ENV UPDATER_CONFIG_PATH=/config.yml
ENV UPDATER_STATE_PATH=/updater_state

WORKDIR /opt/heat-exchanger

COPY --from=builder /source/target/release/heat-exchanger .
RUN chmod +x heat-exchanger

VOLUME $UPDATER_STATE_PATH

ENTRYPOINT ./heat-exchanger
