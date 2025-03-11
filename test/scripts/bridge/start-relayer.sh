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

    # Launch execution relay
    (
        : >$output_dir/execution-relay.log
        while :; do
            echo "Starting execution relay at $(date)"
            "${relay_bin}" run execution \
                --config $output_dir/execution-relay.json \
                --substrate.private-key "//ExecutionRelay" \
                >>"$logs_dir"/execution-relay.log 2>&1 || true
            sleep 20
        done
    ) &
    echo "execution_relay=$!" >> $artifacts_dir/daemons.pid

     # Launch substrate relay for primary channel
    (
        : >$output_dir/substrate-relay-primary.log
        while :; do
            echo "Starting substrate relay primary at $(date)"
            "${relay_bin}" run solochain \
                --config $output_dir/substrate-relay-primary.json \
                --ethereum.private-key $ethereum_key \
                >>"$logs_dir"/substrate-relay-primary.log 2>&1 || true
            sleep 20
        done
    ) &
    echo "substrate_relay_primary=$!" >> $artifacts_dir/daemons.pid
    
     # Launch substrate relay for secondary channel
    (
        : >$output_dir/substrate-relay-secondary.log
        while :; do
            echo "Starting substrate relay secondary at $(date)"
            "${relay_bin}" run solochain \
                --config $output_dir/substrate-relay-secondary.json \
                --ethereum.private-key $ethereum_key \
                >>"$logs_dir"/substrate-relay-secondary.log 2>&1 || true
            sleep 20
        done
    ) &
    echo "substrate_relay_secondary=$!" >> $artifacts_dir/daemons.pid
}

echo "start relayers only!"
trap kill_all SIGINT SIGTERM EXIT
start_relayer
wait
