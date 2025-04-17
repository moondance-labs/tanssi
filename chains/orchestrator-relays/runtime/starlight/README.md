# Starlight: v0.1.0

Starlight is a Mainnet runtime.

## How to build `starlight` runtime

To build wasm runtime blob with customized epoch duration the following command shall be executed:

```bash
./polkadot/scripts/build-only-wasm.sh starlight-runtime /path/to/output/directory/
```

## How to run `starlight-local`

The [Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/) details building, starting, and
testing `starlight-local` and parachains connecting to it.

## How to register a parachain on the Starlight Mainnet

The [parachain registration process](https://docs.substrate.io/tutorials/v3/cumulus/starlight/) on the public Starlight
Mainnet is also outlined.
