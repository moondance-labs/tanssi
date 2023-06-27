#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
./tmp/container-chain-template-simple-node build-spec --disable-default-bootnode --parachain-id 2000 --raw > specs/template-container-2000.json
./tmp/container-chain-template-frontier-node build-spec --disable-default-bootnode --parachain-id 2001 --raw > specs/template-container-2001.json
./tmp/container-chain-template-simple-node build-spec --disable-default-bootnode --parachain-id 2002 --raw > specs/template-container-2002.json
./tmp/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --add-container-chain specs/template-container-2000.json --add-container-chain specs/template-container-2001.json > specs/tanssi-1000.json
