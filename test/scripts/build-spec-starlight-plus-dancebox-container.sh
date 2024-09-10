#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

if [[ -z "${1}" || ${1} == "undefined" ]]; then
    BINARY_FOLDER="../target/release"
else
    BINARY_FOLDER="${1}"
fi

if [[ -z "${2}" || ${2} == "undefined" ]]; then
    CONTAINER_BINARY="tanssi-node"
else
    CONTAINER_BINARY="${2}"
fi

mkdir -p specs
$BINARY_FOLDER/$CONTAINER_BINARY build-spec --chain dancebox-local --parachain-id 1000 --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" --raw > specs/single-dancebox-container-1000.json
$BINARY_FOLDER/tanssi-relay build-spec --chain starlight-local --add-container-chain specs/single-dancebox-container-1000.json --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" > specs/tanssi-relay.json

## Here we need to modify the chain-spec. Zombienet is not able to do this