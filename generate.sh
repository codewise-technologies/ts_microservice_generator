#!/bin/bash

# Default values
NAME="MyProject"
VERSION="1.0"
AUTHOR="John Doe"
ABOUT="This is a TypeScript microservice example."

# Parse command-line arguments
while getopts ":n:v:a:b:" opt; do
  case ${opt} in
    n )
      NAME=$OPTARG
      ;;
    v )
      VERSION=$OPTARG
      ;;
    a )
      AUTHOR=$OPTARG
      ;;
    b )
      ABOUT=$OPTARG
      ;;
    \? )
      echo "Usage: cmd [-n name] [-v version] [-a author] [-b about]"
      exit 1
      ;;
  esac
done
shift $((OPTIND -1))

# Run the generator with the specified arguments
# Assuming the binary will be installed globally and is in the PATH
ts_microservice_generator -n "$NAME" -v "$VERSION" -a "$AUTHOR" -b "$ABOUT"
