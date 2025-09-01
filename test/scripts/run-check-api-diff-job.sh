#!/usr/bin/env bash

set -euo pipefail

echo "🔽 Downloading Polkadot binary..."
pnpm tsx scripts/downloadPolkadot.ts

echo "🌐 Getting Westend API data..."
./tmp/polkadot --chain=westend-dev --rpc-port 34100 > westend.log 2>&1 &
WESTEND_PID=$!
echo $WESTEND_PID > westend.pid

sleep 10  # Wait for node to be ready
pnpm get-api-info tmp/westend-api.json --url ws://127.0.0.1:34100

kill $WESTEND_PID
rm westend.pid

echo "🌐 Getting Dancelight API data..."
../target/release/tanssi-relay --chain=dancelight-dev --rpc-port 34100 > dancelight.log 2>&1 &
DANCELIGHT_PID=$!
echo $DANCELIGHT_PID > dancelight.pid

sleep 10  # Wait for node to be ready
pnpm get-api-info tmp/dancelight-api.json --url ws://127.0.0.1:34100

kill $DANCELIGHT_PID
rm dancelight.pid

echo "🔍 Comparing API data..."
pnpm compare-api-info tmp/westend-api.json tmp/dancelight-api.json

echo "✅ Done."