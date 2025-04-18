#!/bin/bash

# This script is deprecated, use downloadPolkadot.ts instead

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

# Grab Polkadot version
branch=$(egrep -o '/polkadot-sdk.*#([^\"]*)' ../Cargo.lock | head -1)
polkadot_release=$(echo $branch | sed 's/.*tanssi-polkadot-//' | sed 's/#.*//')

# There is a bug where moonwall saves a html file as an executable, and we try to execute that html file.
# To avoid it, delete any files that are not executables according to "file".
delete_if_not_binary() {
	if [[ -f "$1" ]]; then
		if ! file "$1" | grep -q 'executable'; then
			rm "$1"
		fi
	fi
}

delete_if_not_binary tmp/polkadot
delete_if_not_binary tmp/polkadot-execute-worker
delete_if_not_binary tmp/polkadot-prepare-worker

if [[ -f tmp/polkadot && -f tmp/polkadot-execute-worker && -f tmp/polkadot-prepare-worker ]]; then
    echo "Polkadot binary already found, skipping download ..."
else
	echo "Polkadot binary not found, downloading $polkadot_release"
	pnpm moonwall download polkadot $polkadot_release tmp
	chmod +x tmp/polkadot

	pnpm moonwall download polkadot-execute-worker $polkadot_release tmp
	chmod +x tmp/polkadot-execute-worker

	pnpm moonwall download polkadot-prepare-worker $polkadot_release tmp
	chmod +x tmp/polkadot-prepare-worker
fi
