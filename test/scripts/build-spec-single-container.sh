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
    CONTAINER_BINARY="container-chain-simple-node"
else
    CONTAINER_BINARY="${2}"
fi

mkdir -p specs
$BINARY_FOLDER/$CONTAINER_BINARY build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/single-container-template-container-2000.json
$BINARY_FOLDER/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --add-container-chain specs/single-container-template-container-2000.json --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" --invulnerable "Collator1000-03" --invulnerable "Collator2000-01" --invulnerable "Collator2000-02" --raw > specs/single-container-tanssi-1000.json
