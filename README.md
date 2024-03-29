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
```bash
docker pull 5irechain/5ire-thunder-node:0.12
```


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
./target/release/firechain-node --chain qa-dev --alice
```

## Contributions & Code of Conduct

If you wish to contribute, kindly fork the repository, implement your changes, and then submit a pull request. We welcome all pull requests enthusiastically.

In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

### Getting started

1. **Fork and branch**: Fork the `master` branch into your own GitHub account. Create a feature branch for your changes.
2. **Make changes**: Implement your changes or additions in your feature branch.
3. **Contribution quality**: Ensure that your contributions are:
    - **Atomic**: Small, self-contained, logical updates are preferred.
    - **Well documented**: Use clear commit messages. Explain your changes in the pull request description.
    - **Tested**: Verify your changes do not break existing functionality.

### Creating a pull request

1. **Pull request**: Once your changes are complete, create a pull request against the master branch of 5ireChain Repository.
2. **Review process**: Your pull request will be reviewed by the maintainers. They may request changes or clarifications.
3. **Responsibility**: Contributors are expected to maintain their contributions over time and update them as necessary to ensure continued accuracy and relevance.

### Best practices

- **Stay informed**: Keep up-to-date with the latest developments in 5ireChain.
- **Engage with the community**: Participate in discussions and provide feedback on other contributions.
- **Stay consistent**: Ensure your contributions are coherent with the rest of the documentation and do not overlap or contradict existing content.

### Contact and support

- For docs issues (technical or language) open an issue here.
- For technical issues with the software, either raise an issue here and we will follow up, or email us at [support@5ire.org](mailto:support@5ire.org)


