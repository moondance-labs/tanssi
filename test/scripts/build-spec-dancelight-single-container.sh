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

mkdir -p specs
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/single-container-template-container-2000.json
$BINARY_FOLDER/tanssi-relay build-spec --chain dancelight-local --add-container-chain specs/single-container-template-container-2000.json --invulnerable "Collator-01" --invulnerable "Collator-02" > specs/tanssi-relay.json
