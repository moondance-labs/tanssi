#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
../target/release/container-chain-simple-node build-spec --disable-default-bootnode --parachain-id 2000 --raw > specs/template-container-2000.json
../target/release/container-chain-frontier-node build-spec --disable-default-bootnode --parachain-id 2001 --raw > specs/template-container-2001.json
../target/release/container-chain-simple-node build-spec --disable-default-bootnode --parachain-id 2002 --raw > specs/template-container-2002.json
../target/release/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --add-container-chain specs/template-container-2000.json --add-container-chain specs/template-container-2001.json --invulnerable "Collator1000-01" --invulnerable "Collator1000-02" --invulnerable "Collator2002-01" --invulnerable "Collator2002-02" --invulnerable "Collator2000-01" --invulnerable "Collator2000-02" --invulnerable "Collator2001-01" --invulnerable "Collator2001-02" > specs/tanssi-1000.json
