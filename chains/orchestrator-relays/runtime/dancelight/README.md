# Dancelight: v0.1.0

Dancelight is a testnet runtime with no stability guarantees.

## How to build `dancelight` runtime
To build wasm runtime blob with customized epoch duration the following command shall be executed:
```bash
./polkadot/scripts/build-only-wasm.sh dancelight-runtime /path/to/output/directory/
```

## How to run `dancelight-local`

The [Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/) details building, starting, and
testing `dancelight-local` and parachains connecting to it.

## How to register a parachain on the Dancelight testnet

The [parachain registration process](https://docs.substrate.io/tutorials/v3/cumulus/dancelight/) on the public Dancelight
testnet is also outlined.
