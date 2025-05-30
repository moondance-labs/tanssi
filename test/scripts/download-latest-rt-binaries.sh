#!/usr/bin/env bash
#
# download-latest-rt-binaries.sh <binary>...
# Supported binaries:
#   tanssi-node
#   container-chain-frontier-node
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

for bin in "$@"; do
  case "$bin" in
    tanssi-node) DOWNLOAD_TANSSI_NODE=true ;; 
    container-chain-frontier-node) DOWNLOAD_FRONTIER_NODE=true ;; 
    container-chain-simple-node) DOWNLOAD_SIMPLE_NODE=true ;; 
    tanssi-relay) DOWNLOAD_RELAY=true ;; 
    undefined) echo "bug in moonwall passing undefined as an arg" ;;
    *) echo "Unknown binary: $bin" >&2; exit 1 ;;
  esac
done

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
  NONSTARL_TAG=$(jq -r '.[]
    | select(
        .tag_name | test("runtime";"i")
        and (test("starlight";"i") | not)
      )
    | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  [[ -n $NONSTARL_TAG ]]
  NONSTARL_SHA=$(get_sha8 "$NONSTARL_TAG")
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
  STARL_TAG=$(jq -r '.[]
    | select(
        .tag_name | test("runtime";"i")
        and test("starlight";"i")
      )
    | .tag_name' <<<"$RELEASES_JSON" | head -n1)
  [[ -n $STARL_TAG ]]
  STARL_SHA=$(get_sha8 "$STARL_TAG")
  RELAY_IMAGE="moondancelabs/starlight:sha-${STARL_SHA}-fast-runtime"
fi

# Define Docker images

# Ensure tmp exists
mkdir -p tmp

# Remove any leftover containers
docker rm -f tanssi_container frontier_container starlight_container 2>/dev/null || true

# Download requested binaries
if $DOWNLOAD_TANSSI_NODE; then
  echo "Fetching tanssi-node from $TANSSI_IMAGE..."
  docker create --name tanssi_container "$TANSSI_IMAGE" bash
  docker cp tanssi_container:tanssi/tanssi-node tmp/tanssi-node
  docker rm -f tanssi_container
  chmod +x tmp/tanssi-node
  echo "→ tmp/tanssi-node"
fi

if $DOWNLOAD_FRONTIER_NODE; then
  echo "Fetching container-chain-frontier-node from $FRONTIER_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  docker create --name frontier_container "$FRONTIER_IMAGE" bash
  if (( runtime_ver >= 700 )); then
    docker cp frontier_container:container-chain-evm-template/container-chain-frontier-node tmp/container-chain-frontier-node
  else
    docker cp frontier_container:container-chain-evm-template/container-chain-template-frontier-node tmp/container-chain-frontier-node
  fi
  docker rm -f frontier_container
  chmod +x tmp/container-chain-frontier-node
  echo "→ tmp/container-chain-frontier-node"
fi

if $DOWNLOAD_SIMPLE_NODE; then
  echo "Fetching container-chain-simple-node from $SIMPLE_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  docker create --name simple_container "$SIMPLE_IMAGE" bash
  if (( runtime_ver >= 700 )); then
    docker cp simple_container:container-chain-simple-template/container-chain-simple-node tmp/container-chain-simple-node
  else
    docker cp simple_container:container-chain-simple-template/container-chain-template-simple-node tmp/container-chain-simple-node
  fi
  docker rm -f simple_container
  chmod +x tmp/container-chain-simple-node
  echo "→ tmp/container-chain-simple-node"
fi

if $DOWNLOAD_RELAY; then
  echo "Fetching tanssi-relay + workers from $RELAY_IMAGE..."
  docker create --name starlight_container "$RELAY_IMAGE" bash
  docker cp starlight_container:tanssi-relay/tanssi-relay           tmp/tanssi-relay
  docker cp starlight_container:tanssi-relay/tanssi-relay-execute-worker tmp/tanssi-relay-execute-worker
  docker cp starlight_container:tanssi-relay/tanssi-relay-prepare-worker tmp/tanssi-relay-prepare-worker
  docker rm -f starlight_container
  chmod +x tmp/tanssi-relay tmp/tanssi-relay-execute-worker tmp/tanssi-relay-prepare-worker
  echo "→ tmp/tanssi-relay"
  echo "→ tmp/tanssi-relay-execute-worker"
  echo "→ tmp/tanssi-relay-prepare-worker"
fi

echo "All requested binaries downloaded to tmp/"

