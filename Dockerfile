FROM rust:slim AS builder
MAINTAINER Laura Demkowicz-Duffy <edward70891@gmail.com>

WORKDIR /source
COPY . .
RUN cargo build --release

FROM debian:slim AS runner

ENV RUST_LOG=info

WORKDIR /opt/steam-dockerupdater

COPY --from=builder /source/target/release/steam-docker-updater .
RUN chmod +x steam-docker-updater

ENTRYPOINT steam-docker-updater
