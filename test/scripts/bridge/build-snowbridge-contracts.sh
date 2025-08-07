#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building snowbridge contracts"

pushd $contract_dir
#    TODO: Add custom ci-fast profile on the fly
export FOUNDRY_PROFILE=ci
forge build
popd
