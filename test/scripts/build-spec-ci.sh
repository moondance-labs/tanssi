#!/bin/bash

# Exit on any error
set -e

# Always run the commands from the "test" dir
cd $(dirname $0)/..

mkdir -p specs
./tmp/container-chain-template-simple-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9" --parachain-id 2000 --seeds "Collator2000-01,Collator2000-02" --raw > specs/template-container-2000.json
./tmp/container-chain-template-frontier-node build-spec --disable-default-bootnode --add-bootnode "/ip4/127.0.0.1/tcp/33050/ws/p2p/12D3KooWFGaw1rxB6MSuN3ucuBm7hMq5pBFJbEoqTyth4cG483Cc" --parachain-id 2001 --seeds "Collator2001-01,Collator2001-02" --raw > specs/template-container-2001.json
./tmp/container-chain-template-simple-node build-spec --disable-default-bootnode --parachain-id 2002 --seeds "Collator2002-01,Collator2002-02" --raw > specs/template-container-2002.json
./tmp/tanssi-node build-spec --chain dancebox-local --parachain-id 1000 --add-container-chain specs/template-container-2000.json --add-container-chain specs/template-container-2001.json > specs/tanssi-1000.json
