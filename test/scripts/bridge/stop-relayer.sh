#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

source $artifacts_dir/daemons.pid

echo "Killing beefy_relay at $beefy_relay"
kill -KILL $beefy_relay

echo "Killing beacon_relay at $beacon_relay"
kill -KILL $beacon_relay

echo "Killing execution_relay at $execution_relay"
kill -KILL $execution_relay

echo "Killing substrate_relay_primary at $substrate_relay_primary"
kill -KILL $substrate_relay_primary

echo "Killing substrate_relay_secondary at $substrate_relay_secondary"
kill -KILL $substrate_relay_secondary

echo "Killing execution_relay_asset_hub at $execution_relay_asset_hub"
kill -KILL $execution_relay_asset_hub

echo "Killing substrate_relay_asset_hub at $substrate_relay_asset_hub"
kill -KILL $substrate_relay_asset_hub

# rm $artifacts_dir/daemons.pid
# echo "geth=$geth"
# echo "lodestar=$lodestar"

echo "Killed All relayers"
