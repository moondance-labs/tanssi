#!/bin/bash

# Find duplicate git dependencies in Cargo.lock
# This can happen because in Cargo.toml we only specify the branch name, so if
# a new commit gets pushed to the branch, the Cargo.lock will be out of sync.
# In that case we need to manually run cargo update or edit the Cargo.lock file
# using search and replace.
# Then after a merge conflict, it can happen that not all crates have been updated.

# Always run the commands from the "tanssi" dir
cd $(dirname $0)/../..

# Extract dependencies, sort them uniquely
deps=$(grep "source = \"git\+" Cargo.lock | sort -u)

# Extract the base URLs before any query parameters or fragments
base_urls=$(echo "$deps" | sed -E 's|^(source = "git\+https://[^?#"]+)[^"]*|\1|')

# Check for duplicate base URLs
duplicates=$(echo "$base_urls" | sort | uniq -d)

if [[ -n $duplicates ]]; then
    echo "Error: Duplicate dependencies found with the same URL:"
    echo "$duplicates"
    echo ""
    echo "All git dependencies:"
    echo "$deps"
    echo ""
    echo "Help: use this command to update polkadot-sdk to the latest commit:"
    echo "cargo update -p sp_core"
    exit 1
else
    echo "No duplicates found. All good!"
fi
