<p align="center">
  <img src="media/tanssi.png" width="360">
</p>

**A permissionless appchain infrastructure protocol designed for swift and effortless deployment of application-specific blockchains**

🔎 For more about Tanssi Network, head to our [website](https://www.tanssi.network)<br>
📢 Follow our latest updates on [Twitter](https://twitter.com/TanssiNetwork)<br>
🤝 Engage with fellow developers on our [Discord server](https://discord.com/invite/kuyPhew2KB)<br>

## Build the Tanssi Node

To build Tanssi, you will need a proper Substrate development environment.

If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.dev/docs/en/knowledgebase/getting-started/).

```bash
# Fetch the code
git clone https://github.com/moondance-labs/tanssi
cd tanssi

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Run tests

Tanssi has Rust unit tests as well as typescript integration tests. These tests are run in CI, and can also be run locally. Tanssi tests (specially those in typescript) depend on sessions being shorter, so you probably want to compile the node first as:

```bash
# Build the node with short session times
cargo build --features=fast-runtime --release
```

Then to run the tests:

```bash
# Run the Rust unit tests
cargo test --features=fast-runtime --release
```

Typescript tests are run with [Moonwall](https://github.com/Moonsong-Labs/moonwall). To run these you will need to have pnpm installed:

```bash
# Install moonwall
sudo npm i -g pnpm  

# Install dependencies
pnpm i

# Run manual seal orchestrator tests
pnpm moonwall test dev_tanssi

# Run zombienet tests (with container-chains)
pnpm moonwall test zombie_tanssi
```

Moonwall lets you also run the testing environment wihtout performing any tests on it, as a method for you to manually test certain things:

```bash
# Spin up single manual-seal orchestrator
pnpm moonwall run dev_tanssi

# Spin up orchestrator and two container-chains with zombienet
pnpm moonwall run zombie_tanssi
```

### Sealing options

The command above will start the node in instant seal mode. It creates a block when a transaction arrives, similar to Ganache's auto-mine. You can also choose to author blocks at a regular interval, or control authoring manually through the RPC.

```bash
# Author a block every 6 seconds.
./target/release/tanssi-node --dev --sealing 6000

# Manually control the block authorship and finality
./target/release/tanssi-node --dev --sealing manual
```

### Prefunded Development Addresses

Running Tanssi in development mode will pre-fund several well-known addresses that (mostly) These addresses are derived from
using the well known private key `bottom drive obey lake curtain smoke basket hold race lonely fit walk` and appending the account name as a hard derivation key to the seed above, e.g., `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`:

```
# Alice:
- Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

# Bob:
- Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty

# Charlie:
- Address: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y

# Dave:
- Address: 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy

# Eve:
- Address: 5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw

# Ferdie:
- Address: 5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL

```
## Runtime Architecture

The Tanssi Runtime is built using FRAME and consists of pallets from substrate, frontier, cumulus, and `pallets/`.

From substrate:

- _Balances_: Tracks token balances
- _Sudo_: Allows a privileged account to make arbitrary runtime changes - will be removed before
  launch
- _Timestamp_: On-Chain notion of time
- _Transaction Payment_: Transaction payment (fee) management
- _Authorship_: A pallet where authorship information for orchestrator is stored
- _Invulnerables_: A pallet that selects invulnerable collators to be assigned to author in container-chains and orchestrator
- _Session_: A pallet that handles session-changes and keys
- _AuthorityMapping_: A pallet that handles a mapping between collator accounts and authority keys

From cumulus:

- _ParachainSystem_: A helper to perform relay-storage verifications and injection of cross-chain messages
- _ParachainInfo_: A place to store parachain-relevant constants like parachain id

The following pallets are stored in `pallets/`. They are designed for Tanssi's specific requirements:

- _Registrar_: A pallet that stores all registered container-chains
- _Configuration_: A pallet storing the current configuration from which several other components depend
- _CollatorAssignment_: A pallet implementing collator account to orchestrator/container-chain assignment
- _AuthorityAssignment_: A pallet implementing collator authority key to orchestrator/container-chain assignment
- _Initializer_: A pallet that handles everything that happens on a session-change
- _AuthorNoting_: A pallet that stores the latest author of each of the container-chains

When modifying the git repository for these dependencies, a tool called [diener](https://github.com/bkchr/diener) can be used to replace the git URL and branch for each reference in all `Cargo.toml` files with a single command. This alleviates a lot of the repetitive modifications necessary when changing dependency versions.

## Container-chain templates

Currently two templates are offered within this repository


- __Simple template__: Which ressembles the parachain-template node from cumulus and substrate, and only basic pallet like *pallet-balances*, *parachain-system* and basic configuration.

- __Frontier template__: Which ressembles a moonbeam-alike chain, with all pallets necessary for evm and ethereum compatibility

### Build container-chain nodes (full nodes only, not collators)
These nodes will only act as full nodes, but not as collators since these are offered by Tanssi:

```bash
# Build the simple-template node
cargo build -p container-chain-simple-node --release
```

```bash
# Build the frontier-template node
cargo build -p container-chain-frontier-node --release
```

## Run with Zombienet directly
You can directly use the zombieTanssi.json file and pass it to zombienet to spawn yourself the network. From the test directory you can do:


```bash
# Generates the latest specs for orchestrator and container-chains
npm run build-spec

# Spawns Tanssi and container-chains with zombienet
/path/to/zombienet spawn -p native ./configs/zombieTanssi.json
```