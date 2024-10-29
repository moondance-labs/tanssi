#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

LATEST_RUNTIME_RELEASE=$(curl -s https://api.github.com/repos/moondance-labs/tanssi/releases | jq -r '.[] | select(.name | test("runtime";"i") and (test("starlight";"i")|not)) | .tag_name' | head -n 1 | tr -d '[:blank:]') && [[ ! -z "${LATEST_RUNTIME_RELEASE}" ]]

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

RUNTIME_VER=$(echo $LATEST_RUNTIME_RELEASE | tr -d -c 0-9)

if [ "$RUNTIME_VER" -ge 900 ]; then
    DOCKER_TAG_TANSSI="moondancelabs/tanssi:sha-$LATEST_RT_SHA8-fast-runtime"
else
    DOCKER_TAG_TANSSI="moondancelabs/tanssi:sha-$LATEST_RT_SHA8"
fi

docker rm -f tanssi_container 2> /dev/null | true
docker create --name tanssi_container $DOCKER_TAG_TANSSI bash
docker cp tanssi_container:tanssi/tanssi-node tmp/tanssi-node
docker rm -f tanssi_container
chmod uog+x ../target/release/tanssi-node
chmod uog+x tmp/tanssi-node

if [ "$RUNTIME_VER" -ge 900 ]; then
    DOCKER_TAG_CONTAINER_SIMPLE="moondancelabs/container-chain-simple-template:sha-$LATEST_RT_SHA8-fast-runtime"
else
    DOCKER_TAG_CONTAINER_SIMPLE="moondancelabs/container-chain-simple-template:sha-$LATEST_RT_SHA8"
fi

docker rm -f tanssi_container_simple 2> /dev/null | true
docker create --name tanssi_container_simple $DOCKER_TAG_CONTAINER_SIMPLE bash
if [ "$RUNTIME_VER" -ge 700 ]; then
    docker cp tanssi_container_simple:container-chain-template-simple/container-chain-simple-node tmp/container-chain-simple-node
else
    docker cp tanssi_container_simple:container-chain-template-simple/container-chain-template-simple-node tmp/container-chain-simple-node
fi
docker rm -f tanssi_container_simple
chmod uog+x ../target/release/container-chain-simple-node
chmod uog+x tmp/container-chain-simple-node

if [ "$RUNTIME_VER" -ge 900 ]; then
    DOCKER_TAG_CONTAINER_FRONTIER="moondancelabs/container-chain-evm-template:sha-$LATEST_RT_SHA8-fast-runtime"
else
    DOCKER_TAG_CONTAINER_FRONTIER="moondancelabs/container-chain-evm-template:sha-$LATEST_RT_SHA8"
fi

docker rm -f tanssi_container_frontier 2> /dev/null | true
docker create --name tanssi_container_frontier $DOCKER_TAG_CONTAINER_FRONTIER bash
if [ "$RUNTIME_VER" -ge 700 ]; then
    docker cp tanssi_container_frontier:container-chain-template-evm/container-chain-frontier-node tmp/container-chain-frontier-node
else
    docker cp tanssi_container_frontier:container-chain-template-evm/container-chain-template-frontier-node tmp/container-chain-frontier-node
fi
docker rm -f tanssi_container_frontier
chmod uog+x ../target/release/container-chain-frontier-node
chmod uog+x tmp/container-chain-frontier-node
