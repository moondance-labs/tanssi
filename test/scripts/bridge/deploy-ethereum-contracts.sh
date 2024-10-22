#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

deploy_command() {
    local deploy_script=$1

    pushd "$contract_dir"
    forge script \
    --rpc-url $eth_endpoint_http \
    --legacy \
    --broadcast \
    -vvv \
    $deploy_script
    popd
}

echo "Deploying contracts"
deploy_command scripts/DeployLocal.sol:DeployLocal
