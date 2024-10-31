root_dir="$(realpath .)"

scripts_root_dir="$root_dir/scripts/bridge"
assets_dir="$scripts_root_dir/assets"

artifacts_dir="$root_dir/tmp/bridge"
mkdir -p $artifacts_dir

logs_dir="$artifacts_dir/logs"
ethereum_data_dir="$artifacts_dir/ethereum_data"
export output_dir="$artifacts_dir/output"
zombienet_data_dir="$output_dir/zombienet"
export output_bin_dir="$output_dir/bin"
mkdir -p $output_bin_dir
export PATH="$output_bin_dir:$PATH"

relayer_root_dir="$artifacts_dir/relayer"
web_dir="$relayer_root_dir/web"
export contract_dir="$relayer_root_dir/contracts"
test_helpers_dir="$web_dir/packages/test-helpers"
relay_dir="$relayer_root_dir/relayer"
relay_bin="$relay_dir/build/snowbridge-relay"

RELAYER_TAG="relayer-v1.0.30" # we will need to investigate if this is right
GETH_TAG="v1.14.11" # We will need to investigate if this is right
LODESTAR_TAG="v1.19.0"

lodestar_dir=$artifacts_dir/lodestar

geth_dir=$artifacts_dir/geth


export polkadot_sdk_dir="${POLKADOT_SDK_DIR:-../polkadot-sdk}"

eth_network="${ETH_NETWORK:-localhost}"
eth_endpoint_http="${ETH_RPC_ENDPOINT:-http://127.0.0.1:8545}/${INFURA_PROJECT_ID:-}"
eth_endpoint_ws="${ETH_WS_ENDPOINT:-ws://127.0.0.1:8546}/${INFURA_PROJECT_ID:-}"
eth_writer_endpoint="${ETH_WRITER_ENDPOINT:-http://127.0.0.1:8545}/${INFURA_PROJECT_ID:-}"
eth_gas_limit="${ETH_GAS_LIMIT:-5000000}"
eth_chain_id="${ETH_NETWORK_ID:-15}"
etherscan_api_key="${ETHERSCAN_API_KEY:-}"
rebuild_lodestar="${REBUILD_LODESTAR:-true}"

beefy_relay_eth_key="${BEEFY_RELAY_ETH_KEY:-0x935b65c833ced92c43ef9de6bff30703d941bd92a2637cb00cfad389f5862109}"

# Parachain accounts for which the relayer will relay messages over the basic channel.
# These IDs are for the test accounts Alice, Bob, Charlie, Dave, Eve and Ferdie, in order
basic_parachain_account_ids="${BASIC_PARACHAIN_ACCOUNT_IDS:-0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d,0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48,0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22,0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20,0xe659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e,0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c}"
# Ethereum addresses for which the relayer will relay messages over the basic channel.
# This address is for the default eth account used in the E2E tests, taken from test/src/ethclient/index.js.
basic_eth_addresses="${BASIC_ETH_ADDRESSES:-0x89b4ab1ef20763630df9743acf155865600daff2}"
beacon_endpoint_http="${BEACON_HTTP_ENDPOINT:-http://127.0.0.1:9596}"
export BRIDGE_HUB_PARAID="${BRIDGE_HUB_PARAID:-1002}"
export BRIDGE_HUB_AGENT_ID="${BRIDGE_HUB_AGENT_ID:-0x03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314}"
relaychain_ws_url="${RELAYCHAIN_WS_URL:-ws://127.0.0.1:9944}"
relaychain_sudo_seed="${RELAYCHAIN_SUDO_SEED:-//Alice}"

export ASSET_HUB_PARAID="${ASSET_HUB_PARAID:-1000}"
export ASSET_HUB_AGENT_ID="${ASSET_HUB_AGENT_ID:-0x81c5ab2571199e3188135178f3c2c8e2d268be1313d029b30f534fa579b69b79}"

# Token decimal of the relaychain(KSM|ROC:12,DOT:10)
export FOREIGN_TOKEN_DECIMALS=12

## Important accounts

