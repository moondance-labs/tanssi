#!/bin/bash

# Exit on any error
set -e

mkdir -p specs
tmp/tanssi-node build-spec --chain dancebox-local > specs/dancebox-plain-spec.json
pnpm tsx scripts/modify-plain-specs.ts process specs/dancebox-plain-spec.json specs/dancebox-modified-spec.json
tmp/tanssi-node build-spec --chain specs/dancebox-modified-spec.json --raw > specs/dancebox-raw-spec.json