#!/bin/bash

# Exit on any error
set -e

scripts_path="$(realpath ./scripts/bridge)"

source $scripts_path/set-env.sh

# Can be done independently put relayer binary in output directory

echo "Checkout Snowbridge relayer"

if [ -d "$relayer_root_dir" ];
then
  echo "Relayer seems to be already setup. Skipping git fetch"
else
  git clone https://github.com/Snowfork/snowbridge $relayer_root_dir
  pushd $relayer_root_dir
  git fetch && git checkout $RELAYER_TAG
  popd
fi

$scripts_path/build-eth-contracts.sh

echo "Building Relayer"
pushd $relayer_root_dir
cd relayer && mage build
popd

pushd $test_helpers_dir
pnpm install
popd
