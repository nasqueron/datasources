#   -------------------------------------------------------------
#   Nasqueron - Docker image for Nasqueron Datasources
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

#   -------------------------------------------------------------
#   Builder phase
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

FROM debian:bookworm AS builder

RUN apt update && \
    apt install -y git curl build-essential pkg-config \
        libpq-dev libssl-dev && \
    rm -r /var/lib/apt/lists/* && \
    mkdir -p /opt/datasources && \
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

WORKDIR /opt/datasources
ADD . ./

RUN make all

#   -------------------------------------------------------------
#   Release phase
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

FROM debian:bookworm
MAINTAINER Sébastien Santoro aka Dereckson <dereckson+nasqueron-docker@espace-win.org>

RUN apt update && \
    apt install -y unzip libpq5 ca-certificates && \
    rm -r /var/lib/apt/lists/*

COPY --from=builder \
    /opt/datasources/target/release/fantoir-datasource \
    /opt/datasources/target/release/language-subtag-registry-datasource \
    /opt/datasources/target/release/rfc-datasource \
    /usr/local/bin/
