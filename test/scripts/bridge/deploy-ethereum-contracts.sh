#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

deploy_command() {
    local deploy_script=$1
    $scripts_root_dir/add-ci-fast-foundry-profile.sh
    OWNER_PRIVATE_KEY=$ethereum_key FOUNDRY_PROFILE=ci-fast forge script \
    --rpc-url $eth_endpoint_http \
    --sender 0x$ethereum_address \
    --private-key $ethereum_key \
    --legacy \
    --broadcast \
    -vvv \
    --slow \
    --skip-simulation \
    --optimizer-runs 1 \
    $deploy_script
}

echo "Deploying snowbridge contracts"
pushd "$contract_dir"
deploy_command scripts/DeployLocal.sol:DeployLocal
popd

echo "Deploying symbiotic contracts"
pushd "$symbiotic_contracts_dir"
deploy_command demos/DeployTanssiEcosystemDemo.s.sol
popd
