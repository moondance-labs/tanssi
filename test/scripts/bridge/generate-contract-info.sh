#!/usr/bin/env bash

set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

pushd "$test_helpers_dir" > /dev/null
pnpm generateContracts "$output_dir/contracts.json" > /dev/null
popd > /dev/null

# Output the file so that invoker can read it
cat "$output_dir/contracts.json"
