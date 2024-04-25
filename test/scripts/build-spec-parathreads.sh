#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
../target/release/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/parathreads-template-container-2000.json
../target/release/container-chain-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --raw > specs/parathreads-template-container-2001.json
# TODO: add parathreads to genesis when supported by pallet_registrar
../target/release/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" > specs/parathreads-tanssi-1000.json
