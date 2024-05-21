#!/bin/bash

# Default environment
ENVIRONMENT="qa"

# Display usage information
usage() {
  echo "Usage: $0 [-e environment] [-h|--help]"
  echo "Options:"
  echo "  -e  Specify the environment for which docker image will be created (default: qa)"
  echo "  -h, --help  Display this help message"
  exit 1
}

# Parse command-line arguments
while getopts ":e:h-:" opt; do
  case $opt in
    e)
      ENVIRONMENT="$OPTARG"
      if [ "$ENVIORNMENT" != "qa" ] || [ "$ENVIORNMENT" != "thunder" ]; then
        echo "Invalid Enviornment '$ENVIRONMENT'. Only 'qa' and 'thunder' envrionments are allowed at the moment"
        exit 1
      fi
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

# Shift the options so that $1 now refers to the first non-option argument
shift $((OPTIND - 1))

# Display the selected environment
echo "creating docker image for $ENVIRONMENT environment"

docker build -t 5irenode:$ENVIRONMENT -f firechain_builder.Dockerfile ../ --build-arg environment=$ENVIRONMENT
