# Moonwall integration tests for Tanssi

## Setup

Install node and pnpm:

```sh
sudo npm i -g pnpm
pnpm i
```

The expected node version is 20, check the CI workflow file to find the exact version as it can change. For example, this works:

```sh
$ node --version
v20.5.1
$ pnpm --version
8.4.0
```

## Running tests

Before running tests: compile rust binaries

```sh
cargo build --features=fast-runtime --release
```

The "fast-runtime" feature is needed because some tests check session changes, and without this flag 1 session takes 1 hour.

Zombienet tests automatically execute some scripts before running:

* Generate chain spec files
* Download compatible polkadot binary, and store it in tmp/polkadot

Run moonwall TUI interface:

```sh
pnpm moonwall
```

Run tests:

```sh
# manual-seal tests, only orchestrator chain runs, container chains are mocked
pnpm moonwall test dev_tanssi
# zombienet tests, all the chains run
pnpm moonwall test zombie_tanssi
# smoke tests, checks the live stagenet/testnet
pnpm moonwall test dancebox_smoke
# chopsticks upgrade tests, creates a fork of the live network and performs a runtime upgrade
pnpm moonwall test chopsticks_dancebox_upgrade
```

You can find all the test suites in `moonwall.config.json`, or in the interactive moonwall mode when running
`pnpm moonwall`.

You can grep tests by simply appending the pattern after the command:

```sh
# using the exact test id
pnpm moonwall test dev_tanssi DT3301
# or just a prefix
pnpm moonwall test dev_tanssi DT33
```

To allow better debugging, use `run` instead of `test`, which will leave the moonwall environment open after
running the test, allowing you to use polkadot.js to see all the blocks, events, and state:

```sh
pnpm moonwall run zombie_tanssi
```

## Where to find node logs

To see the logs of a failing zombienet node:

```sh
cd /tmp
ls -ltr
# cd into the last zombie folder, that's the most recent zombie network
cd zombie-3aff699b8e6c41a7a0c296f056a750a0_-87975-Ow0nVobAGIPt
# list all the logs
ls *.log
# follow logs
tail -F -n9999 Collator2000-01.log
# nicer interface that allows search
less -R Collator2000-01.log
# or just open it in any other text editor
```

To see the logs of a failing chopsticks test:

```sh
# this is not /tmp, but the tmp folder inside test
cd tmp/node_logs
# find the most recent log file
ls -ltr
# open as usual
```

## Upgrade pnpm packages

To upgrade moonwall or other dependencies:

```sh
pnpm up --latest
```

Remember that everyone else has to run `pnpm i` manually after a package upgrade.
(unlike Rust where cargo handles that automatically)

## Debugging zombienet

You can enable zombienet debug logs to get more information about the commands that are being run:

```
DEBUG=* pnpm moonwall test zombie_tanssi
```

# Typescript-api

When changing some pallet interface or a runtime api, CI will fail if you don't generate a new typescript-api:

```sh
# make sure to compile the node before running the create-local-interfaces command, because it spawns a local node
cargo build --release --features fast-runtime
cd ../typescript-api
pnpm i
pnpm run create-local-interfaces
```

# Debugging with Chopsticks

Chopsticks can be used to re-run live blocks locally. See this guide for a more detailed overview:

<https://docs.moonbeam.network/builders/build/substrate-api/chopsticks/>

The Tanssi Chopsticks config files are in `configs/dancebox.yml` and `configs/stagenet-dancebox.yml`, depending on the network.

For example, to re-run a block:

```sh
pnpm chopsticks run-block --config=./configs/stagenet-dancebox.yml --block 6490 --html --open --runtime-log-level 5
```

You can override the runtime WASM using chopsticks. This is very useful to add some debug logs or asserts.
For example, you can add some logs to a pallet, like this:

```rust
log::info!("state before: {:?}", state);
```

and compile with `--features=force-debug` to get useful debug information instead of `wasm:stripped`:

```sh
cargo build --release --features=force-debug
# Do NOT use --features=fast-runtime
```

Even with `force-debug` some data such as AccountId may not be printed, the workaround is to convert it to hex or call `.encode()` and print the encoded bytes.

Remember to compile the correct runtime version. The one in master will always
be a future version, so it doesn't make sense to use it to replay past blocks.
Check the version of the runtime in polkadot js, and compile from the corresponding branch.
For example, for runtime `dancebox/600`, use the branch `perm-runtime-600`.

To use the new runtime, you can edit the yml file or pass it as a CLI argument, either is fine:

```yml
# configs/dancebox.yml
mock-signature-host: true
db: ./tmp/db_mba.sqlite
wasm-override: "../target/release/wbuild/dancebox-runtime/dancebox_runtime.wasm"
```

Or simply pass it as a CLI argument:

```sh
pnpm chopsticks run-block --config=./configs/dancebox.yml --wasm-override ../target/release/wbuild/dancebox-runtime/dancebox_runtime.wasm --block 1981800 --html --open --runtime-log-level 5
```

### How to find session start?

Sometimes you will need to replay the first block of a new session, because many things happen on session changes.
The easiest way to find out the block number of the last session change is to use a block explorer, such as:

<https://dancebox.subscan.io/event?module=session&event_id=NewSession>

If testing a network with no available block explorer, you can either try to guess by finding the highest multiple of the session length smaller than the current block number (so with session length 600 and block number 10_000, open python and run `10000 // 600 * 600`); or you can add a log somewhere in the runtime that logs the next session start and the session length, and calculate the previous session start from that.

# Spawns Tanssi and container-chains with zombienet
You can directly use the zombieTanssi.json file and pass it to zombienet to spawn yourself the network. From the test directory you can do:

```sh
/path/to/zombienet spawn -p native ./configs/zombieTanssi.json
```
