#!/bin/bash

CHAINS=(
  dancebox
)

# params
RUNTIME_CHAIN_SPEC=$1

# Bump package version
if [[ $# -gt 0 ]]; then
  echo "Bump package version to 0.$RUNTIME_CHAIN_SPEC.0"
  pnpm version --no-git-tag-version 0.$RUNTIME_CHAIN_SPEC.0
fi

if [[ ! -f ../target/release/tanssi-node ]]; then
  echo "Missing ../target/release/tanssi binary"
  exit 1
fi

# Install dependencies
pnpm install

# Get runtimes metadata
for CHAIN in ${CHAINS[@]}; do
  echo "Starting $CHAIN node"
  ../target/release/tanssi-node --no-hardware-benchmarks --no-telemetry --no-prometheus --alice --tmp --chain=$CHAIN-local --dev-service --wasm-execution=interpreted-i-know-what-i-do --rpc-port=9933 &> /tmp/node-$CHAIN-start.log &
  PID=$!  
  echo "Waiting node..."
  ( tail -f -n0 /tmp/node-$CHAIN-start.log & ) | grep -q 'Running JSON-RPC server'
  echo "Getting $CHAIN metadata"
  curl -s -H "Content-Type: application/json" -d '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' http://localhost:9933 > metadata-$CHAIN.json
  pnpm run load:meta:local
  kill $PID
  sleep 5
done

# Generate typescript api code
echo "Generating typescript api code..."
pnpm run generate:defs
pnpm run generate:meta
pnpm run postgenerate

# Build the package
pnpm run build