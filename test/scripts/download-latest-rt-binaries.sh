#!/usr/bin/env bash
#
# download-latest-rt-binaries.sh <binary>...
# Supported binaries:
#   tanssi-node
#   container-chain-frontier-node
#   container-chain-simple-node
#   tanssi-relay      # includes tanssi-relay + worker binaries
#
set -euo pipefail
# Always run the commands from the "test" dir
cd $(dirname $0)/..

if [[ $# -lt 1 ]]; then
  cat <<EOF
Usage: $0 <binary>...
Supported binaries:
  tanssi-node
  container-chain-frontier-node
  container-chain-simple-node
  tanssi-relay      # includes tanssi-relay, tanssi-relay-execute-worker, tanssi-relay-prepare-worker
EOF
  exit 1
fi

# Flags for which artifacts to download
DOWNLOAD_TANSSI_NODE=false
DOWNLOAD_FRONTIER_NODE=false
DOWNLOAD_SIMPLE_NODE=false
DOWNLOAD_RELAY=false
USE_LATEST_CLIENT_VERSION=false
USE_LATEST_RUNTIME_VERSION=false

for bin in "$@"; do
  case "$bin" in
    --latest-client) USE_LATEST_CLIENT_VERSION=true ;;
    --latest-runtime) USE_LATEST_RUNTIME_VERSION=true ;;
    tanssi-node) DOWNLOAD_TANSSI_NODE=true ;; 
    container-chain-frontier-node) DOWNLOAD_FRONTIER_NODE=true ;; 
    container-chain-simple-node) DOWNLOAD_SIMPLE_NODE=true ;; 
    tanssi-relay) DOWNLOAD_RELAY=true ;; 
    undefined) echo "bug in moonwall passing undefined as an arg" ;;
    *) echo "Unknown binary: $bin" >&2; exit 1 ;;
  esac
done


# After processing all args, if neither “latest-client” nor “latest-runtime” was requested,
# default to fetching the latest runtime.
if [ "$USE_LATEST_CLIENT_VERSION" = false ] && [ "$USE_LATEST_RUNTIME_VERSION" = false ]; then
  USE_LATEST_RUNTIME_VERSION=true
fi

# If the user explicitly asked for *both* latest-client AND latest-runtime, error out.
if [ "$USE_LATEST_CLIENT_VERSION" = true ] && [ "$USE_LATEST_RUNTIME_VERSION" = true ]; then
  echo "Error: cannot use both --latest-client and --latest-runtime" >&2
  exit 1
fi

