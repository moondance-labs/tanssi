#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building snowbridge contracts"

pushd $contract_dir
$scripts_root_dir/add-ci-fast-foundry-profile.sh
export FOUNDRY_PROFILE=ci-fast
forge build
popd
