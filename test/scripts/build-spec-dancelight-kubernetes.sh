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
# use dns for bootnodes in kubernetes
# dns works, I verified that, but still the collators are not able to connect to the full nodes
$BINARY_FOLDER/container-chain-simple-node export-chain-spec --add-bootnode "/dns4/fullnode-2000/tcp/30333/ws/p2p/12D3KooWLPFyxiN8gX9ZDb6zESRR77v9qi7ZAEhvPKKRWKgEzZf8" --parachain-id 2000 --raw > specs/single-container-template-container-2000.json
$BINARY_FOLDER/container-chain-frontier-node export-chain-spec --add-bootnode "/dns4/fullnode-2001/tcp/30333/ws/p2p/12D3KooWH9KKBLZMgJgXEQ7CTZj2PtpYZ6pW7NMvFqvjYPZJ6Aui" --parachain-id 2001 --raw > specs/single-container-template-container-2001.json
$BINARY_FOLDER/container-chain-simple-node export-chain-spec --parachain-id 2002 --raw > specs/single-container-template-container-2002.json
$BINARY_FOLDER/tanssi-relay \
  export-chain-spec \
  --chain dancelight-local \
  --add-container-chain "specs/single-container-template-container-2000.json" \
  --add-container-chain "specs/single-container-template-container-2001.json" \
  --authority "Alice" \
  --authority "Bob" \
  --authority "Charlie" \
  --authority "Dave" \
  --invulnerable "Collator-01" \
  --invulnerable "Collator-02" \
  --invulnerable "Collator-03" \
  --invulnerable "Collator-04" \
  --invulnerable "Collator-05" \
  --invulnerable "Collator-06" \
  > specs/tanssi-relay.json

# Also need to build the genesis-state to be able to register the container 2002 later
$BINARY_FOLDER/container-chain-simple-node export-genesis-state --chain specs/single-container-template-container-2002.json specs/para-2002-genesis-state
