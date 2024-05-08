#!/usr/bin/env bash

node_type=$1
bootnode_ip=$2
bootnode_id=$3

read -p "Do you want to build new image or use the old one? (new/old): " upload_choice

if [ "$upload_choice" == "new" ]; then
    # Function to build Docker image
    docker build -t 5ireqa -f firechain_builder.Dockerfile ../
else
    echo "Using Old "
fi

validator="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --port 30333 --rpc-port 9944 --pruning archive --name TestValidatorQA --rpc-external --rpc-cors all --rpc-methods Unsafe --unsafe-rpc-external --rpc-max-connections 40000 --validator"
fullnode="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --port 30333 --rpc-port 9944 --pruning archive --name TestFullnodeQA --rpc-external --rpc-cors all --rpc-methods Unsafe --unsafe-rpc-external --rpc-max-connections 40000 --bootnodes /ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
archive="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --pruning archive --name TestArchiveQA --no-telemetry --bootnodes /ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"

case $node_type in
    "validator")
        echo "Runnig 5ire QA valitor Node in docker container"
        docker run -rm -it -p 30333:30333 -p 9944:9944 5ireqa $validator 
        ;;
    "fullnode")
        echo "Runnig 5ire QA fullnode Node in docker container"
        docker run -rm -it -p 30333:30333 -p 9944:9944 5ireqa $fullnode 
        ;;
    "archive")
        echo "Runnig 5ire QA archive Node in docker container"
        docker run -rm -it -d 5ireqa $archive 
        ;;
    *)
        echo "Invalid Node type"
        ;;
esac
