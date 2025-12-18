#!/bin/bash

# Exit on any error
set -e

mkdir -p specs
# IF and Fallback will be removed after merge in the next PR
if bash ./check-export-chain-spec-cmd.sh ../target/release/container-chain-simple-node | grep -q "export-chain-spec"; then
  tmp/tanssi-node export-chain-spec --chain dancebox-local > specs/dancebox-plain-spec.json
  pnpm tsx scripts/modify-plain-specs.ts process specs/dancebox-plain-spec.json specs/dancebox-modified-spec.json
  tmp/tanssi-node export-chain-spec --chain specs/dancebox-modified-spec.json --raw > specs/dancebox-raw-spec.json
  exit 0
fi

tmp/tanssi-node build-spec --chain dancebox-local > specs/dancebox-plain-spec.json
pnpm tsx scripts/modify-plain-specs.ts process specs/dancebox-plain-spec.json specs/dancebox-modified-spec.json
tmp/tanssi-node build-spec --chain specs/dancebox-modified-spec.json --raw > specs/dancebox-raw-spec.json