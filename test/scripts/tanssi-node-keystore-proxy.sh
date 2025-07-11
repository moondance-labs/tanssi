#!/usr/bin/env bash
set -euo pipefail

#
# tanssi-node-keystore-proxy.sh
#
# Ensures container (Tanssi) args and relay-chain (Polkadot) args
# are in the right order, and allows:
#   • early flags like --set-relay-arg=<arg>
#   • switching which keystore-path is overridden
#   • duplicate relay-arg suppression
#

#─── globals ────────────────────────────────────────────────────────────────────
declare -a OVERRIDE_POLKADOT_ARGS=()     # e.g. “--rpc-port=9962”
declare -a TANSSI_ARGS=()       # “command + tanssi-node flags”
declare -a POLKADOT_ARGS=()     # relay-chain flags
change_relay_keystore=false     # whether to override relay keystore
relay_chain_id="dancelight_local_testnet"

#─── usage helper ────────────────────────────────────────────────────────────────
print_usage() {
  cat <<EOF
Usage: $0 [--set-relay-arg=<arg>] [--change-relay-keystore-path] \
-- <command> <tanssi_args>... -- <polkadot_args>...

  --set-relay-arg=--KEY=VAL        Append or override --KEY=VAL in the relay args
  --change-relay-keystore-path
                             Override the relay-chain’s keystore path with a custom one for testing
EOF
  exit 1
}