# Useful tool to get these account values: https://www.shawntabrizi.com/substrate-js-utilities/
# Beacon relay account (//BeaconRelay 5GWFwdZb6JyU46e6ZiLxjGxogAHe8SenX76btfq8vGNAaq8c in testnet)
beacon_relayer_pub_key="${BEACON_RELAYER_PUB_KEY:-0xc46e141b5083721ad5f5056ba1cded69dce4a65f027ed3362357605b1687986a}"


# Config for deploying contracts

## Deployment key
export PRIVATE_KEY="${DEPLOYER_ETH_KEY:-0x4e9444a6efd6d42725a250b650a781da2737ea308c839eaccb0f7f3dbd2fea77}"
export ETHERSCAN_API_KEY="${ETHERSCAN_API_KEY:-0x0}"

## BeefyClient
# For max safety delay should be MAX_SEED_LOOKAHEAD=4 epochs=4*8*6=192s
# but for rococo-local each session is only 20 slots=120s
# so relax somehow here just for quick test
# for production deployment ETH_RANDAO_DELAY should be configured in a more reasonable sense
export RANDAO_COMMIT_DELAY="${ETH_RANDAO_DELAY:-3}"
export RANDAO_COMMIT_EXP="${ETH_RANDAO_EXP:-3}"
export MINIMUM_REQUIRED_SIGNATURES="${MINIMUM_REQUIRED_SIGNATURES:-16}"

export REJECT_OUTBOUND_MESSAGES="${REJECT_OUTBOUND_MESSAGES:-false}"

## Fee
export REGISTER_TOKEN_FEE="${REGISTER_TOKEN_FEE:-200000000000000000}"
export CREATE_ASSET_FEE="${CREATE_ASSET_FEE:-100000000000}"
export RESERVE_TRANSFER_FEE="${RESERVE_TRANSFER_FEE:-100000000000}"
export RESERVE_TRANSFER_MAX_DESTINATION_FEE="${RESERVE_TRANSFER_MAX_DESTINATION_FEE:-10000000000000}"

## Pricing Parameters
export EXCHANGE_RATE="${EXCHANGE_RATE:-2500000000000000}"
export DELIVERY_COST="${DELIVERY_COST:-10000000000}"
export FEE_MULTIPLIER="${FEE_MULTIPLIER:-1000000000000000000}"


## Vault
export BRIDGE_HUB_INITIAL_DEPOSIT="${ETH_BRIDGE_HUB_INITIAL_DEPOSIT:-10000000000000000000}"


address_for() {
    jq -r ".contracts.${1}.address" "$output_dir/contracts.json"
}

kill_all() {
    trap - SIGTERM
    kill 0
}

cleanup() {
    echo "Cleaning resource"
    rm -rf "$output_dir"
    mkdir "$output_dir"
    mkdir "$output_bin_dir"
    mkdir "$ethereum_data_dir"
}

check_tool() {
    if ! [ -x "$(command -v g++)" ]; then
        echo 'Error: g++ is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v protoc)" ]; then
        echo 'Error: protoc is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v jq)" ]; then
        echo 'Error: jq is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v mage)" ]; then
        echo 'Error: mage is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v pnpm)" ]; then
        echo 'Error: pnpm is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v forge)" ]; then
        echo 'Error: foundry is not installed.'
        exit 1
    fi
    if ! [ -x "$(command -v yarn)" ]; then
        echo 'Error: yarn is not installed.'
        exit 1
    fi
    if [[ "$(uname)" == "Darwin" && -z "${IN_NIX_SHELL:-}" ]]; then
          if ! [ -x "$(command -v gdate)" ]; then
              echo 'Error: gdate (GNU Date) is not installed.'
              exit 1
          fi
      else
          if ! [ -x "$(command -v date)" ]; then
              echo 'Error: date is not installed.'
              exit 1
          fi
    fi
}

wait_contract_deployed() {
    local ready=""
    while [ -z "$ready" ]; do
        if [ -f "$output_dir/contracts.json" ]; then
            ready="true"
        fi
        sleep 2
    done
}
