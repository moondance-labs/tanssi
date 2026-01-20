#!/bin/bash

# THIS FILE IS ONLY FOR TESTING PURPOSE OF THE MIGRATION FROM build-spec TO export-chain-spec
# After this command is removed (around April 2026), this file can be removed as well

set -e

# Always run from project root
cd "$(dirname "$0")"/..

if [[ -z "${1:-}" ]]; then
    BINARY_FOLDER="../target/release"
else
    BINARY_FOLDER="${1}"
fi

mkdir -p specs specs/tmp

compare_specs() {
    local label="$1"
    local build_file="$2"
    local export_file="$3"

    echo "Comparing $label..."

    if ! diff -u "$build_file" "$export_file" > specs/tmp/diff.txt; then
        echo "❌ ERROR: Specs differ for $label"
        echo "------- DIFF -------"
        cat specs/tmp/diff.txt
        echo "---------------------"
        exit 1
    else
        echo "✅ OK: $label specs match"
    fi
}

#############################################
# 1️⃣ Generate container chain specs
#############################################

echo "Generating container-chain-simple-node specs..."
$BINARY_FOLDER/container-chain-simple-node \
    build-spec \
    --disable-default-bootnode \
    --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" \
    --parachain-id 2000 \
    --raw \
    > specs/simple-2000-build.json

$BINARY_FOLDER/container-chain-simple-node \
    export-chain-spec \
    --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" \
    --parachain-id 2000 \
    --raw \
    > specs/simple-2000-export.json

compare_specs \
    "container-chain-simple-node (2000)" \
    specs/simple-2000-build.json \
    specs/simple-2000-export.json


#############################################
# 2️⃣ container-chain-frontier-node
#############################################

echo "Generating container-chain-frontier-node specs..."
$BINARY_FOLDER/container-chain-frontier-node \
    build-spec \
    --disable-default-bootnode \
    --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" \
    --parachain-id 2001 \
    --raw \
    > specs/frontier-2001-build.json

$BINARY_FOLDER/container-chain-frontier-node \
    export-chain-spec \
    --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" \
    --parachain-id 2001 \
    --raw \
    > specs/frontier-2001-export.json

compare_specs \
    "container-chain-frontier-node (2001)" \
    specs/frontier-2001-build.json \
    specs/frontier-2001-export.json


#############################################
# 3️⃣ tanssi-relay
#############################################

echo "Generating tanssi-relay build vs export..."

# Pre-requisit
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --raw > specs/single-container-template-container-2000.json
$BINARY_FOLDER/container-chain-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --raw > specs/single-container-template-container-2001.json
$BINARY_FOLDER/container-chain-simple-node build-spec --disable-default-bootnode --parachain-id 2002 --raw > specs/single-container-template-container-2002.json
$BINARY_FOLDER/tanssi-relay build-spec --chain starlight-local  --disable-default-bootnode --add-container-chain specs/single-container-template-container-2000.json --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" --invulnerable "Collator-05" --invulnerable "Collator-06" > specs/tanssi-relay-build.json

# export-chain-spec version
$BINARY_FOLDER/tanssi-relay export-chain-spec --chain starlight-local --add-container-chain specs/single-container-template-container-2000.json --add-container-chain specs/single-container-template-container-2001.json --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" --invulnerable "Collator-05" --invulnerable "Collator-06" > specs/tanssi-relay-export.json


compare_specs \
    "tanssi-relay" \
    specs/tanssi-relay-build.json \
    specs/tanssi-relay-export.json


#############################################
# 4️⃣ tanssi-node
#############################################

echo "Generating tanssi-node build vs export..."

$BINARY_FOLDER/tanssi-node build-spec --chain dancebox-local --disable-default-bootnode --parachain-id 1000 --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" --invulnerable "Collator-05" > specs/tanssi-node-build.json

$BINARY_FOLDER/tanssi-node export-chain-spec --chain dancebox-local --parachain-id 1000 --invulnerable "Collator-01" --invulnerable "Collator-02" --invulnerable "Collator-03" --invulnerable "Collator-04" --invulnerable "Collator-05" > specs/tanssi-node-export.json


compare_specs \
    "tanssi-node flashbox-local" \
    specs/tanssi-node-build.json \
    specs/tanssi-node-export.json


#############################################
# ☑️ All checks passed
#############################################

echo "🎉 All specs match! Test passed."
