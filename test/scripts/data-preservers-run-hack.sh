#!/usr/bin/env bash
# Reorder args and auto-inject container base-path derived from relay/orchestrator.
# Input (from zombienet):
#   <prefix> -- <group B: orchestrator> -- <group A: container> -- <group C: relay>
# Output (exec):
#   <prefix> <group A> -- <group B> -- <group C>
#
# If group A lacks --base-path, we derive it from a --base-path found in:
#   1) group C (relay), else 2) group B (orchestrator), else 3) prefix segment.
# Derived container base = dirname(source_base)/${CONTAINER_BASE_DIR_NAME:-data/containers}

set -euo pipefail

CONTAINER_BASE_DIR_NAME="${CONTAINER_BASE_DIR_NAME:-data/containers}"

err() { echo "Error: $*" >&2; exit 1; }

# --- Parse segments -----------------------------------------------------------
# 1) Collect the <prefix> until the first --
prefix=()
while [[ $# -gt 0 && "${1:-}" != "--" ]]; do
  prefix+=("$1")
  shift
done
[[ "${1:-}" == "--" ]] || err "expected '--' after prefix."
shift

# 2) Read <group B> (orchestrator) until next --
groupB=()
while [[ $# -gt 0 && "${1:-}" != "--" ]]; do
  groupB+=("$1")
  shift
done
[[ "${1:-}" == "--" ]] || err "expected '--' after group B."
shift

# 3) Read <group A> (container) until next --
groupA=()
while [[ $# -gt 0 && "${1:-}" != "--" ]]; do
  groupA+=("$1")
  shift
done
[[ "${1:-}" == "--" ]] || err "expected '--' after group A."
shift

# 4) Everything left is <group C> (relay)
groupC=()
while [[ $# -gt 0 ]]; do
  groupC+=("$1")
  shift
done

# --- Helpers ------------------------------------------------------------------
# Find --base-path VALUE in an array; prints "index:value" on success.
find_base_path_in_array() {
  # $1: array name (as string)
  local array_name="$1"
  local i

  # Use eval to access array indirectly (bash 3.2 compatible)
  eval "local array_len=\${#${array_name}[@]}"

  for (( i=0; i<array_len; i++ )); do
    eval "local current=\${${array_name}[$i]}"
    if [[ "$current" == "--base-path" ]]; then
      local j=$(( i + 1 ))
      if (( j < array_len )); then
        eval "local next=\${${array_name}[$j]}"
        printf "%d:%s\n" "$i" "$next"
        return 0
      fi
    fi
  done
  return 1
}

# Insert two tokens at the beginning of an array
prepend_to_array() {
  # $1: array name, $2: token1, $3: token2
  local array_name="$1"
  local t1="$2" t2="$3"

  # Use eval to modify array indirectly
  eval "$array_name=(\"\$t1\" \"\$t2\" \"\${${array_name}[@]}\")"
}

# --- Determine/Inject container --base-path -----------------------------------
container_bp=""
if out=$(find_base_path_in_array groupA 2>/dev/null); then
  # Container already has a base-path; keep as-is.
  container_bp="${out#*:}"
else
  # Prefer relay (C), then orchestrator (B), then prefix.
  source_bp=""
  if out=$(find_base_path_in_array groupC 2>/dev/null); then
    source_bp="${out#*:}"
  elif out=$(find_base_path_in_array groupB 2>/dev/null); then
    source_bp="${out#*:}"
  elif out=$(find_base_path_in_array prefix 2>/dev/null); then
    source_bp="${out#*:}"
  fi

  if [[ -n "$source_bp" ]]; then
    parent_dir="$(dirname "$source_bp")"
    container_bp="${parent_dir}/${CONTAINER_BASE_DIR_NAME}"
    mkdir -p "$container_bp"
    prepend_to_array groupA "--base-path" "$container_bp"
  else
    # No source base-path found; proceed without injecting.
    :
  fi
fi

# --- Exec reordered command ---------------------------------------------------
# Final call: <prefix> <group A> -- <group B> -- <group C>
exec "${prefix[@]}" "${groupA[@]}" -- "${groupB[@]}" -- "${groupC[@]}"