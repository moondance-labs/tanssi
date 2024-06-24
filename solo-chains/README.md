# Mozart: v0.1.0

Mozart is a testnet runtime with no stability guarantees.

## How to build `mozart` runtime
`EpochDurationInBlocks` parameter is configurable via `MOZART_EPOCH_DURATION` environment variable. To build wasm
runtime blob with customized epoch duration the following command shall be executed:
```bash
MOZART_EPOCH_DURATION=10 ./polkadot/scripts/build-only-wasm.sh mozart-runtime /path/to/output/directory/
```

## How to run `mozart-local`

The [Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/) details building, starting, and
testing `mozart-local` and parachains connecting to it.

## How to register a parachain on the Mozart testnet

The [parachain registration process](https://docs.substrate.io/tutorials/v3/cumulus/mozart/) on the public Mozart
testnet is also outlined.
