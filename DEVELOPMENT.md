<div align="center">

# Tanssi Development Template 🚀

<img height="180px" alt="Polkadot SDK Logo" src="https://static.wixstatic.com/media/c5e759_e4afdf041c8e4c6daf34c533d795b4a1~mv2.png"/>
</div>

## Table of Contents 📚

- [Tanssi Development Template 🚀](#tanssi-development-template-)
  - [Table of Contents 📚](#table-of-contents-)
  - [1. Project Roadmap](#1-project-roadmap)
    - [1.1 Project Structure](#11-project-structure)
  - [2. Setup](#2-setup)
    - [2.1 Ubuntu/Debian](#21-ubuntudebian)
    - [2.2 MacOS](#22-macos)
    - [2.3 Rust Toolchain](#23-rust-toolchain)
  - [3. Create new Template](#3-create-new-template)
    - [3.1 Copy and Rename Container Chain `Node` and `Runtime`](#31-copy-and-rename-container-chain-node-and-runtime)
    - [3.2 Rename `Cargo.toml` of `Node` and `Runtime`](#32-rename-cargotoml-of-node-and-runtime)
    - [3.3 Rename `Logs` of `Node`](#33-rename-logs-of-node)
  - [4. Create a new Pallet](#4-create-a-new-pallet)
    - [4.1 Copy from Minimal Pallet Template](#41-copy-from-minimal-pallet-template)
    - [4.2 Implement Counter Pallet](#42-implement-counter-pallet)
      - [4.2.1 Add `Config`](#421-add-config)
      - [4.2.2 Add `Event`](#422-add-event)
      - [4.2.3 Add `Error`](#423-add-error)
      - [4.2.4 Add `Storage`](#424-add-storage)
      - [4.2.5 Add `Call`](#425-add-call)
    - [4.3 Write Tests for Counter Pallet](#43-write-tests-for-counter-pallet)
    - [4.4 Create Benchmarks for Counter Pallet](#44-create-benchmarks-for-counter-pallet)
  - [5. Add Pallet into Runtime](#5-add-pallet-into-runtime)
    - [5.0 Add Pallet in `Cargo.toml`](#50-add-pallet-in-cargotoml)
    - [5.1 Import Pallet](#51-import-pallet)
    - [5.2 Type Pallet](#52-type-pallet)
    - [5.3 Use `parameter_types!`](#53-use-parameter_types)
    - [5.4 Add Pallet in `construct_runtime!`](#54-add-pallet-in-construct_runtime)
    - [5.5 Build Runtime](#55-build-runtime)
  - [6. Incorporate Runtime in Node](#6-incorporate-runtime-in-node)
  - [7. Validate new Features in Running Node](#7-validate-new-features-in-running-node)
  - [8. Update pallet](#8-update-pallet)
  - [9. Integrate new Pallet in Runtime](#9-integrate-new-pallet-in-runtime)
  - [10. Upgrade Node Network (new runtime)](#10-upgrade-node-network-new-runtime)
  - [11. Validate new feature](#11-validate-new-feature)

## 1. Project Roadmap

- [ ] setup
- [ ] clone node, runtime and rename
- [ ] create a pallet
- [ ] test pallet
- [ ] benchmark pallet
- [ ] integrate pallet in runtime
- [ ] incorporate runtime in node
- [ ] initiate node
- [ ] validate new features in running node
- [ ] update pallet
- [ ] integrate pallet in runtime
- [ ] upgrade node network (new runtime)
- [ ] validate new feature

### 1.1 Project Structure

```./chains/container-chains/
├── nodes               # Node configuration and CLI
│   ├── ...
└── runtime-templates   # Runtime configuration
    ├── ...
├── ...                 # Other things
└── pallets/            # Custom blockchain logic modules
```

## 2. Setup

### 2.1 Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y git clang curl libssl-dev protobuf-compiler make
```

### 2.2 MacOS

```bash
brew install cmake openssl protobuf
```

### 2.3 Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

## 3. Create new Template

### 3.1 Copy and Rename Container Chain `Node` and `Runtime`

### 3.2 Rename `Cargo.toml` of `Node` and `Runtime`

For node:
```bash
sed -i 's/container-chain-template-simple/container-chain-template-my-chain/g' chains/container-chains/nodes/my-chain/Cargo.toml
```

For runtime: 
```bash
sed -i 's/container-chain-template-simple/container-chain-template-my-chain/g' chains/container-chains/runtime-templates/my-chain/Cargo.toml
```

### 3.3 Rename `Logs` of `Node`

```bash
sed -i 's/Simple Container Chain/My Chain/g' chains/container-chains/nodes/my-chain/src/command.rs
```

## 4. Create a new Pallet

### 4.1 Copy from Minimal Pallet Template

```bash
cp -r pallets/minimal-pallet-template pallets/my-pallet
```

### 4.2 Implement Counter Pallet

#### 4.2.1 Add `Config`
```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type WeightInfo;
}
```

#### 4.2.2 Add `Event`
```rust
#[pallet::event]
pub enum Event<T: Config> {
    ValueStored(u32, T::AccountId),
    ValueRetrieved(u32)
}
```

#### 4.2.3 Add `Error`
```rust
#[pallet::error]
pub enum Error<T> {
    StorageOverflow,
    NoneValue
}
```

#### 4.2.4 Add `Storage`
```rust
#[pallet::storage]
pub type Value<T> = StorageValue<_, u32>;
```

#### 4.2.5 Add `Call`
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
        let who = ensure_signed(origin)?;
        Value::<T>::put(value);
        Self::deposit_event(Event::ValueStored(value, who));
        Ok(())
    }
}
```

### 4.3 Write Tests for Counter Pallet

Create `tests.rs`:
```rust
#[test]
fn set_value_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplatePallet::set_value(Origin::signed(1), 42));
        assert_eq!(TemplatePallet::value(), Some(42));
    });
}
```

### 4.4 Create Benchmarks for Counter Pallet

Create `benchmarks.rs`:
```rust
#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_value() {
        let value = 100u32;
        #[extrinsic_call]
        _(Origin::signed(caller), value);
    }
}
```

## 5. Add Pallet into Runtime

### 5.0 Add Pallet in `Cargo.toml`

```toml
[dependencies]
pallet-my-pallet = { path = "../../pallets/my-pallet", default-features = false }
```

### 5.1 Import Pallet

In `lib.rs`:
```rust
pub use pallet_my_pallet;
```

### 5.2 Type Pallet 

```rust
impl pallet_my_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
```

### 5.3 Use `parameter_types!`

```rust
parameter_types! {
    pub const MaxValue: u32 = 1000;
}
```

### 5.4 Add Pallet in `construct_runtime!`

```rust
construct_runtime!(
    pub enum Runtime where
    {
        // ...
        MyPallet: pallet_my_pallet,
    }
);
```

### 5.5 Build Runtime

```bash
cargo build --release -p container-chain-template-my-chain-runtime
```

## 6. Incorporate Runtime in Node

Update node's `chain_spec.rs` to include new runtime components.

## 7. Validate new Features in Running Node

Start node and test via CLI:
```bash
./target/release/container-chain-template-my-chain --dev
```

## 8. Update pallet

Follow steps 4.2-4.4 to modify pallet logic and test changes.

## 9. Integrate new Pallet in Runtime

Repeat steps 5.0-5.5 with updated pallet version.

## 10. Upgrade Node Network (new runtime)

Build updated runtime WASM and submit upgrade proposal:
```bash
cargo build --release -p container-chain-template-my-chain-runtime
```

## 11. Validate new feature

Test upgraded functionality via runtime API and chain state queries.
