#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

generate_beefy_checkpoint()
{
    pushd "$test_helpers_dir"
    pnpm up "@polkadot/*@14.0.1"
    contract_dir="$artifacts_dir/tanssi-symbiotic" pnpm generateBeefyCheckpoint
    popd
}

echo "generate beefy checkpoint!"
generate_beefy_checkpoint
wait
