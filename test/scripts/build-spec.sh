#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the base dir of the repo
cd $(dirname $0)/../..

mkdir -p specs
./target/release/tanssi-node run-container-command simple build-spec --disable-default-bootnode --parachain-id 2000 --seeds "Collator2000-01,Collator2000-02" --raw > specs/template-container-2000.json
./target/release/tanssi-node run-container-command ethereum build-spec --disable-default-bootnode --parachain-id 2001 --seeds "Collator2001-01,Collator2001-02" --raw > specs/template-container-2001.json
./target/release/tanssi-node run-container-command simple build-spec --disable-default-bootnode --parachain-id 2002 --seeds "Collator2002-01,Collator2002-02" --raw > specs/template-container-2002.json
./target/release/tanssi-node build-spec --parachain-id 1000 --add-container-chain specs/template-container-2000.json --add-container-chain specs/template-container-2001.json > specs/tanssi-1000.json
