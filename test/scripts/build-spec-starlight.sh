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
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/single-container-template-container-2000.json
$BINARY_FOLDER/container-chain-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --raw > specs/single-container-template-container-2001.json
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --parachain-id 2002 --raw > specs/single-container-template-container-2002.json
$BINARY_FOLDER/tanssi-relay build-spec --chain starlight-local --add-container-chain specs/single-container-template-container-2000.json --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" --invulnerable "Collator-05" --invulnerable "Collator-06" > specs/tanssi-relay-starlight.json

# Also need to build the genesis-state to be able to register the container 2002 later
$BINARY_FOLDER/container-chain-simple-node export-genesis-state --chain specs/single-container-template-container-2002.json specs/para-2002-genesis-state
