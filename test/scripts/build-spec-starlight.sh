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
$BINARY_FOLDER/container-chain-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --raw > specs/single-container-template-container-2001.json
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2002 --raw > specs/single-container-template-container-2002.json

#$BINARY_FOLDER/tanssi-relay build-spec --chain starlight-local --add-container-chain specs/single-dancebox-container-1000.json --add-container-chain specs/single-container-template-container-2000.json --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Alice" --invulnerable "Bob" > specs/tanssi-relay.json
$BINARY_FOLDER/tanssi-relay build-spec --chain starlight-local --add-container-chain specs/single-container-template-container-2000.json --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" --invulnerable "Collator1000-03" --invulnerable "Collator1000-04" --invulnerable "Collator2000-01" --invulnerable "Collator2000-02" > specs/tanssi-relay.json
## Here we need to modify the chain-spec. Zombienet is not able to do this
