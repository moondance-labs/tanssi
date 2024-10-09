#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

# Grab Polkadot version
branch=$(egrep -o '/polkadot-sdk.*#([^\"]*)' ../Cargo.lock | head -1)
polkadot_release=$(echo $branch | sed 's/.*branch=//' | sed 's/#.*//')
mkdir -p tmp
wget -O - tmp/ethereum_client_test https://github.com/moondance-labs/polkadot-sdk/archive/$polkadot_release.tar.gz | tar -xz --strip=6 "polkadot-sdk-$polkadot_release/bridges/snowbridge/pallets/ethereum-client/tests/fixtures"
# remove for a clean move
rm -rf  tmp/ethereum_client_test
mv  -u  fixtures tmp/ethereum_client_test