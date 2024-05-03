#!/usr/bin/env bash

# This script can be used for running tanssi's benchmarks.
#
# The tanssi binary is required to be compiled with --features=runtime-benchmarks
# in release mode.

set -e

# By default we use the tanssi-node release binary
# However we can use any binary by running the benchmark tool with
# BINARY=./target/release/container-chain-simple-node ./tools/benchmarking.sh
if [[ -z "${BINARY}" ]]; then
    BINARY="./target/release/tanssi-node"
else
    BINARY="${BINARY}"
fi

if [[ -z "${CHAIN}" ]]; then
    CHAIN="dev"
else
    CHAIN="${CHAIN}"
fi

if [[ -z "${OUTPUT_PATH}" ]]; then
    mkdir -p tmp
    OUTPUT_PATH="tmp"
else
    OUTPUT_PATH="${OUTPUT_PATH}"
fi

if [[ -z "${TEMPLATE_PATH}" ]]; then
    TEMPLATE_PATH="./benchmarking/frame-weight-pallet-template.hbs"
else
    TEMPLATE_PATH="${TEMPLATE_PATH}"
fi

STEPS=50
REPEAT=20

if [[ ! -f "${BINARY}" ]]; then
    echo "binary '${BINARY}' does not exist."
    echo "ensure that the tanssi binary is compiled with '--features=runtime-benchmarks' and in production mode."
    exit 1
fi

function help {
    echo "USAGE:"
    echo "  ${0} [<pallet> <benchmark>] [--check]"
    echo ""
    echo "EXAMPLES:"
    echo "  ${0}                       " "list all benchmarks and provide a selection to choose from" 
    echo "  ${0} --check               " "list all benchmarks and provide a selection to choose from, runs in 'check' mode (reduced steps and repetitions)" 
    echo "  ${0} foo \"*\"             " "run all benchmarks for pallet 'foo' (the * must be inside quotes)"
    echo "  ${0} foo bar               " "run a benchmark for pallet 'foo' and benchmark 'bar'" 
    echo "  ${0} foo bar --check       " "run a benchmark for pallet 'foo' and benchmark 'bar' in 'check' mode (reduced steps and repetitions)" 
    echo "  ${0} foo bar --all         " "run a benchmark for all pallets" 
    echo "  ${0} foo bar --all --check " "run a benchmark for all pallets in 'check' mode (reduced steps and repetitions)" 
}

function choose_and_bench {
    readarray -t options < <(${BINARY} benchmark pallet  --chain=${CHAIN} --list | sed 1d)
    options+=('EXIT')

    select opt in "${options[@]}"; do
        IFS=', ' read -ra parts <<< "${opt}"
        [[ "${opt}" == 'EXIT' ]] && exit 0
        
        bench "${parts[0]}" "${parts[1]}" "${1}"
        break
    done
}

function bench {
    OUTPUT="${OUTPUT_PATH}/${1}.rs"
    echo "benchmarking '${1}::${2}' --check=${3}, writing results to '${OUTPUT}'"
    # Check enabled
    if [[ "${3}" -eq 1 ]]; then
        STEPS=16
        REPEAT=1
    fi
    echo "${1}"
    if [[ ${1} == "*" ]] ; then
        # Load all pallet names in an array.
        ALL_PALLETS=($(
        $BINARY benchmark pallet --list --chain="${CHAIN}" |\
            tail -n+2 |\
            cut -d',' -f1 |\
            sort |\
            uniq
        ))
        echo "[+] Benchmarking ${#ALL_PALLETS[@]} pallets"
        for PALLET in "${ALL_PALLETS[@]}"; do
            if [[ "$PALLET" == *"pallet_xcm_benchmarks"* ]]; then
                TEMPLATE_PATH="./benchmarking/frame-weight-runtime-template-xcm.hbs"
            fi
            OUTPUT="${OUTPUT_PATH}/$PALLET.rs"
            WASMTIME_BACKTRACE_DETAILS=1 ${BINARY} benchmark pallet \
            --execution=wasm \
            --wasm-execution=compiled \
            --pallet "$PALLET" \
            --extrinsic "*" \
            --chain="${CHAIN}" \
            --steps "${STEPS}" \
            --repeat "${REPEAT}" \
            --template="${TEMPLATE_PATH}" \
            --json-file raw.json \
            --output "${OUTPUT}"
        done
    else
        if [[ "${1}" == *"pallet_xcm_benchmarks"* ]]; then
            TEMPLATE_PATH="./benchmarking/frame-weight-runtime-template-xcm.hbs"
        fi
        WASMTIME_BACKTRACE_DETAILS=1 ${BINARY} benchmark pallet \
            --execution=wasm \
            --wasm-execution=compiled \
            --pallet "${1}" \
            --extrinsic "${2}" \
            --chain="${CHAIN}" \
            --steps "${STEPS}" \
            --repeat "${REPEAT}" \
            --template="${TEMPLATE_PATH}" \
            --json-file raw.json \
            --output "${OUTPUT}"
    fi
}

if [[ "${@}" =~ "--help" ]]; then
    help
else
    CHECK=0
    if [[ "${@}" =~ "--check" ]]; then
        CHECK=1
        set -o noglob && set -- ${@/'--check'} && set +o noglob
    fi

    ALL=0
    if [[ "${@}" =~ "--all" ]]; then
        ALL=1
    fi

    if [[ "${ALL}" -eq 1 ]]; then
        mkdir -p weights/
        bench '*' '*' "${CHECK}" "weights/"
    elif [[ $# -ne 2 ]]; then
        choose_and_bench "${CHECK}"
    else
        bench "${1}" "${2}" "${CHECK}"
    fi
fi
