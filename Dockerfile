FROM rust:slim AS builder
MAINTAINER Laura Demkowicz-Duffy <edward70891@gmail.com>

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

WORKDIR /source
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runner

ENV RUST_LOG=info
ENV UPDATER_CONFIG_PATH=/config.yml
ENV UPDATER_STATE_PATH=/updater_state

WORKDIR /opt/steam-dockerupdater

COPY --from=builder /source/target/release/steam-docker-updater .
RUN chmod +x steam-docker-updater

VOLUME $UPDATER_STATE_PATH

ENTRYPOINT ./steam-docker-updater
