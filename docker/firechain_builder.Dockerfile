# Use BuildKit
# syntax=docker/dockerfile:experimental
# Use the Rust nightly image as the builder stage

FROM rust:1.61.0 AS builder


WORKDIR /firechain



COPY . .


RUN set -eux; \
    git config --global url.git@github.com:.insteadOf https://github.com/; \
    mkdir -m 0700 -p ~/.ssh; \
    ssh-keyscan github.com >> ~/.ssh/known_hosts


ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN apt-get update && apt-get install -y protobuf-compiler

RUN --mount=type=ssh cargo build --release


FROM debian:bullseye-slim


RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl git openssh-client; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*


COPY --from=builder /firechain/target/release/firechain-node /usr/local/bin/firechain-node


RUN useradd -m -u 1000 -U -s /bin/sh -d /firechain firechain


RUN set -eux; \
    mkdir -p /data /firechain/.local/share/firechain; \
    chown -R firechain:firechain /data; \
    ln -s /data /firechain/.local/share/firechain


EXPOSE 30333 9933 9944 9946


USER firechain


CMD ["firechain-node"]



