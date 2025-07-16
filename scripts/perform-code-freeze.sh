#!/bin/bash

# Exit on any error
set -e

# Always run from project root
cd "$(dirname "$0")/.."

# Help
usage() {
  echo "Usage: $0 --runtime-version=XXXX --client-version=v0.XX.0 [--from-branch=master]"
  echo "Example: $0 --runtime-version=1234 --client-version=v0.14.0"
  exit 1
}

# Defaults
FROM_BRANCH="master"

# Parse arguments
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --runtime-version=*) RUNTIME_VERSION="${1#*=}" ;;
    --client-version=*) CLIENT_VERSION="${1#*=}" ;;
    --from-branch=*) FROM_BRANCH="${1#*=}" ;;
    --help) usage ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

# Validate input
if [[ -z "$RUNTIME_VERSION" || -z "$CLIENT_VERSION" ]]; then
  echo "Error: --runtime-version and --client-version are required"
  usage
fi

# Ensure latest info
git fetch origin

# Checkout and update base branch
git checkout "$FROM_BRANCH"
git pull origin "$FROM_BRANCH"

# Define branches to create
BRANCHES=(
  "perm-runtime-${RUNTIME_VERSION}"
  "perm-${CLIENT_VERSION}"
  "perm-runtime-${RUNTIME_VERSION}-starlight"
  "perm-runtime-${RUNTIME_VERSION}-templates"
)

# Create and push each branch
for BRANCH in "${BRANCHES[@]}"; do
  git checkout -b "$BRANCH"
  git push -u origin "$BRANCH"
done

echo "✅ All branches created and pushed successfully:"
printf "  - %s\n" "${BRANCHES[@]}"