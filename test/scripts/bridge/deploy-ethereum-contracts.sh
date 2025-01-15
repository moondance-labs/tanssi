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
    $deploy_script
}

echo "Deploying snowbridge contracts"
pushd "$symbiotic_contracts_dir"
deploy_command script/test/DeployLocalSnowbridge.sol:DeployLocalSnowbridge
popd


echo "Deploying symbiotic contracts"
pushd "$symbiotic_contracts_dir"
deploy_command script/test/DeployTanssiEcosystemDemo.s.sol:DeployTanssiEcosystem
popd
