#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

BINARY_FOLDER="../target/release"
RUNTIME="dancelight"

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --bin-dir=*) BINARY_FOLDER="${1#*=}"; shift ;;
        --runtime=*) RUNTIME="${1#*=}"; shift ;;
        --help)
            echo "Usage: $0 [--bin-dir=PATH] [--runtime=dancelight|starlight]"
            exit 0
            ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

if [[ "$RUNTIME" != "dancelight" && "$RUNTIME" != "starlight" ]]; then
    echo "Invalid runtime: $RUNTIME"
    echo "Allowed values are: dancelight, starlight"
    exit 1
fi

mkdir -p specs
# IF and Fallback will be removed after merge in the next PR
if bash ./check-export-chain-spec-cmd.sh $BINARY_FOLDER/tanssi-relay | grep -q "export-chain-spec"; then
  $BINARY_FOLDER/tanssi-relay export-chain-spec --chain "${RUNTIME}-local" > "specs/tanssi-relay.json"
  echo "Spec for $RUNTIME saved to specs/tanssi-relay.json"
  exit 0
fi
$BINARY_FOLDER/tanssi-relay build-spec --chain "${RUNTIME}-local" > "specs/tanssi-relay.json"
echo "Spec for $RUNTIME saved to specs/tanssi-relay.json"
