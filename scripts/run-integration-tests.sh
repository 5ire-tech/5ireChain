#!/bin/bash

: "${BUILD_BINARY:=1}"

if [[ $BUILD_BINARY == "1" ]]; then
	echo "*** Building 5ire node binary..."
	cargo build --release --features firechain-qa 1>/dev/null
	echo "*** Binary compiled"
fi



cd integration-test-suite

yarn install

yarn test

cd ..
