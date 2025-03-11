#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

deploy_command() {
    local deploy_script=$1

    OWNER_PRIVATE_KEY=$ethereum_key forge script \
    --rpc-url $eth_endpoint_http \
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
deploy_command demos/DeployTanssiEcosystemDemo.s.sol:DeployTanssiEcosystem
popd
