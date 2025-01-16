#!/bin/bash

# Exit on any error
set -e

bridge_scripts=$(realpath ./scripts/bridge)
source $bridge_scripts/set-env.sh

check_tool

# Install sszgen
GOBIN=$output_bin_dir go install github.com/ferranbt/fastssz/sszgen@v0.1.4

$bridge_scripts/build-ethereum-node.sh
$bridge_scripts/checkout-tanssi-symbiotic.sh
$bridge_scripts/build-relayer.sh
$bridge_scripts/build-symbiotic-contracts.sh

pushd $ts_scripts_dir
pnpm install
popd