#─── parse_script_flags ──────────────────────────────────────────────────────────
# Strips out any --set-relay-arg / --change-relay-keystore-path
# *from the global* arguments and stores the rest in REMAIN_ARGS.
parse_script_flags() {
  local args=( "$@" )
  OVERRIDE_POLKADOT_ARGS=()
  change_relay_keystore=false

  # pull off script-only flags
  while [[ ${#args[@]} -gt 0 && "${args[0]}" != -- ]]; do
    case "${args[0]}" in
      --set-relay-arg=*)
        OVERRIDE_POLKADOT_ARGS+=( "${args[0]#*=}" ) ;;
      --change-relay-keystore-path)
        change_relay_keystore=true ;;
      *)
        echo "Error: unknown flag ${args[0]}" >&2
        print_usage ;;
    esac
    args=( "${args[@]:1}" )
  done

  # must now see the “--” delimiter
  if [[ ${#args[@]} -eq 0 || "${args[0]}" != -- ]]; then
    print_usage
  fi

  # drop that “--” and stash the rest for main()
  REMAIN_ARGS=( "${args[@]:1}" )
}

#─── split_command_sections ─────────────────────────────────────────────────────
# Splits the remaining arguments into:
#   • TANSSI_ARGS = command + all flags up to the next “--”
#   • POLKADOT_ARGS = everything after that “--”
split_command_sections() {
  # the very first token is the command to exec
  TANSSI_ARGS=( "$1" )
  shift

  # gather tanssi-node flags until “--”
  while [[ $# -gt 0 && $1 != -- ]]; do
    TANSSI_ARGS+=( "$1" )
    shift
  done

  # drop the “--” if present
  [[ $# -gt 0 && $1 == -- ]] && shift

  # the rest are relay-chain flags
  POLKADOT_ARGS=( "$@" )
}

#─── get_arg_value ───────────────────────────────────────────────────────────────
# Given a key and a named array, returns the element immediately after key.
get_arg_value() {
  local key=$1; shift
  local -n arr=$1
  for ((i=0; i<${#arr[@]}; i++)); do
    if [[ "${arr[i]}" == "$key" ]]; then
      echo "${arr[i+1]}"
      return 0
    fi
  done
  return 1
}

#─── compute_relay_keystore_path ─────────────────────────────────────────────────
# Reads --base-path from POLKADOT_ARGS and builds:
#   <base>/chains/<relay_chain_id>/keystore
compute_relay_keystore_path() {
  local base
  base="$(get_arg_value --base-path POLKADOT_ARGS)" || {
    echo "Error: missing relay --base-path" >&2
    exit 1
  }
  echo "$base/tmp_keystore_zombie_test"
}

#─── override_keystore ───────────────────────────────────────────────────────────
# Overrides any existing --keystore-path in the given array by using
# override_relay_args (which strips duplicates and appends new flags).
#
# Arguments:
#   1) Name of array to modify (e.g. TANSSI_ARGS or POLKADOT_ARGS)
#   2) New keystore-path
override_keystore() {
  local array_name=$1
  local keystore="$2"

  # Create a nameref to the target array
  local -n arr=$array_name

  # 1) Stash any existing relay-overrides
  local old_overrides=( "${OVERRIDE_POLKADOT_ARGS[@]}" )

  # 2) Set up just our keystore override
  OVERRIDE_POLKADOT_ARGS=( "--keystore-path=$keystore" )

  # 3) Delegate to override_relay_args, which will strip any existing
  #    --keystore-path (both forms) and append our new one to the end
  if ! override_relay_args "$array_name"; then
    echo "Error: failed to apply keystore override" >&2
    # restore before bailing
    OVERRIDE_POLKADOT_ARGS=( "${old_overrides[@]}" )
    return 1
  fi

  # 4) Restore the original overrides list
  OVERRIDE_POLKADOT_ARGS=( "${old_overrides[@]}" )
}

#─── override_relay_args ─────────────────────────────────────────────────────────
# Adds --set-relay-arg overrides into the named array.
# Very verbose: will trace every step and stop on any error.
override_relay_args() {
  # turn on xtrace for this function, for debugging
  #set -x

  # trap failures locally
  trap 'echo "✖ [override_relay_args] ERROR at line $LINENO: \"$BASH_COMMAND\" (exit $?)" >&2' ERR

  # nameref to the target array
  local -n arr=$1 || { echo "✖ failed to nameref array '$1'" >&2; return 1; }

  # how many overrides do we have?
  local n_overrides=${#OVERRIDE_POLKADOT_ARGS[@]}
  echo "override_relay_args: OVERRIDE_POLKODOT_ARGS has $n_overrides entries" >&2

  if (( n_overrides == 0 )); then
    echo "override_relay_args: nothing to do" >&2
    set +x; trap - ERR
    return 0
  fi

  # initial state
  for override in "${OVERRIDE_POLKADOT_ARGS[@]}"; do
    echo "--> processing override: '$override'" >&2
    local key="${override%%=*}"
    echo "    key = '$key'" >&2

    local tmp=()
    local i=0
    local matched=false
    local arr_len=${#arr[@]}

    # scan & strip
    while (( i < arr_len )); do
      local elem="${arr[i]}"
      echo "    scanning arr[$i] = '$elem'" >&2

      if [[ "$elem" == "$key="* ]]; then
        matched=true
        echo "      strip key=value form: '$elem'" >&2
        (( ++i ))    # use pre-increment to return non-zero

      elif [[ "$elem" == "$key" ]]; then
        matched=true
        local next="${arr[i+1]:-<MISSING>}"
        echo "      strip bare key+value: '$elem' + '$next'" >&2
        (( i+=2 )) || :   # swallow any zero exit-code

      else
        tmp+=( "$elem" )
        (( ++i ))    # use pre-increment
      fi
    done

    # warn if nothing was removed
    if ! $matched; then
      echo "    no existing '$key' entries found" >&2
    fi

    # append the new override
    tmp+=( "$override" )
    echo "    after append, tmp = [${tmp[*]}]" >&2

    # write back to arr
    arr=( "${tmp[@]}" )
    echo "    arr now = [${arr[*]}]" >&2
  done

  # turn off xtrace and trap
  set +x
  trap - ERR

  return 0
}

#─── debug_print ─────────────────────────────────────────────────────────────────
debug_print() {
  echo "DEBUG:"
  echo "  OVERRIDE_POLKADOT_ARGS:   ${OVERRIDE_POLKADOT_ARGS[*]}"
  echo "  TANSSI_ARGS:     ${TANSSI_ARGS[*]}"
  echo "  POLKADOT_ARGS:   ${POLKADOT_ARGS[*]}"
  echo "  change_relay_keystore: $change_relay_keystore"
  echo
}

#─── main flow ─────────────────────────────────────────────────────────────────
main() {
  # 1) peel off our script-only flags into OVERRIDE_POLKADOT_ARGS/change_relay_keystore
  parse_script_flags "$@"

  # 2) replace global $@ with only the *remaining* args
  set -- "${REMAIN_ARGS[@]}"

  # 3) now safely split into TANSSI_ARGS vs POLKADOT_ARGS
  split_command_sections "$@"

  debug_print

  if [[ "$change_relay_keystore" == true ]]; then
    local relay_keystore
    relay_keystore="$(compute_relay_keystore_path)"
    override_keystore POLKADOT_ARGS "$relay_keystore"
  fi

  override_relay_args POLKADOT_ARGS

  debug_print

  exec "${TANSSI_ARGS[@]}" -- "${POLKADOT_ARGS[@]}"
}

main "$@"