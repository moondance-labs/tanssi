#!/usr/bin/env bash

# This script can be used for running tanssi's benchmarks.
#
# The tanssi binary is required to be compiled with --features=runtime-benchmarks
# in release mode.

set -euo pipefail

: "${OUTPUT_PATH:=}"
if [[ -z "${OUTPUT_PATH}" ]]; then
    mkdir -p tmp
    OUTPUT_PATH="tmp"
else
    OUTPUT_PATH="${OUTPUT_PATH}"
fi

: "${TEMPLATE_PATH:=}"
if [[ -z "${TEMPLATE_PATH}" ]]; then
    TEMPLATE_PATH="./benchmarking/frame-weight-pallet-template.hbs"
else
    TEMPLATE_PATH="${TEMPLATE_PATH}"
fi

STEPS=50
REPEAT=20

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
    readarray -t options < <(frame-omni-bencher v1 benchmark pallet --no-csv-header --no-storage-info --no-min-squares --no-median-slopes --list --all --runtime="${RUNTIME}" | sed 1d)
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
        frame-omni-bencher v1 benchmark pallet --no-csv-header --no-storage-info --no-min-squares --no-median-slopes --list --all --runtime="${RUNTIME}" |\
            tail -n+2 |\
            cut -d',' -f1 |\
            sort |\
            uniq
        ))
        echo "[+] Benchmarking ${#ALL_PALLETS[@]} pallets"
        printf " - %s\n" "${ALL_PALLETS[@]}"
        for PALLET in "${ALL_PALLETS[@]}"; do
            TEMPLATE_TO_USE=$TEMPLATE_PATH
            OUTPUT="${OUTPUT_PATH}/$PALLET.rs"
            if [[ "$PALLET" == *"pallet_xcm_benchmarks"* ]]; then
                echo "Using pallet xcm benchmarks for $PALLET"
                TEMPLATE_TO_USE="./benchmarking/frame-weight-runtime-template-xcm.hbs"
                MODIFIED_PALLET_FILE=${PALLET/::/_}
                OUTPUT="${OUTPUT_PATH}/$MODIFIED_PALLET_FILE.rs"
            elif [[ "$PALLET" == *"runtime_common"* || "$PALLET" == *"runtime_parachains"* ]]; then
                MODIFIED_PALLET_FILE=${PALLET/::/_}
                OUTPUT="${OUTPUT_PATH}/$MODIFIED_PALLET_FILE.rs"
            fi
            touch "$OUTPUT"
            # Taking arguments from parity setup: https://github.com/moondance-labs/polkadot-sdk/blob/98cefb870b0fe6dbc7fa6d02ffefd1268ff2a5b7/.github/scripts/cmd/cmd.py#L233
            WASMTIME_BACKTRACE_DETAILS=1 frame-omni-bencher v1 benchmark pallet \
            --runtime="${RUNTIME}" \
            --pallet="$PALLET" \
            --extrinsic="*" \
            --wasm-execution=compiled \
            --steps="${STEPS}" \
            --repeat="${REPEAT}" \
            --template="${TEMPLATE_TO_USE}" \
            --output="${OUTPUT}" \
            --heap-pages=4096 \
            --no-storage-info --no-min-squares --no-median-slopes
        done
    else
        TEMPLATE_TO_USE=$TEMPLATE_PATH
        OUTPUT="${OUTPUT_PATH}/${1}.rs"
        if [[ "${1}" == *"pallet_xcm_benchmarks"* ]]; then
            TEMPLATE_TO_USE="./benchmarking/frame-weight-runtime-template-xcm.hbs"
            MODIFIED_PALLET_FILE=${1/::/_}
            OUTPUT="${OUTPUT_PATH}/$MODIFIED_PALLET_FILE.rs"
        elif [[ "${1}" == *"runtime_common"* || "$1" == *"runtime_parachains"* ]]; then
            MODIFIED_PALLET_FILE=${1/::/_}
            OUTPUT="${OUTPUT_PATH}/$MODIFIED_PALLET_FILE.rs"
        fi
        touch "$OUTPUT"
        # Taking arguments from parity setup: https://github.com/moondance-labs/polkadot-sdk/blob/98cefb870b0fe6dbc7fa6d02ffefd1268ff2a5b7/.github/scripts/cmd/cmd.py#L233
        WASMTIME_BACKTRACE_DETAILS=1 frame-omni-bencher v1 benchmark pallet \
            --runtime="${RUNTIME}" \
            --pallet="${1}" \
            --extrinsic="${2}" \
            --wasm-execution=compiled \
            --steps="${STEPS}" \
            --repeat="${REPEAT}" \
            --template="${TEMPLATE_TO_USE}" \
            --output="${OUTPUT}" \
            --heap-pages=4096 \
            --no-storage-info --no-min-squares --no-median-slopes
    fi
}

if [[ "${*}" =~ "--help" ]]; then
    help
else
    CHECK=0
    ALL=0

    filtered_args=()
    for arg in "$@"; do
        if [[ "$arg" == "--check" ]]; then
            CHECK=1
        elif [[ "$arg" == "--all" ]]; then
            ALL=1
        else
            filtered_args+=("$arg")
        fi
    done

    set -- "${filtered_args[@]}"

    if [[ "${ALL}" -eq 1 ]]; then
        mkdir -p weights/
        bench '*' '*' "${CHECK}" "weights/"
    elif [[ $# -ne 2 ]]; then
        choose_and_bench "${CHECK}"
    else
        bench "${1}" "${2}" "${CHECK}"
    fi
fi
