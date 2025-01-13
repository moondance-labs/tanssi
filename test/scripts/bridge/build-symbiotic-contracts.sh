#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building symbiotic contracts"

echo "Checkout Symbiotic contract repository"

if [ -d "$symbiotic_contracts_dir" ];
then
  echo "Symbiotic contract repository seems to be already setup. Skipping git fetch"
else
  git clone https://github.com/moondance-labs/tanssi-symbiotic $symbiotic_contracts_dir
  pushd $symbiotic_contracts_dir
  git fetch && git checkout 142d523cd5c62f2f2b62541b6cb68de75a2404e3
  popd
fi

pushd $symbiotic_contracts_dir
forge build
popd
