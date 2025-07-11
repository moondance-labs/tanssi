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
cd "$(dirname "$0")/.."

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

SPECIFIED_TAG=""
TAG_TO_USE=""
RELAY_TAG_TO_USE=""

# Flags for which artifacts to download
DOWNLOAD_TANSSI_NODE=false
DOWNLOAD_FRONTIER_NODE=false
DOWNLOAD_SIMPLE_NODE=false
DOWNLOAD_RELAY=false
USE_LATEST_CLIENT_VERSION=false
USE_LATEST_RUNTIME_VERSION=false
OUTPUT_TO_TARGET_RELEASE=false
INCLUDE_ALL_ARCHS=false


while [[ $# -gt 0 ]]; do
  case "$1" in
    --latest-client)
      USE_LATEST_CLIENT_VERSION=true
      shift
      ;;
    --latest-runtime)
      USE_LATEST_RUNTIME_VERSION=true
      shift
      ;;
    --tag)
      if [[ -n "${2:-}" && ! "$2" =~ ^-- ]]; then
        SPECIFIED_TAG="$2"
        shift 2
      else
        echo "Error: --tag requires a value" >&2
        exit 1
      fi
      ;;
    --output-to-target-release)
      OUTPUT_TO_TARGET_RELEASE=true
      shift
      ;;
    --include-all-archs)
      INCLUDE_ALL_ARCHS=true
      shift
      ;;
    tanssi-node)
      DOWNLOAD_TANSSI_NODE=true
      shift
      ;;
    container-chain-frontier-node)
      DOWNLOAD_FRONTIER_NODE=true
      shift
      ;;
    container-chain-simple-node)
      DOWNLOAD_SIMPLE_NODE=true
      shift
      ;;
    tanssi-relay)
      DOWNLOAD_RELAY=true
      shift
      ;;
    *)
      echo "Unknown option or binary: $1" >&2
      exit 1
      ;;
  esac
done

# After processing all args, if neither “latest-client” nor “latest-runtime” was requested,
# default to fetching the latest runtime.
if [ -n "$SPECIFIED_TAG" ]; then
  # If the user explicitly asked for *both* latest-client AND latest-runtime, error out.
  if [ "$USE_LATEST_CLIENT_VERSION" = true ] || [ "$USE_LATEST_RUNTIME_VERSION" = true ]; then
    echo "Error: --tag cannot be used with --latest-client or --latest-runtime" >&2
    exit 1
  fi
fi

OUTPUT_PATH="tmp"
if [ "$OUTPUT_TO_TARGET_RELEASE" = true ]; then
  OUTPUT_PATH="../target/release"
fi

