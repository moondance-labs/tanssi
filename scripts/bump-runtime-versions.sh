#!/bin/bash

# Example:
# ./scripts/bump-runtime-versions.sh --client-version=0.9.0 --runtime-version=900

# Exit on any error
set -e

# Always run the commands from the project dir
cd "$(dirname "$0")/.."

# Function to display help message
display_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "This script updates the client version in Cargo.toml files and"
    echo "the runtime version in lib.rs files for all the runtimes in this repo."
    echo ""
    echo "Options:"
    echo "  --client-version=VERSION     Set the client version (required)"
    echo "  --runtime-version=VERSION    Set the runtime version (required)"
    echo "  --help                       Display this help message"
    echo ""
}

# Initialize variables to check if values are provided
CLIENT_VERSION=""
RUNTIME_VERSION=""

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --client-version=*) CLIENT_VERSION="${1#*=}"; shift ;;
        --runtime-version=*) RUNTIME_VERSION="${1#*=}"; shift ;;
        --client-version) CLIENT_VERSION="$2"; shift 2 ;;
        --runtime-version) RUNTIME_VERSION="$2"; shift 2 ;;
        --help) display_help; exit 0 ;;
        *) echo "Unknown parameter: $1"; display_help; exit 1 ;;
    esac
done

# Check if required parameters are provided
if [[ -z "$CLIENT_VERSION" || -z "$RUNTIME_VERSION" ]]; then
    echo "Error: --client-version and --runtime-version are required."
    display_help
    exit 1
fi

# Function to update the version in Cargo.toml files
update_cargo_toml() {
    local file=$1
    local new_version=$2
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \".*\"$/version = \"$new_version\"/" "$file"
    else
        sed -i "s/^version = \".*\"$/version = \"$new_version\"/" "$file"
    fi
    echo "Updated $file to version $new_version"
}

# Function to update the runtime version in lib.rs files
update_lib_rs() {
    local file=$1
    local new_version=$2
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/.*spec_version: .*,/    spec_version: $new_version,/" "$file"
    else
        sed -i "s/.*spec_version: .*,/    spec_version: $new_version,/" "$file"
    fi
    echo "Updated $file to spec_version $new_version"
}

# Function to update the version in a package.json file for typescript-api
update_package_json() {
    local file=$1
    local runtime_version=$2
    local new_version="0.${runtime_version}.0"

    if command -v jq >/dev/null 2>&1; then
        tmp_file=$(mktemp)
        jq --arg v "$new_version" '.version = $v' "$file" > "$tmp_file" && mv "$tmp_file" "$file"
        echo "Updated $file to version $new_version using jq"
    else
        # Fallback using sed if jq is not available
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/\"version\": \".*\"/\"version\": \"$new_version\"/" "$file"
        else
            sed -i "s/\"version\": \".*\"/\"version\": \"$new_version\"/" "$file"
        fi
        echo "Updated $file to version $new_version using sed"
    fi
}

# Update Cargo.toml files
update_cargo_toml "chains/container-chains/nodes/frontier/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "chains/container-chains/nodes/simple/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "chains/orchestrator-paras/node/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "chains/orchestrator-relays/client/cli/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "chains/orchestrator-relays/node/tanssi-relay/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "chains/orchestrator-relays/node/tanssi-relay-service/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "client/service-orchestrator-chain/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "client/service-container-chain-rpc-provider/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "client/service-container-chain-spawner/Cargo.toml" "$CLIENT_VERSION"
update_cargo_toml "client/node-common/Cargo.toml" "$CLIENT_VERSION"

# Update lib.rs files
update_lib_rs "chains/container-chains/runtime-templates/frontier/src/lib.rs" "$RUNTIME_VERSION"
update_lib_rs "chains/container-chains/runtime-templates/simple/src/lib.rs" "$RUNTIME_VERSION"
update_lib_rs "chains/orchestrator-paras/runtime/dancebox/src/lib.rs" "$RUNTIME_VERSION"
update_lib_rs "chains/orchestrator-paras/runtime/flashbox/src/lib.rs" "$RUNTIME_VERSION"
update_lib_rs "chains/orchestrator-relays/runtime/dancelight/src/lib.rs" "$RUNTIME_VERSION"
update_lib_rs "chains/orchestrator-relays/runtime/starlight/src/lib.rs" "$RUNTIME_VERSION"

update_package_json "typescript-api/package.json" "$RUNTIME_VERSION"

echo "All files updated successfully. Updating Cargo.lock"

# Update Cargo.lock file
cargo metadata --format-version=1 > /dev/null

echo "Done. Run this command to create a commit:"
echo "git commit -am 'Bump node and runtime versions to ""$RUNTIME_VERSION""'"
