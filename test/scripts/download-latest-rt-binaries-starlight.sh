#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

LATEST_RUNTIME_RELEASE=$(curl -s https://api.github.com/repos/moondance-labs/tanssi/releases | jq -r '.[] | select(.name | test("runtime";"i") and (test("starlight";"i"))) | .tag_name' | head -n 1 | tr -d '[:blank:]') && [[ ! -z "${LATEST_RUNTIME_RELEASE}" ]]
ENDPOINT="https://api.github.com/repos/moondance-labs/tanssi/git/refs/tags/$LATEST_RUNTIME_RELEASE"
RESPONSE=$(curl -s -H "Accept: application/vnd.github.v3+json" $ENDPOINT)
TYPE=$(echo $RESPONSE | jq -r '.object.type')
if [[ $TYPE == "commit" ]]
then
    LATEST_RT_SHA8=$(echo $RESPONSE | jq -r '.object.sha' | cut -c -8)
elif [[ $TYPE == "tag" ]]
then
    URL=$(echo $RESPONSE | jq -r '.object.url')
    TAG_RESPONSE=$(curl -s -H "Accept: application/vnd.github.v3+json" $URL)
    TAG_RESPONSE_CLEAN=$(echo $TAG_RESPONSE | tr -d '\000-\037')
    LATEST_RT_SHA8=$(echo $TAG_RESPONSE_CLEAN | jq -r '.object.sha' | cut -c -8)
fi

DOCKER_TAG_STARLIGHT="moondancelabs/starlight:sha-$LATEST_RT_SHA8-fast-runtime"

docker rm -f starlight_container 2> /dev/null | true
docker create --name starlight_container $DOCKER_TAG_STARLIGHT bash
docker cp starlight_container:tanssi-relay/tanssi-relay tmp/tanssi-relay
docker cp starlight_container:tanssi-relay/tanssi-relay-execute-worker tmp/tanssi-relay-execute-worker
docker cp starlight_container:tanssi-relay/tanssi-relay-prepare-worker tmp/tanssi-relay-prepare-worker
docker rm -f starlight_container
chmod uog+x ../target/release/tanssi-relay
chmod uog+x tmp/tanssi-relay