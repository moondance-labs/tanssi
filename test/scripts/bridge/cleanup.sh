#!/usr/bin/env bash

set -e

scripts_path="$(realpath ./scripts/bridge)"
source $scripts_path/set-env.sh

if [ -z "${1}" ]; then
  echo "No arguments supplied. You can supply: o (removes output dir), ol (removes output and log dir), ole (removes output, log and ethereum_data dir), olep (removes output, log, ethereum_data dir and terminate any leftover processes)"
  exit 1
fi

if [ $1 = "o" ]; then
  rm -rf $output_dir
elif [ $1 = "ol" ]; then
  rm -rf $logs_dir
  rm -rf $output_dir
elif [ $1 = "ole" ]; then
  rm -rf $logs_dir
  rm -rf $output_dir
  rm -rf $ethereum_data_dir
elif [ $1 = "olep" ]; then
  rm -rf $logs_dir
  rm -rf $output_dir
  rm -rf $ethereum_data_dir

  lodestar=""
  geth=""
  beacon_relay=""
  beefy_relay=""

  # Source daemons.pid if it exists
  source $artifacts_dir/daemons.pid 2> /dev/null || true

  # Using interrupt instead of kill signal for process to cleanup
  kill -s INT $lodestar 2> /dev/null || true
  kill -s INT $geth 2> /dev/null || true
  kill -s INT $beacon_relay 2> /dev/null || true
  kill -s INT $beefy_relay 2> /dev/null || true

  # Always remove this to prevent us for terminating any other process for which the PID was reused
  rm $artifacts_dir/daemons.pid 2> /dev/null || true
fi

