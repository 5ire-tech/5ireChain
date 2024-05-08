FROM rust:1.61.0 AS builder

WORKDIR /5ire

COPY . /5ire

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN apt-get update && apt-get install -y protobuf-compiler libclang-dev

RUN cargo build --release --features firechain-qa

FROM debian:bullseye-slim

WORKDIR /5ire

COPY --from=builder /5ire/target/release/firechain-node /5ire/firechain-node


COPY --from=builder /5ire/specs/5ire-qa-chain-spec-raw.json /5ire/specs/5ire-qa-chain-spec-raw.json

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl git openssh-client; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*

RUN ldd /5ire/firechain-node
RUN /5ire/firechain-node --version

EXPOSE 30333 9944

VOLUME ["5ire/data"]

ENTRYPOINT ["/5ire/firechain-node"]
