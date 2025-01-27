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
  git clone --recurse-submodules https://github.com/moondance-labs/tanssi-bridge-relayer $relayer_root_dir
  pushd $relayer_root_dir
  git fetch && git checkout $RELAYER_BRANCH
  popd
fi

echo "Building Relayer"
pushd $relayer_root_dir
mage build
popd


pushd $test_helpers_dir
pnpm install node-gyp
pnpm install
popd
