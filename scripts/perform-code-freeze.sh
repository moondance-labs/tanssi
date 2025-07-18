#!/bin/bash

# Exit on any error
set -e

# Always run from project root
cd "$(dirname "$0")/.."

# Help
usage() {
  echo "Usage: $0 --runtime-version=XXXX --client-version=v0.XX.0 [--from-branch=master] [--from-commit=abcdef]"
  echo "Example: $0 --runtime-version=1234 --client-version=v0.14.0 --from-commit=abc1234"
  exit 1
}

# Defaults
DEFAULT_BRANCH="master"
FROM_BRANCH=""
FROM_COMMIT=""

# Parse arguments
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --runtime-version=*) RUNTIME_VERSION="${1#*=}" ;;
    --client-version=*) CLIENT_VERSION="${1#*=}" ;;
    --from-branch=*) FROM_BRANCH="${1#*=}" ;;
    --from-commit=*) FROM_COMMIT="${1#*=}" ;;
    --help) usage ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

# Validate input
if [[ -z "$RUNTIME_VERSION" || -z "$CLIENT_VERSION" ]]; then
  echo "❌ Error: --runtime-version and --client-version are required"
  usage
fi

if [[ -n "$FROM_BRANCH" && -n "$FROM_COMMIT" ]]; then
  echo "❌ Error: Specify only one of --from-branch or --from-commit"
  usage
fi

if [[ -z "$FROM_BRANCH" && -z "$FROM_COMMIT" ]]; then
  FROM_BRANCH="$DEFAULT_BRANCH"
fi

# Ensure latest info
git fetch origin

# Determine base ref
if [[ -n "$FROM_BRANCH" ]]; then
  BASE_REF="$FROM_BRANCH"
  echo "🔄 Using branch $FROM_BRANCH as base"
  git checkout "$FROM_BRANCH"
  git pull origin "$FROM_BRANCH"
else
  BASE_REF="$FROM_COMMIT"
  echo "🔄 Using commit $FROM_COMMIT as base"
  git checkout "$FROM_COMMIT"
fi

# Define branches to create
BRANCHES=(
  "perm-runtime-${RUNTIME_VERSION}"
  "perm-${CLIENT_VERSION}"
  "perm-runtime-${RUNTIME_VERSION}-starlight"
  "perm-runtime-${RUNTIME_VERSION}-templates"
  "perm-runtime-${RUNTIME_VERSION}-para"
)

TAGS=(
  "runtime-${RUNTIME_VERSION}-starlight"
  "runtime-${RUNTIME_VERSION}-para"
  "runtime-${RUNTIME_VERSION}-templates"
  "${CLIENT_VERSION}"
  "${CLIENT_VERSION}-para"
)

# Create and push each branch
for BRANCH in "${BRANCHES[@]}"; do
  echo "🚀 Creating branch: $BRANCH from $BASE_REF"
  git checkout "$BASE_REF"
  git checkout -b "$BRANCH"
  git push -u origin "$BRANCH"
done

git checkout "$BASE_REF"

# Create and push tags
for TAG in "${TAGS[@]}"; do
  echo "🏷  Creating tag: $TAG"
  git tag "$TAG"
  git push origin "$TAG"
done

echo "✅ All branches and tags created and pushed successfully:"
printf "  - Branch: %s\n" "${BRANCHES[@]}"
printf "  - Tag:    %s\n" "${TAGS[@]}"