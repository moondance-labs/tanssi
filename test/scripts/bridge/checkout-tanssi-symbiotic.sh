#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building symbiotic contracts"

echo "Checkout Symbiotic contract repository"

if [ -d "$symbiotic_contracts_dir" ];
then
  pushd $symbiotic_contracts_dir
  git fetch && git checkout 695c7d1c5c0541f93587a607bc193c4e9944e985
  popd
  echo "Symbiotic contract repository seems to be already setup. Skipping git clone"
else
  git clone https://github.com/moondance-labs/tanssi-symbiotic $symbiotic_contracts_dir
  pushd $symbiotic_contracts_dir
  git fetch && git checkout 695c7d1c5c0541f93587a607bc193c4e9944e985
  popd
fi