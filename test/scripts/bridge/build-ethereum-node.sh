#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

echo "Building lodestar Snowfork fork"


set_slot_time() {
    local new_value=$1
    echo "Hack lodestar for faster slot time"
    local preset_mainnet_config_file="$artifacts_dir/lodestar/packages/config/src/chainConfig/configs/mainnet.ts"
    if [[ "$(uname)" == "Darwin" && -z "${IN_NIX_SHELL:-}" ]]; then
        gsed -i "s/SECONDS_PER_SLOT: .*/SECONDS_PER_SLOT: $new_value,/g" $preset_mainnet_config_file
    else
        sed -i "s/SECONDS_PER_SLOT: .*/SECONDS_PER_SLOT: $new_value,/g" $preset_mainnet_config_file
    fi
}

echo "Downloading lodestar"

if [ -d "$lodestar_dir" ];
then
  echo "Lodestar seems to be already downloaded. Skipping downloading again"
else
  git clone https://github.com/ChainSafe/lodestar $lodestar_dir
  pushd $lodestar_dir
  git fetch && git checkout $LODESTAR_TAG
  set_slot_time 1
  popd
fi

echo "Building lodestar"
pushd $lodestar_dir
yarn install && yarn run build
popd


echo "Downloading geth"

if [ -d "$geth_dir" ];
then
  echo "Geth seems to be already downloaded. Skipping downloading"
else
  git clone https://github.com/ethereum/go-ethereum.git $geth_dir
  pushd $geth_dir
  git fetch && git checkout $GETH_TAG
  popd
fi

echo "Building Geth"
pushd $geth_dir
GOBIN=$output_bin_dir go install ./cmd/geth
GOBIN=$output_bin_dir go install ./cmd/abigen
popd

