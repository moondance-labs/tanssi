#!/usr/bin/env bash

# mkdir just in case as the benchmarking fails if they don't exist
mkdir -p tmp/dancebox_weights tmp/flashbox_weights tmp/simple_template_weights tmp/frontier_template_weights

# Empty directories
rm -rf tmp/*_weights/*

# Dancebox weights
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    OUTPUT_PATH=tmp/dancebox_weights \
    tools/benchmarking.sh "*" "*"

# Flashbox weights
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    CHAIN=flashbox_dev \
    OUTPUT_PATH=tmp/flashbox_weights \
    tools/benchmarking.sh "*" "*"

# Simple template weights
BINARY=target/release/container-chain-template-simple-node \
    TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    OUTPUT_PATH=tmp/simple_template_weights \
    tools/benchmarking.sh "*" "*"

# Frontier template weights
BINARY=target/release/container-chain-template-frontier-node \
    TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs \
    OUTPUT_PATH=tmp/frontier_template_weights \
    tools/benchmarking.sh "*" "*"
