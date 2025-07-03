#!/bin/bash

# Script to write early args to append at the end of the args list. As
# moonwall + zombienet detects `--` marking the start of polkadot args, it performs some
# reordering to add automatic args. This however conflicts with container chain arguments
# being provided before polkadot ones. This script thus should be called by moonwall + zombienet
# which will append polkadot args at the end, and it will call the command with the correct order

# Check if at least 2 arguments are provided: args to add, delimiter
if [ "$#" -lt 3 ]; then
  echo "Usage: $0 <container_args>... -- <command> <tanssi_args>... -- <polkadot_args>..."
  exit 1
fi

# Array to hold the arguments to add
container_args=()
tanssi_args=()
polkadot_args=()

# Container args
while [[ "$#" -gt 0 ]]; do
  if [ "$1" == "--" ]; then
    shift
    break
  fi
  container_args+=("$1")
  shift
done

# Check if there are remaining arguments for the command
if [ "$#" -lt 1 ]; then
  echo "No command provided after the delimiter"
  exit 1
fi

# The command to execute
command="$1"
shift

# Tanssi args
while [[ "$#" -gt 0 ]]; do
  if [ "$1" == "--" ]; then
    shift
    break
  fi
  tanssi_args+=("$1")
  shift
done

# Polkadot args args
while [[ "$#" -gt 0 ]]; do
  if [ "$1" == "--" ]; then
    shift
    break
  fi
  polkadot_args+=("$1")
  shift
done


# Execute the command with the added arguments
$command "$@" "${tanssi_args[@]}" "--" "${polkadot_args[@]}"