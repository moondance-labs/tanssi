#!/bin/bash

set -e

cd "$(dirname "$0")/.."

# Help
usage() {
  echo "Usage: $0 --prev-runtime=1300 --next-runtime=1400 --prev-client=v0.13.0 --next-client=v0.14.0 --sha=COMMIT_SHA"
  echo ""
  echo "Arguments:"
  echo "  --prev-runtime    Previous runtime version number (e.g. 1300)"
  echo "  --next-runtime    Next runtime version number (e.g. 1400)"
  echo "  --prev-client     Previous client version tag (e.g. v0.13.0)"
  echo "  --next-client     Next client version tag (e.g. v0.14.0)"
  echo "  --sha             Git commit SHA (default: current HEAD)"
  echo "  --help            Show this help message"
  exit 1
}

# Parse arguments
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --prev-runtime=*) PREV_RUNTIME="${1#*=}" ;;
    --next-runtime=*) NEXT_RUNTIME="${1#*=}" ;;
    --prev-client=*) PREV_CLIENT="${1#*=}" ;;
    --next-client=*) NEXT_CLIENT="${1#*=}" ;;
    --sha=*) SHA="${1#*=}" ;;
    --help) usage ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

# Validation
if [[ -z "$PREV_RUNTIME" || -z "$NEXT_RUNTIME" || -z "$PREV_CLIENT" || -z "$NEXT_CLIENT" || -z "$SHA" ]]; then
  echo "Missing required parameters"
  usage
fi

echo "🚀 Launching release workflows..."
echo "  Previous Runtime Version: $PREV_RUNTIME"
echo "  Next Runtime Version:     $NEXT_RUNTIME"
echo "  Previous Client Version:  $PREV_CLIENT"
echo "  Next Client Version:      $NEXT_CLIENT"
echo "  Commit SHA:               $SHA"
echo ""

# 1. Execute Create Client Release ticket CI job for parachain and solochain
echo "🔧 Running Create Client Release CI (Parachain)..."
gh workflow run client-release-issue.yml -f from="$PREV_CLIENT" -f to="$NEXT_CLIENT" -f binary-type="parachain"

echo "🔧 Running Create Client Release CI (Solochain)..."
gh workflow run client-release-issue.yml -f from="$PREV_CLIENT" -f to="$NEXT_CLIENT" -f binary-type="solochain"

# 2. Execute Create runtime release ticket CI job for parachain and solochain
echo "🔧 Running Create Client Release CI (Parachain)..."
gh workflow run runtime-release-issue.yml -f from="$PREV_RUNTIME" -f to="$NEXT_RUNTIME" -f binary-type="parachain"

echo "🔧 Running Create Client Release CI (Solochain)..."
gh workflow run runtime-release-issue.yml -f from="$PREV_RUNTIME" -f to="$NEXT_RUNTIME" -f binary-type="solochain"

# 3. Execute Public Binary draft
echo "🔧 Running Publish Binary Draft..."
gh workflow run publish-binary.yml -f from="$PREV_CLIENT" -f to="$NEXT_CLIENT"

echo "🔧 Publish Dancelight Binary Draft..."
gh workflow run publish-binary-tanssi-solochain.yml -f from="$PREV_CLIENT" -f to="$NEXT_CLIENT"

# 4. Prepare optimized binary drafts
echo "📦 Running Prepare Optimized Binary Draft CI..."
gh workflow run prepare-binary.yml -f sha="$SHA"

echo "📦 Running Prepare Optimized Dancelight Binary Draft CI..."
gh workflow run prepare-tanssi-relay-binary.yml -f sha="$SHA"

# 5. Publish Runtime Draft CI jobs
TAGS=("para" "starlight" "templates")
CHAINS=("orchestrator-para-only" "orchestrator-solo-only" "templates-only")

if [ ${#TAGS[@]} -ne ${#CHAINS[@]} ]; then
  echo "❌ Error: TAGS and CHAINS arrays have different lengths!"
  echo "  TAGS length: ${#TAGS[@]}"
  echo "  CHAINS length: ${#CHAINS[@]}"
  exit 1
fi

for i in "${!TAGS[@]}"; do
  TAG=${TAGS[$i]}
  CHAIN=${CHAINS[$i]}
  echo "📤 Running Publish Runtime Draft for $CHAIN..."
  gh workflow run publish-runtime.yml \
    -f from="runtime-${PREV_RUNTIME}-${TAG}" \
    -f to="runtime-${NEXT_RUNTIME}-${TAG}" \
    -f chains="$CHAIN"
done

echo ""
echo "✅ All workflows triggered successfully!"
