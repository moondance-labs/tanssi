#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

data_store_dir="$output_dir/relayer_data"
mkdir -p $data_store_dir
mkdir -p $logs_dir

echo "$logs_dir"/beefy-relay.log

# Requires that relayers are configured
start_relayer() {
    echo "Starting relay services"
    # Launch beefy relay
    (
        : >"$logs_dir"/beefy-relay.log
        while :; do
            echo "Starting beefy relay at $(date)"
            "${relay_bin}" run beefy \
                --config "$output_dir/beefy-relay.json" \
                --ethereum.private-key $beefy_relay_eth_key \
                >>"$logs_dir"/beefy-relay.log 2>&1 || true
            sleep 20
        done
    ) &
    echo "beefy_relay=$!" >> $artifacts_dir/daemons.pid

    # Launch beacon relay
    (
        : >"$logs_dir"/beacon-relay.log
        while :; do
            echo "Starting beacon relay at $(date)"
            "${relay_bin}" run beacon \
                --config $output_dir/beacon-relay.json \
                --substrate.private-key "//BeaconRelay" \
                >>"$logs_dir"/beacon-relay.log 2>&1 || true
            sleep 20
        done
    ) &
     echo "beacon_relay=$!" >> $artifacts_dir/daemons.pid
}

echo "start relayers only!"
trap kill_all SIGINT SIGTERM EXIT
start_relayer
wait
