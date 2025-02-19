<div align="center">

# Tanssi Development Template 🚀

<img height="180px" alt="Polkadot SDK Logo" src="https://static.wixstatic.com/media/c5e759_e4afdf041c8e4c6daf34c533d795b4a1~mv2.png"/>
</div>

## Table of Contents 📚

- [Tanssi Development Template 🚀](#tanssi-development-template-)
  - [Table of Contents 📚](#table-of-contents-)
  - [1. Setup](#1-setup)
    - [1.1 Ubuntu/Debian](#11-ubuntudebian)
    - [1.2 MacOS](#12-macos)
    - [1.3 Install Rust](#13-install-rust)
    - [1.4 Install Toolchain](#14-install-toolchain)
  - [2. Create a new Pallet](#2-create-a-new-pallet)
    - [2.1 Setup Pallet](#21-setup-pallet)
      - [2.1.1 Create a cargo package](#211-create-a-cargo-package)
      - [2.1.2 Setup `counter-pallet/Cargo.toml`](#212-setup-counter-palletcargotoml)
      - [2.1.3 Build Counter Pallet](#213-build-counter-pallet)
    - [2.2 Implement Counter Pallet](#22-implement-counter-pallet)
      - [2.2.0 Add imports](#220-add-imports)
      - [2.2.1 Add `Config`](#221-add-config)
      - [2.2.2 Add `Event`](#222-add-event)
      - [2.2.3 Add `Error`](#223-add-error)
      - [2.2.4 Add `Storage`](#224-add-storage)
      - [2.2.5 Add `Call`](#225-add-call)
      - [2.2.6 Complete Pallet](#226-complete-pallet)
    - [2.3 Write Tests for Counter Pallet](#23-write-tests-for-counter-pallet)
    - [2.4 Create Benchmarks for Counter Pallet](#24-create-benchmarks-for-counter-pallet)
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
  - [3. Create new Template](#3-create-new-template)
    - [3.1 Copy and Rename Container Chain `Node` and `Runtime`](#31-copy-and-rename-container-chain-node-and-runtime)
    - [3.2 Rename `Cargo.toml` of `Node` and `Runtime`](#32-rename-cargotoml-of-node-and-runtime)
    - [3.3 Rename `Logs` of `Node`](#33-rename-logs-of-node)

## 1. Setup

### 1.1 Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y git clang curl libssl-dev protobuf-compiler make
```

### 1.2 MacOS

```bash
brew install cmake openssl protobuf
```

### 1.3 Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.4 Install Toolchain

```bash
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

## 2. Create a new Pallet

> [!IMPORTANT]
> Let's create a simple pallet counter on which any account can write and read its own number

### 2.1 Setup Pallet

#### 2.1.1 Create a cargo package

```bash
cargo new --lib pallets/counter-pallet
```

#### 2.1.2 Setup `counter-pallet/Cargo.toml`

```toml
[package]
name = "counter-pallet"
authors = ["YOUR NAME", "YOUT@EMAIL.COM"]
description = "Simple pallet counter"
edition = "2021"
license = "GPL-3.0-only"
version = "0.1.0"

[package.metadata.docs.rs]
targets = [ "x86_64-unknown-linux-gnu" ]

[lints]
workspace = true

[dependencies]
frame-support = { workspace = true }
frame-system = { workspace = true }
pallet-session = { workspace = true }
parity-scale-codec = { workspace = true }
scale-info = { workspace = true }
sp-runtime = { workspace = true }
sp-std = { workspace = true }

[dev-dependencies]
sp-core = { workspace = true }
sp-io = { workspace = true }

[features]
default = [ "std" ]
std = [
 "frame-support/std",
 "frame-system/std",
 "pallet-session/std",
 "parity-scale-codec/std",
 "scale-info/std",
 "sp-core/std",
 "sp-io/std",
 "sp-runtime/std",
 "sp-std/std",
]
try-runtime = [
 "frame-support/try-runtime",
 "frame-system/try-runtime",
 "pallet-session/try-runtime",
 "sp-runtime/try-runtime",
]
```

#### 2.1.3 Build Counter Pallet

```bash
# ✨ cargo b -p counter-pallet -r
cargo build --package counter-pallet --release
```

### 2.2 Implement Counter Pallet

> [!IMPORTANT]
> To implement a pallet we just need to write a `Config`, `Event`, `Error`, `Storage` and `Call`.

#### 2.2.0 Add imports

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // rest code here...
}
```

#### 2.2.1 Add `Config`

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type WeightInfo;
}
```

#### 2.2.2 Add `Event`

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    ValueStored(T::AccountId, u32),
    ValueRetrieved(T::AccountId, u32)
}
```

#### 2.2.3 Add `Error`

```rust
#[pallet::error]
pub enum Error<T> {
    NoneValue,
}
```

#### 2.2.4 Add `Storage`

```rust
#[pallet::storage]
pub type Values<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;
```

#### 2.2.5 Add `Call`

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {

    #[pallet::call_index(0)]
    #[pallet::weight(Weight::default())]
    pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
        let account = ensure_signed(origin)?;
        Values::<T>::insert(&account, value);
        Self::deposit_event(Event::ValueStored(account, value));

        Ok(())
    }
    
    #[pallet::call_index(1)]
    #[pallet::weight(Weight::default())]
    pub fn get_value(origin: OriginFor<T>) -> DispatchResult {
        let account = ensure_signed(origin)?;
        let value = Values::<T>::get(&account).ok_or(Error::<T>::NoneValue)?;
        Self::deposit_event(Event::ValueRetrieved(account, value));
    
        Ok(())
    }
}
```

#### 2.2.6 Complete Pallet

```rust
#![cfg_attr(not(feature = "std"), no_std)]

//! # Counter Pallet
//!
//! This pallet allows each account to store and retrieve a value of type `u32`.
//!
//! ## Features
//!
//! - **set_value**: Allows an account to store a new value. Only the signing account can change its own value.
//! - **get_value**: Allows retrieving the stored value. If an account parameter is provided, the value of the specified account will be returned;
//!                  otherwise, the value of the signing account will be returned.
//!
//! ## Events
//!
//! - **ValueStored**: Emitted after storing a value.
//! - **ValueRetrieved**: Emitted after retrieving a value.
//!
//! ## Errors
//!
//! - **NoneValue**: Returned when there is no stored value for the queried account.

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Main structure of the pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);
    
    /// Configuration of the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Type of event emitted by the pallet.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for the pallet's calls.
        type WeightInfo;
    }

    /// Events emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event triggered when a value is stored.
        ///
        /// **Parameters:**
        /// - `T::AccountId`: The account that stored the value.
        /// - `u32`: The stored value.
        ValueStored(T::AccountId, u32),
        /// Event triggered when a value is retrieved.
        ///
        /// **Parameters:**
        /// - `T::AccountId`: The account whose value was retrieved.
        /// - `u32`: The retrieved value.
        ValueRetrieved(T::AccountId, u32)
    }

    /// Possible errors of the pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Error returned when there is no stored value for the queried account.
        NoneValue,
    }

    /// Storage that associates each account (`T::AccountId`) with a value (`u32`).
    #[pallet::storage]
    pub type Values<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    /// Calls (extrinsics) available in the pallet.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Stores a new value for the signing account.
        ///
        /// # Parameters
        ///
        /// - `origin`: The origin of the call, which must be signed.
        /// - `value`: The new value to be stored.
        ///
        /// # Events
        ///
        /// - Emits the `ValueStored` event upon successfully storing the value.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            // Checks if the call is signed and gets the account.
            let account = ensure_signed(origin)?;
            // Stores the value associated with the account.
            Values::<T>::insert(&account, value);
            // Emits the event informing that the value was stored.
            Self::deposit_event(Event::ValueStored(account, value));
            Ok(())
        }
        
        /// Retrieves the stored value associated with an account.
        ///
        /// # Parameters
        ///
        /// - `origin`: The origin of the call. If the `account` parameter is `None`, the signing account will be used.
        /// - `account`: Optional. If provided, the value of the specified account will be retrieved.
        ///
        /// # Behavior
        ///
        /// - If `account` is `Some(account)`, the pallet retrieves the stored value for that account.
        /// - If `account` is `None`, the signing account (obtained by `ensure_signed(origin)`) will be used for the query.
        ///
        /// In both cases, if there is no stored value for the queried account, the call will return the `NoneValue` error.
        ///
        /// # Events
        ///
        /// - Emits the `ValueRetrieved` event after successfully retrieving the value.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn get_value(origin: OriginFor<T>, account: Option<T::AccountId>) -> DispatchResult {
            match account {
                Some(account) => {
                    let value = Values::<T>::get(&account).ok_or(Error::<T>::NoneValue)?;
                    Self::deposit_event(Event::ValueRetrieved(account, value));
                }
                None => {
                    let my_account = ensure_signed(origin)?;
                    let value = Values::<T>::get(&my_account).ok_or(Error::<T>::NoneValue)?;
                    Self::deposit_event(Event::ValueRetrieved(my_account, value));
                }
            };
        
            Ok(())
        }
    }
}
```

### 2.3 Write Tests for Counter Pallet

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

### 2.4 Create Benchmarks for Counter Pallet

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
