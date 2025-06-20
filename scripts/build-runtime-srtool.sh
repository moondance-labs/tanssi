
# CARGO_NET_GIT_FETCH_WITH_CLI=true and --entrypoint /srtool/entrypoint.sh
# are required to allow srtool to fetch from github private repositories

# self-hosted runner uses user `maintenance` to match srtool `builder` user 1001
# $(~/srtool/uid-gid-mapping.sh 1001 | xargs) is used to map the user and group

# Docker command to generate JSON blob of the runtime
if [[ $GH_WORKFLOW_MATRIX_CHAIN == *"template"* ]]; then
  FOLDER_NAME=$(echo $GH_WORKFLOW_MATRIX_CHAIN |sed 's/-template.*//')
  RUNTIME_DIR=chains/container-chains/runtime-templates/${FOLDER_NAME}
  PACKAGE=container-chain-template-${FOLDER_NAME}-runtime
elif [[ $GH_WORKFLOW_MATRIX_CHAIN == *"light"* ]]; then
  RUNTIME_DIR=chains/orchestrator-relays/runtime/${GH_WORKFLOW_MATRIX_CHAIN}
  PACKAGE=${GH_WORKFLOW_MATRIX_CHAIN}-runtime
else
  RUNTIME_DIR=chains/orchestrator-paras/runtime/${GH_WORKFLOW_MATRIX_CHAIN}
  PACKAGE=${GH_WORKFLOW_MATRIX_CHAIN}-runtime
fi

# Default mode is build
MODE="${1:-build}"

if [[ "$MODE" == "cleanup" ]]; then
  echo "About to clean the srtools folder"
  docker run --rm -v "${PWD}:/build" alpine \
    sh -c "rm -rf /build/${RUNTIME_DIR}/target"
  exit 0
fi

mkdir -p ${RUNTIME_DIR}/target
chmod uog+rwX ${RUNTIME_DIR}/target

CMD="docker run \
  -i \
  --rm \
  -e CARGO_NET_GIT_FETCH_WITH_CLI=true \
  -e PACKAGE=${PACKAGE} \
  -e RUNTIME_DIR=${RUNTIME_DIR} \
  -e BUILD_OPTS=${RUNTIME_BUILD_OPTS} \
  -e PROFILE=production \
  -v ${PWD}:/build \
  ${GH_WORKFLOW_MATRIX_SRTOOL_IMAGE}:${GH_WORKFLOW_MATRIX_SRTOOL_IMAGE_TAG} \
    build --app --json -cM"

# Here we run the command and stream the output (JSON blob) to a variable
stdbuf -oL $CMD | {
  while IFS= read -r line
  do
      echo ║ $line
      JSON="$line"
  done

  echo "json=$JSON" >> $GITHUB_OUTPUT

  PROP=`echo $JSON | jq -r .runtimes.compact.prop`
  echo "proposal_hash=$PROP" >> $GITHUB_OUTPUT

  WASM=`echo $JSON | jq -r .runtimes.compact.wasm`
  echo "wasm=$WASM" >> $GITHUB_OUTPUT

  Z_WASM=`echo $JSON | jq -r .runtimes.compressed.wasm`
  echo "wasm_compressed=$Z_WASM" >> $GITHUB_OUTPUT

  IPFS=`echo $JSON | jq -r .runtimes.compact.ipfs`
  echo "ipfs=$IPFS" >> $GITHUB_OUTPUT
}

# Clean up file permissions after srtool
podman unshare chown -R 0:0 ${RUNTIME_DIR}/target/srtool