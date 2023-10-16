#!/usr/bin/env bash
set -e

# Set SSH_AUTH_SOCK to the path of your SSH agent socket
export SSH_AUTH_SOCK="location to your SSH_auth_sock"

pushd .

# Change to the project root and support calls from symlinks
cd $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")")

# Find the current version from Cargo.toml
VERSION=$(grep "^version" ./node/Cargo.toml | egrep -o "([0-9\.]+)")
GITUSER=5ire-tech
GITREPO=5ire-evm-base

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
sudo DOCKER_BUILDKIT=1 docker build --ssh default=$SSH_AUTH_SOCK -t ${GITUSER}/${GITREPO}:latest -f docker/firechain_builder.Dockerfile .

docker tag ${GITUSER}/${GITREPO}:latest ${GITUSER}/${GITREPO}:v${VERSION}

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

popd
