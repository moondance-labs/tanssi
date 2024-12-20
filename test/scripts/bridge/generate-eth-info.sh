#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

pushd "$ts_scripts_dir" > /dev/null
contract_dir="$artifacts_dir/relayer/contracts" deploy_script="DeployLocal.sol" pnpm generateContracts "$output_dir/snowbridge_contracts.json" > /dev/null
contract_dir="$artifacts_dir/tanssi-symbiotic" deploy_script="DeployTanssiEcosystemDemo.s.sol" pnpm generateContracts "$output_dir/symbiotic_contracts.json" > /dev/null
popd > /dev/null

# Output the file so that invoker can read it
snowbridge_info=$(cat "$output_dir/snowbridge_contracts.json")
symbiotic_info=$(cat "$output_dir/symbiotic_contracts.json")

echo "{ \"snowbridge_info\": $snowbridge_info, \"symbiotic_info\": $symbiotic_info, \"ethereum_key\": \"$ethereum_key\" }"
