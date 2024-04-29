#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
../target/release/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/warp-sync-template-container-2000.json
../target/release/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --add-container-chain specs/warp-sync-template-container-2000.json --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" --invulnerable "Collator1000-03" --invulnerable "Collator2000-01" --invulnerable "Collator2000-02" > specs/warp-sync-tanssi-1000.json
