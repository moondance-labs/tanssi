#!/bin/bash

# Utility script to detect which chain spec command is available
# Usage: check-chain-spec-cmd.sh <binary-path>
# Returns: "export-chain-spec" or "build-spec"
# Exit code: 0 if found, 1 if neither available

BINARY="$1"

if [[ -z "$BINARY" ]]; then
    echo "Error: Binary path required" >&2
    exit 1
fi

if [[ ! -x "$BINARY" ]]; then
    echo "Error: Binary not found or not executable: $BINARY" >&2
    exit 1
fi

# Check for export-chain-spec (new command)
if "$BINARY" export-chain-spec --help &>/dev/null; then
    echo "export-chain-spec"
    exit 0
fi

# Fallback to build-spec (legacy command)
if "$BINARY" build-spec --help &>/dev/null; then
    echo "build-spec"
    exit 0
fi

# Neither command found
echo "Error: Neither export-chain-spec nor build-spec available" >&2
exit 1