#!/usr/bin/env bash

# Usage:
#   script <prefix (binary + non-reordered args)> -- <group B> -- <group A> -- <group C>
# Calls:
#   <prefix> <group A> -- <group B> -- <group C>
#
# Necessary as zombienet automatically fills arguments expecting orchestrator/parachain to be
# the first group and relaychain to be the last.

# 1) Collect the <prefix> until the first --
prefix=()
while [[ $# -gt 0 && "$1" != "--" ]]; do
  prefix+=("$1")
  shift
done
# need the first --
if [[ "$1" != "--" ]]; then
  echo "Error: expected '--' after prefix." >&2
  exit 1
fi
shift

# 2) Read <group B> until next --
groupB=()
while [[ $# -gt 0 && "$1" != "--" ]]; do
  groupB+=("$1")
  shift
done
[[ "$1" == "--" ]] || { echo "Error: expected '--' after group B." >&2; exit 1; }
shift

# 3) Read <group A> until next --
groupA=()
while [[ $# -gt 0 && "$1" != "--" ]]; do
  groupA+=("$1")
  shift
done
[[ "$1" == "--" ]] || { echo "Error: expected '--' after group A." >&2; exit 1; }
shift

# 4) Everything left is <group C>
groupC=("$@")

# 5) Execute in the desired order (note: no leading -- before group A)
exec "${prefix[@]}" "${groupA[@]}" -- "${groupB[@]}" -- "${groupC[@]}"