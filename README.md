# ![Tanssi]

![Tests](https://github.com/PureStake/moonbeam/workflows/Release/badge.svg)

**A Substrate [Parachain](https://polkadot.network/technology/) that offers collation/data-retrievability services to appchains.**

### Sealing options

The command above will start the node in instant seal mode. It creates a block when a transaction arrives, similar to Ganache's auto-mine. You can also choose to author blocks at a regular interval, or control authoring manually through the RPC.

```bash
# Author a block every 6 seconds.
docker run --network="host" purestake/moonbeam:v0.31.0 --dev --sealing 6000

# Manually control the block authorship and finality
docker run --network="host" purestake/moonbeam:v0.31.0 --dev --sealing manual
```

### Prefunded Development Addresses

Running Tanssi in development mode will pre-fund several well-known addresses that (mostly) These addresses are derived from
using the account name as a seed:

```
# Alice:
- Address: 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac

# Bob:
- Address: 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0

# Charleth:
- Address: 0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc

# Dorothy:
- Address: 0x773539d4Ac0e786233D90A233654ccEE26a613D9

# Ethan:
- Address: 0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB

# Faith:
- Address: 0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d

# Goliath:
- Address: 0x7BF369283338E12C90514468aa3868A551AB2929
```

Also, the prefunded default account for testing purposes is:

## Build the Tanssi Node

To build Tanssi, you will need a proper Substrate development environment.

If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.dev/docs/en/knowledgebase/getting-started/).

```bash
# Fetch the code
git clone https://github.com/PureStake/tanssi
cd tanssi

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Run tests

Tanssi has Rust unit tests as well as typescript integration tests. These tests are run in CI, and can also be run locally. Tanssi tests (specially those in typescript) depend on sessions being shorter, so you probably want to compile the node first as:

```bash
# Run the Rust unit tests
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
## Runtime Architecture

The Tanssi Runtime is built using FRAME and consists of pallets from substrate, frontier, cumulus, and `pallets/`.

From substrate:

- _Balances_: Tracks GLMR token balances
- _Sudo_: Allows a privileged account to make arbitrary runtime changes - will be removed before
  launch
- _Timestamp_: On-Chain notion of time
- _Transaction Payment_: Transaction payment (fee) management
- _Authorship_: A pallet where authorship information for orchestrator is stored
- _CollatorSelection_: A pallet that selects collators to be assigned to author in container-chains and orchestrator
- _Session_: A pallet that handles session-changes and keys
- _AuthorityMapping_: A pallet that handles a mapping between collator accounts and authority keys

From cumulus:

- _ParachainSystem_: A helper to perform relay-storage verifications and injection of cross-chain messages
- _ParachainInfo_: A place to store parachain-relevant constants like parachain id

The following pallets are stored in `pallets/`. They are designed for Moonbeam's specific requirements:

- _Registrar_: A pallet that stores all registered container-chains
- _Configuration_: A pallet storing the current configuration from which several other components depend
- _CollatorAssignment_: A pallet implementing collator account to orchestrator/container-chain assignment
- _AuthorityAssignment_: A pallet implementing collator authority key to orchestrator/container-chain assignment
- _Initializer_: A pallet that handles everything that happens on a session-change
- _AuthorNoting_: A pallet that stores the latest author of each of the container-chains

When modifying the git repository for these dependencies, a tool called [diener](https://github.com/bkchr/diener) can be used to replace the git URL and branch for each reference in all `Cargo.toml` files with a single command. This alleviates a lot of the repetitive modifications necessary when changing dependency versions.