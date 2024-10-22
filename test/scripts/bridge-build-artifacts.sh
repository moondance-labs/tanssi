#!/bin/bash

# Exit on any error
set -e

bridge_scripts=$(realpath ./scripts/bridge)
source $bridge_scripts/set-env.sh

check_tool

$bridge_scripts/build-relayer.sh
$bridge_scripts/build-ethereum-node.sh
