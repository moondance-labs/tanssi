#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

# Check if running on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    echo "Running on macOS - building binaries from source"
    
    # Build Lodestar from source
    echo "Building lodestar from source (version $LODESTAR_TAG)"
    if [ -d "$artifacts_dir/lodestar" ]; then
        echo "Lodestar seems to be already downloaded. Skipping downloading again"
    else
        git clone https://github.com/ChainSafe/lodestar "$artifacts_dir/lodestar"
        pushd "$artifacts_dir/lodestar"
        git fetch && git checkout $LODESTAR_TAG
        popd
    fi

    echo "Building lodestar"
    pushd "$artifacts_dir/lodestar"
    yarn install && yarn run build
    # Create a symlink or copy binary to output directory
    ln -sf "$(pwd)/packages/cli/bin/lodestar" "$output_bin_dir/lodestar"
    chmod +x "$output_bin_dir/lodestar"
    popd
    
    # Build Geth from source
    echo "Building geth from source (version $GETH_TAG)"
    if [ -d "$artifacts_dir/geth" ]; then
        echo "Geth seems to be already downloaded. Skipping downloading"
    else
        git clone https://github.com/ethereum/go-ethereum.git "$artifacts_dir/geth"
        pushd "$artifacts_dir/geth"
        git fetch && git checkout $GETH_TAG
        popd
    fi

    echo "Building Geth"
    pushd "$artifacts_dir/geth"
    GOBIN=$output_bin_dir go install ./cmd/geth
    GOBIN=$output_bin_dir go install ./cmd/abigen
    popd
else
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
fi