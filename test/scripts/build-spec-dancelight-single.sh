#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

if [[ -z "${1}" ]]; then
    BINARY_FOLDER="../target/release"
else
    BINARY_FOLDER="${1}"
fi

mkdir -p specs
$BINARY_FOLDER/tanssi-relay build-spec --chain dancelight-local > specs/tanssi-relay.json