# Helper: get the short SHA8 for a given tag
get_sha8() {
  local tag ret resp type url tagresp

  tag=$1

  # Fetch the ref object
  resp=$(curl -s -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/moondance-labs/tanssi/git/refs/tags/$tag")
  ret=$?
  (( ret == 0 )) || return "$ret"

  # Determine if it’s a lightweight or annotated tag
  type=$(jq -r '.object.type' <<<"$resp")
  ret=$?
  (( ret == 0 )) || return "$ret"

  if [[ $type == "commit" ]]; then
    # Lightweight tag: object.sha is the commit
    jq -r '.object.sha' <<<"$resp" | cut -c1-8
  else
    # Annotated tag: need to follow the tag object
    url=$(jq -r '.object.url' <<<"$resp")
    ret=$?
    (( ret == 0 )) || return "$ret"

    tagresp=$(curl -s -H "Accept: application/vnd.github.v3+json" "$url")
    ret=$?
    (( ret == 0 )) || return "$ret"

    jq -r '.object.sha' <<<"$tagresp" | cut -c1-8
  fi
}

# Cache GitHub releases JSON
RELEASES_JSON=$(curl -s https://api.github.com/repos/moondance-labs/tanssi/releases)

# Fetch latest non-starlight runtime tag
if $DOWNLOAD_TANSSI_NODE || $DOWNLOAD_FRONTIER_NODE || $DOWNLOAD_SIMPLE_NODE; then
  if [ -z "$SPECIFIED_TAG" ]; then
    if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
      NONSTARL_TAG=$(jq -r '.[]
        | select(.tag_name | test("v";"i"))
        | .tag_name' <<<"$RELEASES_JSON" | head -n1)
      [[ -n $NONSTARL_TAG ]]
      NONSTARL_SHA=$(get_sha8 "$NONSTARL_TAG")
      runtime_ver=${NONSTARL_TAG//[!0-9]/}
      if (( runtime_ver >= 900 )); then
        TAG_TO_USE="sha-${NONSTARL_SHA}-fast-runtime"
      else
        TAG_TO_USE="sha-${NONSTARL_SHA}"
      fi
    else
      NONSTARL_TAG=$(jq -r '.[]
        | select(
            .tag_name | test("runtime";"i")
            and (test("starlight";"i") | not)
          )
        | .tag_name' <<<"$RELEASES_JSON" | head -n1)
      [[ -n $NONSTARL_TAG ]]
      NONSTARL_SHA=$(get_sha8 "$NONSTARL_TAG")
      runtime_ver=${NONSTARL_TAG//[!0-9]/}
      TAG_TO_USE="sha-${NONSTARL_SHA}-fast-runtime"
      fi
    else
      TAG_TO_USE="${SPECIFIED_TAG}"
    fi
  fi

TANSSI_IMAGE="moondancelabs/tanssi:${TAG_TO_USE}"
FRONTIER_IMAGE="moondancelabs/container-chain-evm-template:${TAG_TO_USE}"
SIMPLE_IMAGE="moondancelabs/container-chain-simple-template:${TAG_TO_USE}"

# Fetch latest starlight runtime tag
if $DOWNLOAD_RELAY; then
  if [ -z "$SPECIFIED_TAG" ]; then
    if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
      STARL_TAG=$(jq -r '.[]
        | select(.tag_name | test("v";"i"))
        | .tag_name' <<<"$RELEASES_JSON" | head -n1)
      [[ -n $STARL_TAG ]]
      STARL_SHA=$(get_sha8 "$STARL_TAG")
      RELAY_TAG_TO_USE="sha-${STARL_SHA}-fast-runtime"
    else
      STARL_TAG=$(jq -r '.[]
        | select(
            .tag_name | test("runtime";"i")
            and test("starlight";"i")
          )
        | .tag_name' <<<"$RELEASES_JSON" | head -n1)
      [[ -n $STARL_TAG ]]
      STARL_SHA=$(get_sha8 "$STARL_TAG")
      RELAY_TAG_TO_USE="sha-${STARL_SHA}-fast-runtime"
    fi
  else
    RELAY_TAG_TO_USE=${SPECIFIED_TAG}
  fi
fi

RELAY_IMAGE="moondancelabs/starlight:${RELAY_TAG_TO_USE}"

# Define Docker images

# Ensure output path exists
mkdir -p $OUTPUT_PATH

# Download requested binaries
if $DOWNLOAD_TANSSI_NODE; then
  echo "Fetching tanssi-node from $TANSSI_IMAGE..."
  BINARIES=(
    tanssi-node
  )
  BINARIES_SKYLAKE=(
    tanssi-node-skylake
  )
  BINARIES_ZNVER3=(
    tanssi-node-znver3
  )
  if [ "$INCLUDE_ALL_ARCHS" = true ]; then
      BINARIES+=("${BINARIES_SKYLAKE[@]}")
      BINARIES+=("${BINARIES_ZNVER3[@]}")
  fi
  docker run --rm \
    --entrypoint tar \
    "$TANSSI_IMAGE" \
    -C /tanssi -cf - \
      "${BINARIES[@]}" | tar -C $OUTPUT_PATH -xf -

  chmod +x $OUTPUT_PATH/tanssi-node
  echo "→ $OUTPUT_PATH/tanssi-node"
fi

if $DOWNLOAD_FRONTIER_NODE; then
  echo "Fetching container-chain-frontier-node from $FRONTIER_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
      # If we are asked to use the latest client version it is hard to find the actual runtime
      # version, but at least we know it is >= 900
      runtime_ver="900"
  fi
  if (( runtime_ver >= 700 )); then
    BINARIES=(
    container-chain-frontier-node
    )
    BINARIES_SKYLAKE=(
      container-chain-frontier-node-skylake
    )
    BINARIES_ZNVER3=(
      container-chain-frontier-node-znver3
    )
    if [ "$INCLUDE_ALL_ARCHS" = true ]; then
      BINARIES+=("${BINARIES_SKYLAKE[@]}")
      BINARIES+=("${BINARIES_ZNVER3[@]}")
    fi
    docker run --rm \
    --entrypoint tar \
    "$FRONTIER_IMAGE" \
    -C /container-chain-template-evm  -cf - \
      "${BINARIES[@]}" | tar -C $OUTPUT_PATH -xf -
  else
    docker run --rm \
        --entrypoint tar \
        "$FRONTIER_IMAGE" \
        -C /container-chain-template-evm -cf - container-chain-template-frontier-node \
      | tar -C $OUTPUT_PATH -xf -
  fi
  chmod +x $OUTPUT_PATH/container-chain-frontier-node
  echo "→ $OUTPUT_PATH/container-chain-frontier-node"
fi

if $DOWNLOAD_SIMPLE_NODE; then
  echo "Fetching container-chain-simple-node from $SIMPLE_IMAGE..."
  # extract numeric version for path logic
  runtime_ver=${NONSTARL_TAG//[!0-9]/}
  if [ "$USE_LATEST_CLIENT_VERSION" = true ]; then
      # If we are asked to use the latest client version it is hard to find the actual runtime
      # version, but at least we know it is >= 900
      runtime_ver="900"
  fi
  if (( runtime_ver >= 700 )); then
    BINARIES=(
    container-chain-simple-node
    )
    BINARIES_SKYLAKE=(
      container-chain-simple-node-skylake
    )
    BINARIES_ZNVER3=(
      container-chain-simple-node-znver3
    )
    if [ "$INCLUDE_ALL_ARCHS" = true ]; then
      BINARIES+=("${BINARIES_SKYLAKE[@]}")
      BINARIES+=("${BINARIES_ZNVER3[@]}")
    fi
    docker run --rm \
    --entrypoint tar \
    "$SIMPLE_IMAGE" \
    -C /container-chain-template-simple -cf - \
      "${BINARIES[@]}" | tar -C $OUTPUT_PATH -xf -
  else
    docker run --rm \
        --entrypoint tar \
        "$SIMPLE_IMAGE" \
        -C /container-chain-template-simple -cf - container-chain-template-simple-node \
      | tar -C $OUTPUT_PATH -xf -
  fi
  chmod +x $OUTPUT_PATH/container-chain-simple-node
  echo "→ $OUTPUT_PATH/container-chain-simple-node"
fi

if $DOWNLOAD_RELAY; then
  echo "Fetching tanssi-relay + workers from $RELAY_IMAGE..."
  BINARIES=(
  tanssi-relay
  tanssi-relay-execute-worker
  tanssi-relay-prepare-worker
  )
  BINARIES_SKYLAKE=(
  tanssi-relay-skylake
  tanssi-relay-execute-worker-skylake
  tanssi-relay-prepare-worker-skylake
  )
  BINARIES_ZNVER3=(
  tanssi-relay-znver3
  tanssi-relay-execute-worker-znver3
  tanssi-relay-prepare-worker-znver3
  )
  if [ "$INCLUDE_ALL_ARCHS" = true ]; then
    BINARIES+=("${BINARIES_SKYLAKE[@]}")
    BINARIES+=("${BINARIES_ZNVER3[@]}")
  fi
  echo "${BINARIES[@]}"
  docker run --rm \
    --entrypoint tar \
    "$RELAY_IMAGE" \
    -C /tanssi-relay -cf - \
      "${BINARIES[@]}" | tar -C $OUTPUT_PATH -xf -

  chmod +x \
    $OUTPUT_PATH/tanssi-relay \
    $OUTPUT_PATH/tanssi-relay-execute-worker \
    $OUTPUT_PATH/tanssi-relay-prepare-worker

  echo "→ $OUTPUT_PATH/tanssi-relay"
  echo "→ $OUTPUT_PATH/tanssi-relay-execute-worker"
  echo "→ $OUTPUT_PATH/tanssi-relay-prepare-worker"
fi

echo "All requested binaries downloaded to $OUTPUT_PATH/"

