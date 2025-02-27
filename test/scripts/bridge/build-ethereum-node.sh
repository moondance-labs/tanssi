#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

# Check if lodestar is already installed in our output directory
if [ -f "$output_bin_dir/lodestar" ] && [ -x "$output_bin_dir/lodestar" ]; then
    lodestar_version=$($output_bin_dir/lodestar --version 2>/dev/null | grep -o "v[0-9]\+\.[0-9]\+\.[0-9]\+" || echo "unknown")
    echo "Lodestar $lodestar_version seems to be already installed in $output_bin_dir. Skipping download"
    if [[ "$lodestar_version" != "$LODESTAR_TAG" && "$lodestar_version" != "unknown" ]]; then
        echo "WARNING: Installed Lodestar version ($lodestar_version) does not match expected version ($LODESTAR_TAG)"
    fi
else
    echo "Downloading lodestar from GitHub releases (version $LODESTAR_TAG)"
    
    # assuming linux since the rest of the project requires it
    DOWNLOAD_URL="https://github.com/ChainSafe/lodestar/releases/download/$LODESTAR_TAG/lodestar-$LODESTAR_TAG-linux-amd64.tar.gz"
    
    echo "Download URL: $DOWNLOAD_URL"
    
    TMP_DIR=$(mktemp -d)
    
    curl -L "$DOWNLOAD_URL" -o "$TMP_DIR/lodestar.tar.gz"
    tar -xzf "$TMP_DIR/lodestar.tar.gz" -C "$TMP_DIR"
    
    cp "$TMP_DIR/lodestar" "$output_bin_dir/lodestar"
    chmod +x "$output_bin_dir/lodestar"
    rm -rf "$TMP_DIR"
    
    echo "Lodestar binary (version $LODESTAR_TAG) has been downloaded and placed in $output_bin_dir"
fi

# Check if geth is already installed in our output directory
if [ -f "$output_bin_dir/geth" ] && [ -x "$output_bin_dir/geth" ]; then
    geth_version=$($output_bin_dir/geth --version 2>/dev/null | awk '{print $3}' | cut -d'-' -f1 || echo "unknown")
    echo "Geth $geth_version seems to be already installed in $output_bin_dir. Skipping download"
    if [[ "v$geth_version" != "$GETH_TAG" && "$geth_version" != "unknown" ]]; then
        echo "WARNING: Installed Geth version ($geth_version) does not match expected version ($GETH_TAG)"
    fi
else
    echo "Downloading geth from Docker image (version $GETH_TAG)"
    
    container_id=$(docker create ethereum/client-go:alltools-${GETH_TAG})
    echo "Extracting geth binary..."
    docker cp $container_id:/usr/local/bin/geth $output_bin_dir/
    
    echo "Extracting abigen binary..."
    docker cp $container_id:/usr/local/bin/abigen $output_bin_dir/
    
    docker rm $container_id
    
    chmod +x $output_bin_dir/geth
    chmod +x $output_bin_dir/abigen
    
    echo "Geth binaries (version $GETH_TAG) have been downloaded and placed in $output_bin_dir"
fi