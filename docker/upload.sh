#!/bin/bash

TAG=$(date +'%Y-%m-%d_%H-%M-%S')
echo $TAG

# Display usage information
usage() {
  echo "Usage: $0 [-e environment] [-h|--help]"
  echo "Options:"
  echo "  -e  Specify the environment for which docker image will be uploaded"
  echo "  -h, --help  Display this help message"
  exit 1
}

# Parse command-line arguments
while getopts ":e:h-:" opt; do
  case $opt in
    e)
      ENVIRONMENT="$OPTARG"
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

docker tag 5irenode$environment <to be decided>:$TAG 