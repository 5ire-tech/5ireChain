#!/usr/bin/env bash

# Display usage information
usage() {
  echo "Usage: $0 [-v|--validator] [-f|--fullnode] [-a|--archivenode] [-h|--help]"
  echo "Options:"
  echo "  -e  Specify the environment for which docker image will be created (default: qa)"
  echo "  -f, --fullndode  Run node as fullnode"
  echo "  -v, --validator  Run node as validator"
  echo "  -a, --archivenode  Run node as archivenode"
  echo "  -b, --bootnode  Bootnode IP to connect the nodes with"
  echo "  -bid, --bootnode-id  Bootnode ID to connect the nodes with"
  echo "  --build To build docker image before running (default: use existing docker image)"
  echo "  -h, --help  Display this help message"
  exit 1
}

# Parse command-line arguments
while getopts ":f-:a-:v-:b-:bid-:build:h-:" opt; do
  case $opt in
    b | - | --bootnode)
      bootnode_ip=$OPTARG
      ;;
    e)
      environment=$OPTARG
      ;;
    bid | - | --bootnode-id)
      bootnode_id=$OPTARG
      ;;
    f | - | --fullnode)
      command="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --port 30333 --rpc-port 9944 --pruning archive --name TestFullnodeQA --rpc-external --rpc-cors all --rpc-methods Unsafe --unsafe-rpc-external --rpc-max-connections 40000 --bootnodes /ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
      ;;
    v | - | --validator)
      command="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --port 30333 --rpc-port 9944 --pruning archive --name TestValidatorQA --rpc-external --rpc-cors all --rpc-methods Unsafe --unsafe-rpc-external --rpc-max-connections 40000 --validator --bootnodes /ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
      ;;
    a | - | --archivenode)
      command="--base-path /5ire/data --chain /5ire/specs/5ire-qa-chain-spec-raw.json --pruning archive --name TestArchiveQA --no-telemetry --bootnodes /ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
      ;;
    build)
      echo "Creating Docker for 5irechain $environment node....."
      build.sh -e $environment
      ;;
    h | - | --help)
      usage
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      usage
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      usage
      exit 1
      ;;
  esac
done

docker images | grep latest | awk '{print $1}' | grep 5irenode
if [ $? -ne 0 ]; then  
  echo "No image exists for $environment. Please run the script with --build flag" 
  exit 1
fi

# Runnig docker Node
echo "Starting docker conatiner for 5irechaon $environment node"

docker run --name 5ire$environment -d -p 30333:30333 -p 9944:9944 5irenode:$environment $command 

docker logs -f 5ire$environment