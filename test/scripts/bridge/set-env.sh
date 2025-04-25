root_dir="$(realpath .)"

scripts_root_dir="$root_dir/scripts/bridge"
ts_scripts_dir="$scripts_root_dir/ts-scripts"
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
symbiotic_contracts_dir="$artifacts_dir/tanssi-symbiotic"
web_dir="$relayer_root_dir/snowbridge/web"
export contract_dir="$relayer_root_dir/snowbridge/contracts"
test_helpers_dir="$web_dir/packages/test-helpers"
relay_bin="$relayer_root_dir/build/tanssi-bridge-relayer"

RELAYER_COMMIT="05b5e6cf8fe836690cca4e88d2dff3307bf17fa4" # TODO: Change to tag when we do releases
TANSSI_SYMBIOTIC_COMMIT="dd6c64c3555c80f9cc491b9a6b4fa4dfac301263" # TODO: Change to tag when we do release
GETH_TAG="v1.15.3" # We will need to investigate if this is right
LODESTAR_TAG="v1.27.0"

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

export ASSET_HUB_PARAID="${ASSET_HUB_PARAID:-0}"
export ASSET_HUB_AGENT_ID="${ASSET_HUB_AGENT_ID:-0x180bffca5d695ff9c422143d57db8ac7d32f92e6658684e489f947308cc143f5}"

# Token decimal of the relaychain(KSM|ROC:12,DOT:10)
export FOREIGN_TOKEN_DECIMALS=12

## Important accounts

# Useful tool to get these account values: https://www.shawntabrizi.com/substrate-js-utilities/
# Beacon relay account (//BeaconRelay 5GWFwdZb6JyU46e6ZiLxjGxogAHe8SenX76btfq8vGNAaq8c in testnet)
beacon_relayer_pub_key="${BEACON_RELAYER_PUB_KEY:-0xc46e141b5083721ad5f5056ba1cded69dce4a65f027ed3362357605b1687986a}"

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

## Message passing
export PRIMARY_GOVERNANCE_CHANNEL_ID="0x0000000000000000000000000000000000000000000000000000000000000001"
export SECONDARY_GOVERNANCE_CHANNEL_ID="0x0000000000000000000000000000000000000000000000000000000000000002"
export ASSET_HUB_CHANNEL_ID="0xcdd46650b730ab688f476efce47941584cfb1d9abcef4b8bbb51734b4467a918"
# Execution relay account (//ExecutionRelay 5CFNWKMFPsw5Cs2Teo6Pvg7rWyjKiFfqPZs8U4MZXzMYFwXL in testnet)
execution_relayer_assethub_pub_key="${EXECUTION_RELAYER_PUB_KEY:-0x08228efd065c58a043da95c8bf177659fc587643e71e7ed1534666177730196f}"
# Funded ethereum key
ethereum_key="0x5e002a1af63fd31f1c25258f3082dc889762664cb8f218d86da85dff8b07b342"
# Above key's address
ethereum_address="90A987B944Cb1dCcE5564e5FDeCD7a54D3de27Fe"

# Config for deploying contracts

## Deployment key
export PRIVATE_KEY="${DEPLOYER_ETH_KEY:-$ethereum_key}"
export ETHERSCAN_API_KEY="${ETHERSCAN_API_KEY:-0x0}"


snowbridge_address_for() {
    jq -r ".contracts.${1}.address" "$output_dir/snowbridge_contracts.json"
}

symbiotic_address_for() {
    jq -r ".contracts.${1}.address" "$output_dir/symbiotic_contracts.json"
}

kill_all() {
    trap - SIGTERM
    kill 0
}

cleanup() {
    echo "Cleaning resource"
    # rm -rf "$output_dir"
    mkdir -p "$output_dir"
    mkdir -p "$output_bin_dir"
    mkdir -p "$ethereum_data_dir"
}

check_node_version() {
    local expected_version=$1
    
    if ! [ -x "$(command -v node)" ]; then
        echo 'Error: NodeJS is not installed.'
        exit 1
    fi
    
    node_version=$(node -v) # This does not seem to work in Git Bash on Windows.
    # "node -v" outputs version in the format "v18.12.1"
    node_version=${node_version:1} # Remove 'v' at the beginning
    node_version=${node_version%\.*} # Remove trailing ".*".
    node_version=${node_version%\.*} # Remove trailing ".*".
    node_version=$(($node_version)) # Convert the NodeJS version number from a string to an integer.
    if [ $node_version -lt "$expected_version" ]
    then
        echo "NodeJS version is lower than $expected_version (it is $node_version), Please update your node installation!"
        exit 1
    fi
}

vercomp() {
    if [[ $1 == $2 ]]
    then
        echo "Equal"
        return
    fi
    local IFS=.
    local i ver1=($1) ver2=($2)
    # fill empty fields in ver1 with zeros
    for ((i=${#ver1[@]}; i<${#ver2[@]}; i++))
    do
        ver1[i]=0
    done
    for ((i=0; i<${#ver1[@]}; i++))
    do
        if ((10#${ver1[i]:=0} > 10#${ver2[i]:=0}))
        then
            echo "Greater"
            return
        fi
        if ((10#${ver1[i]} < 10#${ver2[i]}))
        then
            echo "Less"
            return
        fi
    done
}


check_go_version() {
    local expected_version=$1
    
    if ! [ -x "$(command -v go)" ]; then
        echo 'Error: Go is not installed.'
        exit 1
    fi
    
    go_version=$(go version | { read _ _ v _; echo ${v#go}; })
    op=$(vercomp "$go_version" "$1")
    
    if [[ $op = "Less" ]]
    then
        echo "Go version is lower than $expected_version (it is $go_version), Please update your go installation!"
        exit 1
    fi
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
        if ! [ -x "$(command -v gsed)" ]; then
            echo 'Error: gsed is not installed.'
            exit 1
        fi
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
    
    check_node_version 22
    check_go_version 1.21.2
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
