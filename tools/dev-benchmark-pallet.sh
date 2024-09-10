#!/usr/bin/env bash

# Helper script to run benchmarks locally when adding a new extrinsic to an existing pallet.
# Runs the benchmark script with `--check` so this is faster than the other benchmarking script,
# because benchmarks must be run on a specific hardware anyway so we don't benefit from having
# accurate benchmarks run on an inaccurate machine. And also automatically copies the weights to
# the expected location.
#
# Usage:
# cargo build --release --features=runtime-benchmarks
# git commit -am 'before running benchmarks"
# ./tools/dev-benchmark-pallet.sh "pallet_registrar"
#
# It is strongly recommended to create a git commit before running this script, because if the benchmarks fail
# for the runtime but not for the pallet, it will leave the weights in an inconsistent state and the runtime will
# not compile, and the easiest way to fix it is to revert the changes.

# Exit on any error
set -e

# Always run the commands from the project dir
cd "$(dirname "$0")/.."

# mkdir just in case as the benchmarking fails if they don't exist
mkdir -p tmp/dancebox_weights tmp/flashbox_weights tmp/simple_template_weights tmp/frontier_template_weights  tmp/starlight_weights

# Empty directories
rm -rf tmp/*_weights/*

export PALLET="${1:-*}"

if [[ $PALLET = "*" ]]; then
	echo "Please provide a pallet name"
	exit
fi

# Pallet weights
TEMPLATE_PATH=benchmarking/frame-weight-pallet-template.hbs \
    OUTPUT_PATH=tmp \
    tools/benchmarking.sh "$PALLET" "*" --check

# Copy "tmp/pallet_registrar.rs" to "pallets/registrar/src/weights.rs"
# Use cargo metadata command to get the pallet folder from the pallet name.
# Need to use a regex to support pallets with underscores properly
export CRATE_NAME_REGEX=$(echo "$PALLET" | sed -E 's/[-_]/[-_]/g')
export CRATE_NAME_REGEX="^$CRATE_NAME_REGEX$"
export CRATE_PATH=$(cargo metadata --no-deps --format-version 1 | \
    jq -r --arg crate_name_regex "$CRATE_NAME_REGEX" '
        .packages[] |
        select(.name | test($crate_name_regex)) |
        .manifest_path |
        gsub("/Cargo.toml$"; "")
    ')
# Real crate name as defined in Cargo.toml (probably using - instead of _)
# This is unused but may be needed
export CRATE_NAME_REAL=$(cargo metadata --no-deps --format-version 1 | \
    jq -r --arg crate_name_regex "$CRATE_NAME_REGEX" '
        .packages[] |
        select(.name | test($crate_name_regex)) |
	.name
    ')

if [ -z $CRATE_PATH ]; then
	echo "Couldn't find pallet folder, you will need to copy the weights manually"
else
	cp -v tmp/$PALLET.rs $CRATE_PATH/src/weights.rs
fi

# Dancebox weights
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    OUTPUT_PATH=tmp/dancebox_weights \
    tools/benchmarking.sh "$PALLET" "*" --check
cp -v tmp/dancebox_weights/$PALLET.rs runtime/dancebox/src/weights/$PALLET.rs

# Flashbox weights
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    CHAIN=flashbox_dev \
    OUTPUT_PATH=tmp/flashbox_weights \
    tools/benchmarking.sh "$PALLET" "*" --check
cp -v tmp/flashbox_weights/$PALLET.rs runtime/flashbox/src/weights/$PALLET.rs

# Starlight weights
BINARY=target/release/tanssi-relay \
    TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    CHAIN=starlight-dev \
    OUTPUT_PATH=tmp/starlight_weights \
    tools/benchmarking.sh "$PALLET" "*" --check

# Probably don't need to add weights to templates, change false to true if the pallet is also included in the templates
if false; then
	# Simple template weights
	BINARY=target/release/container-chain-simple-node \
	    TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
	    OUTPUT_PATH=tmp/simple_template_weights \
	    tools/benchmarking.sh "$PALLET" "*" --check

	# Frontier template weights
	BINARY=target/release/container-chain-frontier-node \
	    TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
	    OUTPUT_PATH=tmp/frontier_template_weights \
	    tools/benchmarking.sh "$PALLET" "*" --check
fi
