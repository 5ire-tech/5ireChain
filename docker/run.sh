#!/usr/bin/env bash

args=$@

# handle when arguments not provided. run arguments provided to script.
if [ "$args" = "" ] ; then
    printf "Note: Please try providing an argument to the script.\n\n"
    exit 1
else
    printf "*** Running Firenode Docker container with provided arguments: $args\n\n"
    docker run --rm -it 5ire-tech/5ire-evm-base $args
fi
