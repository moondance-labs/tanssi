#!/bin/bash

# Exit on any error
set -eu

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh


data_store_dir="$output_dir/relayer_data"
mkdir -p $data_store_dir

config_relayer() {
    # Configure beefy relay
    jq \
        --arg k1 "$(snowbridge_address_for BeefyClient)" \
        --arg k2 "$(snowbridge_address_for GatewayProxy)" \
        --arg eth_endpoint_ws $eth_endpoint_ws \
        --arg eth_gas_limit $eth_gas_limit \
        --arg relay_chain_endpoint $RELAYCHAIN_ENDPOINT \
        '
      .sink.contracts.BeefyClient = $k1
    | .sink.contracts.Gateway = $k2
    | .sink.ethereum.endpoint = $eth_endpoint_ws
    | .sink.ethereum."gas-limit" = $eth_gas_limit
    | .source.polkadot.endpoint = $relay_chain_endpoint
    ' \
        $assets_dir/beefy-relay.json > $output_dir/beefy-relay.json

    # Configure beacon relay
    local deneb_forked_epoch=132608
    deneb_forked_epoch=0
    jq \
        --arg beacon_endpoint_http $beacon_endpoint_http \
        --argjson deneb_forked_epoch $deneb_forked_epoch \
        --arg relay_chain_endpoint $RELAYCHAIN_ENDPOINT \
        --arg data_store_dir $data_store_dir \
        '
      .source.beacon.endpoint = $beacon_endpoint_http
    | .source.beacon.spec.denebForkedEpoch = $deneb_forked_epoch
    | .sink.parachain.endpoint = $relay_chain_endpoint
    | .source.beacon.datastore.location = $data_store_dir
    ' \
        $assets_dir/beacon-relay.json >$output_dir/beacon-relay.json


    # Configure execution relay for starlight
    jq \
        --arg eth_endpoint_ws $eth_endpoint_ws \
        --arg k1 "$(snowbridge_address_for GatewayProxy)" \
        --arg relay_chain_endpoint $RELAYCHAIN_ENDPOINT \
        --arg channelID $PRIMARY_GOVERNANCE_CHANNEL_ID \
        --arg data_store_dir $data_store_dir \
        '
      .source.ethereum.endpoint = $eth_endpoint_ws
    | .source.contracts.Gateway = $k1
    | .source."channel-id" = $channelID
    | .source.beacon.datastore.location = $data_store_dir
    | .sink.parachain.endpoint = $relay_chain_endpoint
    | .schedule.id = 0
    ' \
        $assets_dir/execution-relay.json >$output_dir/execution-relay.json


    # Configure substrate relay for ethereum
    jq \
        --arg k1 "$(snowbridge_address_for GatewayProxy)" \
        --arg k2 "$(snowbridge_address_for BeefyClient)" \
        --arg eth_endpoint_ws $eth_endpoint_ws \
        --arg eth_writer_endpoint $eth_writer_endpoint \
        --arg channelID $PRIMARY_GOVERNANCE_CHANNEL_ID \
        --arg relay_chain_endpoint $RELAYCHAIN_ENDPOINT \
        '
      .source.ethereum.endpoint = $eth_endpoint_ws
    | .source.polkadot.endpoint = $relay_chain_endpoint
    | .source.contracts.BeefyClient = $k2
    | .source.contracts.Gateway = $k1
    | .source."channel-id" = $channelID
    | .sink.contracts.Gateway = $k1
    | .sink.ethereum.endpoint = $eth_writer_endpoint
    ' \
        $assets_dir/substrate-relay.json >$output_dir/substrate-relay.json
}

write_beacon_checkpoint() {
    pushd $output_dir > /dev/null
    $relay_bin generate-beacon-checkpoint --config $output_dir/beacon-relay.json --export-json > /dev/null
    cat $output_dir/dump-initial-checkpoint.json
    popd > /dev/null
}

wait_beacon_chain_ready() {
    local initial_beacon_block=""
    while [ -z "$initial_beacon_block" ] || [ "$initial_beacon_block" == "0x0000000000000000000000000000000000000000000000000000000000000000" ]; do
        initial_beacon_block=$(curl -s "$beacon_endpoint_http/eth/v1/beacon/states/head/finality_checkpoints" |
            jq -r '.data.finalized.root' || true)
        sleep 3
    done
}


setup-relayer() {
  config_relayer
  wait_beacon_chain_ready
  write_beacon_checkpoint
}

setup-relayer
