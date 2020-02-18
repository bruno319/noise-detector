FROM rust:1.41 as builder

WORKDIR /application
COPY . .
RUN apt-get update && \
    apt-get install -y libasound-dev portaudio19-dev && \
    rustup default nightly && \
    cargo build


FROM ubuntu:18.04

COPY --from=builder /application/target/debug/noise-detector /opt
RUN apt-get update && \
    apt-get install -y --no-install-recommends libasound-dev portaudio19-dev alsa-base alsa-utils && \
    rm -rf /var/lib/apt/lists/*

CMD ["/opt/noise-detector"]
