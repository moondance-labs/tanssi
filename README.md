# tanssi

## Running with Zombienet

- Download zombienet: 
- Download the polkadot binary from https://github.com/paritytech/polkadot/releases
- Export the path where you downloaded the binary: export PATH=$(pwd):$PATH
- Compile current directory
- Export the target/release path: export PATH=$(pwd)/target/release/:$PATH
- Download zombienet: https://github.com/paritytech/zombienet/releases
- To regenerate the raw specs:
    - container-chain-template-node build-spec --parachain-id 2000 --seeds "Collator2000-01,Collator2000-02" --raw > specs/template-container-2000.json
    - container-chain-template-node build-spec --parachain-id 2001 --seeds "Collator2001-01,Collator2001-02" --raw > specs/template-container-2001.json
- ./zombienet-linux-x64 spawn -p native zombienet_example.toml