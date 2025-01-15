#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building symbiotic contracts"

pushd $symbiotic_contracts_dir
forge build
popd
