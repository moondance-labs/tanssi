#!/usr/bin/env bash

# Exit on any error
set -e

# Always run the commands from the project dir
cd "$(dirname "$0")/.."

# Check for --parallel flag
PARALLEL=false
if [[ "$1" == "--parallel" ]]; then
    PARALLEL=true
fi

# mkdir just in case as the benchmarking fails if they don't exist
mkdir -p tmp/dancebox_weights tmp/flashbox_weights tmp/simple_template_weights tmp/frontier_template_weights tmp/dancelight_weights tmp/starlight_weights

# Empty directories
rm -rf tmp/*_weights/*

# Define benchmark commands
declare -a BENCHMARK_CMDS=(
  "TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=tmp/dancebox_weights tools/benchmarking.sh '*' '*'"
  "TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs CHAIN=flashbox_dev OUTPUT_PATH=tmp/flashbox_weights tools/benchmarking.sh '*' '*'"
  "BINARY=target/release/container-chain-simple-node TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=tmp/simple_template_weights tools/benchmarking.sh '*' '*'"
  "BINARY=target/release/container-chain-frontier-node TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=tmp/frontier_template_weights tools/benchmarking.sh '*' '*'"
  "BINARY=target/release/tanssi-relay TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs CHAIN=dancelight-dev OUTPUT_PATH=tmp/dancelight_weights tools/benchmarking.sh '*' '*'"
  "BINARY=target/release/tanssi-relay TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs CHAIN=starlight-dev OUTPUT_PATH=tmp/starlight_weights tools/benchmarking.sh '*' '*'"
)

echo "Running benchmarks. Option parallel: ${$PARALLEL}"

if $PARALLEL; then
    for cmd in "${BENCHMARK_CMDS[@]}"; do
        bash -c "$cmd" &
    done
    wait
else
    for cmd in "${BENCHMARK_CMDS[@]}"; do
        bash -c "$cmd"
    done
fi