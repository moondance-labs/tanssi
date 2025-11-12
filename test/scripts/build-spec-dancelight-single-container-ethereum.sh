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
$BINARY_FOLDER/container-chain-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --raw > specs/single-container-template-container-2001.json
$BINARY_FOLDER/tanssi-relay build-spec --chain dancelight-local --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Collator-01" --invulnerable "Collator-02" > specs/tanssi-relay.json