# Helper: get the short SHA8 for a given tag
get_sha8() {
  local tag=$1
  local resp=$(curl -s -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/moondance-labs/tanssi/git/refs/tags/$tag")
  local type=$(jq -r '.object.type' <<<"$resp")

  if [[ $type == "commit" ]]; then
    jq -r '.object.sha' <<<"$resp" | cut -c1-8
  else
    local url=$(jq -r '.object.url' <<<"$resp")
    local tagresp=$(curl -s -H "Accept: application/vnd.github.v3+json" "$url")
    jq -r '.object.sha' <<<"$tagresp" | cut -c1-8
  fi
}

# Cache GitHub releases JSON
RELEASES_JSON=$(curl -s https://api.github.com/repos/moondance-labs/tanssi/releases)

# Fetch latest non-starlight runtime tag
if $DOWNLOAD_TANSSI_NODE || $DOWNLOAD_FRONTIER_NODE || $DOWNLOAD_SIMPLE_NODE; then
  if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
    NONSTARL_TAG=$(jq -r '.[]
      | select(.tag_name | test("v";"i"))
      | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  else
    NONSTARL_TAG=$(jq -r '.[]
      | select(
          .tag_name | test("runtime";"i")
          and (test("starlight";"i") | not)
        )
      | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  fi
  [[ -n $NONSTARL_TAG ]]
  NONSTARL_SHA=$(get_sha8 "$NONSTARL_TAG")
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  if (( runtime_ver >= 900 )); then
    TANSSI_IMAGE="moondancelabs/tanssi:sha-${NONSTARL_SHA}-fast-runtime"
  else
    TANSSI_IMAGE="moondancelabs/tanssi:sha-${NONSTARL_SHA}"
  fi
  if (( runtime_ver >= 900 )); then
    FRONTIER_IMAGE="moondancelabs/container-chain-evm-template:sha-${NONSTARL_SHA}-fast-runtime"
  else
    FRONTIER_IMAGE="moondancelabs/container-chain-evm-template:sha-${NONSTARL_SHA}"
  fi
  if (( runtime_ver >= 900 )); then
    SIMPLE_IMAGE="moondancelabs/container-chain-simple-template:sha-${NONSTARL_SHA}-fast-runtime"
  else
    SIMPLE_IMAGE="moondancelabs/container-chain-simple-template:sha-${NONSTARL_SHA}"
  fi
fi

# Fetch latest starlight runtime tag
if $DOWNLOAD_RELAY; then
  if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
    STARL_TAG=$(jq -r '.[]
      | select(.tag_name | test("v";"i"))
      | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  else
    STARL_TAG=$(jq -r '.[]
      | select(
          .tag_name | test("runtime";"i")
          and test("starlight";"i")
        )
      | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  fi
  [[ -n $STARL_TAG ]]
  STARL_SHA=$(get_sha8 "$STARL_TAG")
  RELAY_IMAGE="moondancelabs/starlight:sha-${STARL_SHA}-fast-runtime"
fi

# Define Docker images

# Ensure tmp exists
mkdir -p tmp

# Download requested binaries
if $DOWNLOAD_TANSSI_NODE; then
  echo "Fetching tanssi-node from $TANSSI_IMAGE..."
  docker run --rm \
      --entrypoint tar \
      "$TANSSI_IMAGE" \
      -C / -cf - tanssi/tanssi-node \
    | tar -C tmp -xf -
  chmod +x tmp/tanssi-node
  echo "→ tmp/tanssi-node"
fi

if $DOWNLOAD_FRONTIER_NODE; then
  echo "Fetching container-chain-frontier-node from $FRONTIER_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  if (( runtime_ver >= 700 )); then
    docker run --rm \
        --entrypoint tar \
        "$FRONTIER_IMAGE" \
        -C / -cf - container-chain-evm-template/container-chain-frontier-node \
      | tar -C tmp -xf -
  else
    docker run --rm \
        --entrypoint tar \
        "$FRONTIER_IMAGE" \
        -C / -cf - container-chain-evm-template/container-chain-template-frontier-node \
      | tar -C tmp -xf -
  fi
  chmod +x tmp/container-chain-frontier-node
  echo "→ tmp/container-chain-frontier-node"
fi

if $DOWNLOAD_SIMPLE_NODE; then
  echo "Fetching container-chain-simple-node from $SIMPLE_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  if (( runtime_ver >= 700 )); then
    docker run --rm \
        --entrypoint tar \
        "$SIMPLE_IMAGE" \
        -C / -cf - container-chain-simple-template/container-chain-simple-node \
      | tar -C tmp -xf -
  else
    docker run --rm \
        --entrypoint tar \
        "$SIMPLE_IMAGE" \
        -C / -cf - container-chain-simple-template/container-chain-template-simple-node \
      | tar -C tmp -xf -
  fi
  chmod +x tmp/container-chain-simple-node
  echo "→ tmp/container-chain-simple-node"
fi

if $DOWNLOAD_RELAY; then
  echo "Fetching tanssi-relay + workers from $RELAY_IMAGE..."
  docker run --rm \
    --entrypoint tar \
    "$RELAY_IMAGE" \
    -C / -cf - \
      tanssi-relay/tanssi-relay \
      tanssi-relay/tanssi-relay-execute-worker \
      tanssi-relay/tanssi-relay-prepare-worker \
  | tar -C tmp -xf -

  chmod +x \
    tmp/tanssi-relay \
    tmp/tanssi-relay-execute-worker \
    tmp/tanssi-relay-prepare-worker

  echo "→ tmp/tanssi-relay"
  echo "→ tmp/tanssi-relay-execute-worker"
  echo "→ tmp/tanssi-relay-prepare-worker"
fi

echo "All requested binaries downloaded to tmp/"

