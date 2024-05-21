#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
../target/release/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --invulnerable "Collator-01" --invulnerable "Collator-02" > specs/one-node-tanssi-1000.json
