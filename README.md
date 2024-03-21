<p align="center">
    <img src="./docs/media/5ire-logo.png">
</p>

<h1><code>5ireChain Node</code></h1>

5ireChain is an EVM-compatible sustainability-oriented smart contract platform that focuses on developing a sustainable and for-benefit ecosystem based on the United Nations Sustainable Development Goals (UN SDGs) .To enable this, our goal is to build a computing platform that promotes and advocates these activities to achieve the goals.

5ire is a layer-1 blockchain ecosystem designed with economic and environmental sustainability at its core and is one of India's fastest-growing unicorns, enabled by a community of people worldwide.

This repository hosts the Rust implementation of the 5ireChain node, built upon the Polkadot SDK

# Roadmap
This is our roadmap

[RoadMap](docs/README.md)

# Running 5ireChain node

## Rust Setup

Prior to starting a 5ireChain node, you must set up your development environment with the necessary compiler and tools corresponding to your operating system.

https://docs.substrate.io/install/

## Connect to 5ireChain Thunder testnet

### Pull Docker Image

docker pull 5irechain/5ire-thunder-node:0.12

### Run docker image

```bash
docker run  -p 30333:30333  -p 9933:9933 -p 9944:9944 5irechain/5ire-thunder-node:0.12  --port 30333 --no-telemetry --name 5ire-thunder-archive --base-path /5ire/data --keystore-path /5ire/data   --node-key-file /5ire/secrets/node.key --chain /5ire/thunder-chain-spec.json --bootnodes /ip4/13.215.176.156/tcp/30333/ws/p2p/12D3KooWSCPiw5WquLQ1rZCbVUU8U95tgGU55EEuRZryxVJZyB7a --pruning archive --ws-external --rpc-external --rpc-cors all
```

## Connect to 5ireChain Local Network

### Build

```bash
cargo build --release --features firechain-qa 
```

### Run Alice node as A Validator

```bash
./target/release/firechain-node \
--base-path /tmp/alice \
--chain qa-local \
--alice \
--port 30333 \
--rpc-port 9944 \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--validator
```

### Run Bob node as A Validator

```bash
./target/release/firechain-node \
--base-path /tmp/bob \
--chain qa-local \
--bob \
--port 30334 \
--rpc-port 9945 \
--validator \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

## Connect to 5ireChain Dev

### Build

```bash
cargo build --release --features firechain-qa 
```

### Run a single dev node

```bash
./target/release/firechain-node --chain qa-dev --alice --tmp
```

## Contributions & Code of Conduct

If you wish to contribute, kindly fork the repository, implement your changes, and then submit a pull request. We welcome all pull requests enthusiastically.

In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).




