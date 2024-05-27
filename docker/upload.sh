#!/bin/bash

build_docker=""
profile="default"

TAG=$(grep -E '^version\s*=' "../node/Cargo.toml" | awk -F '"' '{print $2}')
echo $TAG


# Display usage information
usage() {
  echo "Usage: $0 [-e environment] [-h|--help]"
  echo "Options:"
  echo "  -e  Specify the environment for which docker image will be uploaded"
  echo "  --build Build docker image"
  echo "  --profile AWS profile to use while uploading image (default: default)"
  echo "  -h, --help  Display this help message"
  exit 1
}

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    -e)
      environment="$2"
<<<<<<< HEAD
      if [ "$environment" != "qa" ] && [ "$environment" != "thunder" ]; then
=======
      if [ "$environment" != "qa" ] || [ "$environment" != "thunder" ]; then
>>>>>>> cdb6b8a (updated validator nodes flags, added scripts readme & added upload.sh)
        echo "Invalid Environment '$environment'. Only 'qa' and 'thunder' envrionments are allowed at the moment"
        exit 1
      fi
      shift 2
      ;;
    --build)
      build_docker=true
      shift
      ;;
    --build)
      profile="$2"
      shift 2
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

# Build Docker image if requested
if [ "$build_docker" = true ]; then
  echo "Creating Docker for 5irechain node....."
  build.sh -e "$environment"
fi

docker images | grep latest | awk '{print $1}' | grep 5irenode
if [ $? -ne 0 ]; then  
  echo "No image exists for $environment. Please run the script with --build flag" 
  exit 1
fi

echo "Pushing docker image to $environment environment"
if [ "$environment" == "qa" ]; then 
  aws ecr get-login-password --region us-west-2 --profile $profile | docker login --username AWS --password-stdin 392225661532.dkr.ecr.us-west-2.amazonaws.com
  docker tag 5irenode$environment 392225661532.dkr.ecr.us-west-2.amazonaws.com/firechain-qa:$TAG
  docker push 392225661532.dkr.ecr.us-west-2.amazonaws.com/firechain-qa:$TAG 
elif [ "$environment" == "thunder" ]; then
  docker tag 5irenode$environment 5irechain/5ire-thunder-node:$TAG
  docker push 5irechain/5ire-thunder-node:$TAG
<<<<<<< HEAD
fi
=======
fi
>>>>>>> cdb6b8a (updated validator nodes flags, added scripts readme & added upload.sh)
