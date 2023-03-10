# moondance

## Running with Zombienet

- Download zombienet: 
- Download the polkadot binary from https://github.com/paritytech/polkadot/releases
- Export the path where you downloaded the binary: export PATH=$(pwd):$PATH
- Compile current directory
- Export the target/release path: export PATH=$(pwd)/target/release/:$PATH
- Download zombienet: https://github.com/paritytech/zombienet/releases
- ./zombienet-linux-x64 spawn -p native zombienet_example.toml