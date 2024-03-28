#!/bin/bash

: "${CHAIN:=qa-local}"
: "${BUILD_BINARY:=1}"
: "${SPEC_PATH:=specs/}"

FULL_PATH="$SPEC_PATH$CHAIN.json"

if [[ $BUILD_BINARY == "1" ]]; then
	echo "*** Building 5ire node binary..."
	cargo build --release --features firechain-qa 1>/dev/null
	echo "*** Binary compiled"
fi

echo "*** Building chainspec..."
./target/release/firechain-node build-spec --disable-default-bootnode --raw --chain $CHAIN > $FULL_PATH
echo "*** Chainspec built and output to file"

echo "*** Purging previous state..."
./target/release/firechain-node purge-chain -y --base-path /tmp/bob --chain="$FULL_PATH" >/dev/null 2>&1
./target/release/firechain-node purge-chain -y --base-path /tmp/alice --chain="$FULL_PATH" >/dev/null 2>&1
echo "*** Previous chainstate purged"

echo "*** Starting localnet nodes..."
alice_start=(
	./target/release/firechain-node
	--base-path /tmp/alice
	--chain="$FULL_PATH"
	--alice
	--port 30334
	--rpc-port 9934
	--validator
	--rpc-cors=all
	--execution native
)

bob_start=(
	./target/release/firechain-node
	--base-path /tmp/bob
	--chain="$FULL_PATH"
	--bob
	--port 30335
	--rpc-port 9935
	--validator
	--execution native
	--bootnodes "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWBBUaVWE5SYj3UvnoXojfS8fvPorw5biRDaDQV7XXwCXm"
)

(trap 'kill 0' SIGINT; ("${alice_start[@]}" 2>&1) & ("${bob_start[@]}" 2>&1))