#!/bin/bash

# Exit on any error
set -e

# Start ethereum nodes, the nodes must be built first
scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh


mkdir -p $ethereum_data_dir
echo $ethereum_data_dir

mkdir -p $output_dir
mkdir -p $logs_dir

start_geth() {
    echo "Starting geth local node"
    local timestamp="0" #start Prague from genesis
    jq \
        --argjson timestamp "$timestamp" \
        '
        .config.PragueTime = $timestamp
        ' \
        $assets_dir/genesis.json > $output_dir/genesis.json
    geth init --datadir "$ethereum_data_dir" --state.scheme=hash "$output_dir/genesis.json"
    geth --vmdebug --datadir "$ethereum_data_dir" --networkid 11155111 \
        --http --http.api debug,personal,eth,net,web3,txpool,engine,miner --ws --ws.api debug,eth,net,web3 \
        --rpc.allow-unprotected-txs --mine \
        --miner.etherbase=0xBe68fC2d8249eb60bfCf0e71D5A0d2F2e292c4eD \
        --authrpc.addr="127.0.0.1" \
        --http.addr="0.0.0.0" \
        --ws.addr="0.0.0.0" \
        --http.corsdomain '*' \
        --allow-insecure-unlock \
        --authrpc.jwtsecret $assets_dir/jwtsecret \
        --password /dev/null \
        --rpc.gascap 0 \
        --ws.origins "*" \
        --trace "$ethereum_data_dir/trace" \
        --gcmode archive \
        --syncmode=full \
        --state.scheme=hash \
        >"$logs_dir/geth.log" 2>&1 &
     echo "geth=$!" >> $artifacts_dir/daemons.pid
}

start_lodestar() {
    echo "Starting lodestar local node"
    local genesisHash=$(curl $eth_endpoint_http \
        -X POST \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc": "2.0", "id": "1", "method": "eth_getBlockByNumber","params": ["0x0", false]}' | jq -r '.result.hash')
    echo "genesisHash is: $genesisHash"
    # use gdate here for raw macos without nix
    local timestamp=""
    if [[ "$(uname)" == "Darwin" && -z "${IN_NIX_SHELL:-}" ]]; then
        timestamp=$(gdate -d'+10second' +%s)
    else
        timestamp=$(date -d'+10second' +%s)
    fi

    export LODESTAR_PRESET="mainnet"

    pushd $artifacts_dir/lodestar
    ./lodestar dev \
        --genesisValidators 8 \
        --genesisTime $timestamp \
        --startValidators "0..7" \
        --enr.ip6 "127.0.0.1" \
        --rest.address "0.0.0.0" \
        --eth1.providerUrls "http://127.0.0.1:8545" \
        --execution.urls "http://127.0.0.1:8551" \
        --dataDir "$ethereum_data_dir" \
        --reset \
        --terminal-total-difficulty-override 0 \
        --genesisEth1Hash $genesisHash \
        --params.ALTAIR_FORK_EPOCH 0 \
        --params.BELLATRIX_FORK_EPOCH 0 \
        --params.CAPELLA_FORK_EPOCH 0 \
        --params.DENEB_FORK_EPOCH 0 \
        --params.ELECTRA_FORK_EPOCH 0 \
        --eth1=true \
        --rest.namespace="*" \
        --jwt-secret $assets_dir/jwtsecret \
        --chain.archiveStateEpochFrequency 1 \
        >"$logs_dir/lodestar.log" 2>&1 &
    echo "lodestar=$!" >> $artifacts_dir/daemons.pid
    popd
}

deploy_local() {
    # 1. deploy execution client
    echo "Starting execution node"
    start_geth

    echo "Waiting for geth API to be ready"
    sleep 3

    # 2. deploy consensus client
    echo "Starting beacon node"
    start_lodestar
}

echo "start ethereum only!"
trap kill_all SIGINT SIGTERM EXIT
deploy_local
echo "ethereum local nodes started!"
wait
