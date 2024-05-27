#!/usr/bin/env bash

# Initialize variables
command=""
bootnode_ip=""
bootnode_id=""
build_docker=false
environment="qa"

# Display usage ination
usage() {
  echo "Usage: $0 [-e <environmnent>] [-f|--fullnode] [-v|--validator] [-a|--archivenode] -b|--bootnode <bootnode_ip_address> -bid|--bootnode-id <bootnode_id> [--build] [-h|--help]"
  echo "NOTE: Only one of -f | --fullnode, -v | --validator, or -a | --archivenode is mandatory."
  echo "Options:"
  echo "  -e                  Specify the environment for which docker image will be used (default: qa)"
  echo "  -f, --fullnode      Run node as fullnode"
  echo "  -v, --validator     Run node as validator"
  echo "  -a, --archivenode   Run node as archivenode"
  echo "  -b, --bootnode      Bootnode IP to connect the nodes with (mandatory)"
  echo "  -bid, --bootnode-id Bootnode ID to connect the nodes with (mandatory)"
  echo "  --build             To build docker image before running (default: use existing docker image)"
  echo "  -h, --help          Display this help message"
  exit 1
}

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    -f | --fullnode)
      if [ -n "$command" ]; then
        echo "Error: Only one flag from -f | --fullndoe, -v | --validator, or -a | --archivenode is allowed."
        usage
      fi
      command="fullnode"
      shift
      ;;
    -v | --validator)
      if [ -n "$command" ]; then
        echo "Error: Only one flag from -f | --fullndoe, -v | --validator, or -a | --archivenode is allowed."
        usage
      fi
      command="validator"
      shift
      ;;
    -a | --archivenode)
      if [ -n "$command" ]; then
        echo "Error: Only one flag from -f | --fullndoe, -v | --validator, or -a | --archivenode is allowed."
        usage
      fi
      command="archivenode"
      shift
      ;;
    -e)
      environment="$2"
      if [ "$environment" != "qa" ] && [ "$environment" != "thunder" ]; then
        echo "Invalid Environment '$environment'. Only 'qa' and 'thunder' envrionments are allowed at the moment"
        exit 1
      fi
      shift 2
      ;;
    -b | --bootnode)
      bootnode_ip="$2"
      shift 2
      ;;
    -bid | --bootnode-id)
      bootnode_id="$2"
      shift 2
      ;;
    --build)
      build_docker=true
      shift
      ;;
    -h | --help)
      usage
      ;;
    *)
      echo "Error: Invalid option: $1"
      usage
      ;;
  esac
done

# Check if bootnode options are provided
if [ -z "$bootnode_ip" ] || [ -z "$bootnode_id" ] || [ -z "$command" ]; then
  usage
fi

# Build Docker image if requested
if [ "$build_docker" = true ]; then
  echo "Creating Docker for 5irechain node....."
  build.sh -e "$environment"
fi

# Check if Docker image exists
if ! docker images | grep $environment | awk '{print $1}' | grep 5irenode; then  
  echo "Error: No image exists for 5irenode:$environment. Please run the script with --build flag." 
  exit 1
fi

# Run Docker Node
echo "Starting Docker container for 5irechain node"

case $command in
  fullnode)
    docker run --name 5ire$environment -d -p 30333:30333 -p 9944:9944 5irenode:$environment \
      --base-path /5ire/data --chain /5ire/specs/5ire-$environment-specRaw.json --port 30333 \
      --rpc-port 9944 --pruning archive --name TestFullnode --rpc-external --rpc-cors all \
      --rpc-methods Unsafe --unsafe-rpc-external --rpc-max-connections 40000 \
      --bootnodes "/ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
    ;;
  validator)
    docker run --name 5ire$environment -d -p 30333:30333 -p 9944:9944 5irenode:$environment \
      --base-path /5ire/data --chain /5ire/specs/5ire-$environment-specRaw.json --port 30333 \
      --rpc-port 9944 --name TestValidator --rpc-external --rpc-cors all \
      --rpc-methods Unsafe --unsafe-rpc-external --validator \
      --bootnodes "/ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
    ;;
  archivenode)
    docker run --name 5ire$environment -d -p 30333:30333 -p 9944:9944 5irenode:$environment \
      --base-path /5ire/data --chain /5ire/specs/5ire-$environment-specRaw.json --pruning archive \
      --name TestArchive --no-telemetry --bootnodes "/ip4/$bootnode_ip/tcp/30333/p2p/$bootnode_id"
    ;;
esac

docker logs -f 5ire$environment
