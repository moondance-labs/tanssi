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
      - [2.2.6 Complete Counter Pallet](#226-complete-counter-pallet)
      - [2.2.7 Compile Counter Pallet](#227-compile-counter-pallet)
  - [3. Testing a new Pallet](#3-testing-a-new-pallet)
    - [3.1 Setup a Runtime for Testing the Counter Pallet](#31-setup-a-runtime-for-testing-the-counter-pallet)
      - [3.1.1 Add imports](#311-add-imports)
      - [3.1.2 Contruct Runtime macro](#312-contruct-runtime-macro)
      - [3.1.3 Implement Runtime](#313-implement-runtime)
      - [3.1.4 Create a utils functions](#314-create-a-utils-functions)
      - [3.1.5 Complete Mock Runtime](#315-complete-mock-runtime)
      - [3.1.6 Compile mock](#316-compile-mock)
    - [3.2 Write Tests](#32-write-tests)
      - [3.2.1 Add imports](#321-add-imports)
      - [3.2.2 Test if `set_value` works](#322-test-if-set_value-works)
      - [3.2.3 Test if `get_value` works with `none`](#323-test-if-get_value-works-with-none)
      - [3.2.4 Test if `get_value` works with `some`](#324-test-if-get_value-works-with-some)
      - [3.2.5 Test if `get_value` fails](#325-test-if-get_value-fails)
      - [2.2.6 Complete Counter Tests](#226-complete-counter-tests)
      - [3.2.7 Run Tests](#327-run-tests)
  - [4. \[WIP\] Create Benchmarks for Counter Pallet](#4-wip-create-benchmarks-for-counter-pallet)
    - [4.1 Create `benchmarking.rs`](#41-create-benchmarkingrs)
    - [4.2 Update `Cargo.toml`](#42-update-cargotoml)
    - [4.3 Update `lib.rs`](#43-update-librs)
    - [4.4 Run Benchmarks](#44-run-benchmarks)
    - [4.5 Generate Weights](#45-generate-weights)
    - [4.6 Weights Integration](#46-weights-integration)
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

#### 2.2.6 Complete Counter Pallet

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

#### 2.2.7 Compile Counter Pallet

```bash
cargo b -p counter-pallet -r
```

## 3. Testing a new Pallet

In this section, we'll set up the test environment and write the unit tests to check the functionality of your pallet.

### 3.1 Setup a Runtime for Testing the Counter Pallet

> [!IMPORTANT]
> We need to set up an environment that simulates the runtime, so we use `mock.rs`

#### 3.1.1 Add imports

```rust
use crate::pallet as counter_pallet;
use frame_support::construct_runtime;
use frame_support::derive_impl;
use frame_system::mocking::MockBlock;
use sp_runtime::BuildStorage;
```

#### 3.1.2 Contruct Runtime macro

```rust
construct_runtime!(
    pub enum Runtime {
        // ---^^^^^^ This is where `enum Runtime` is defined.
        System: frame_system,
        Counter: counter_pallet,
    }
);
```

#### 3.1.3 Implement Runtime

```rust
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = MockBlock<Runtime>;
    type AccountId = u64;
}

// our simple pallet has nothing to be configured.
impl counter_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
```

#### 3.1.4 Create a utils functions

```rust
/// Auxiliary function to create the test environment with the initial state.
pub fn new_test_ext() -> sp_io::TestExternalities {
    // Creates the initial storage from the default system configuration.
    let storage = frame_system::GenesisConfig::<Runtime>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(storage);
    // Defines the starting block number.
    ext.execute_with(|| {
        frame_system::Pallet::<Runtime>::set_block_number(1);
    });
    ext
}
```

#### 3.1.5 Complete Mock Runtime

```rust
//! # Mock Runtime for Testing
//!
//! This module provides a mock runtime environment for testing the `counter_pallet` in isolation.
//! It simulates the Substrate runtime by defining a `Runtime` enum, configuring necessary pallets,
//! and providing utilities to initialize a test environment.
//!
//! ## Key Components
//! - **`Runtime`**: The mock runtime, combining the `frame_system` and `counter_pallet` pallets.
//! - **`frame_system::Config`**: Configuration for the `frame_system` pallet, using `MockBlock` and `u64` as `AccountId`.
//! - **`counter_pallet::Config`**: Configuration for the `counter_pallet`, specifying `RuntimeEvent` and `WeightInfo`.
//! - **`new_test_ext`**: A helper function to initialize a test environment with a default genesis configuration.
//!
//! ## Usage
//! Use `new_test_ext()` to create a test environment with a clean state and a starting block number of 1.
//! This allows you to test your pallet logic in a controlled and reproducible environment.

use crate::pallet as counter_pallet;
use frame_support::{construct_runtime, derive_impl};
use frame_system::mocking::MockBlock;
use sp_runtime::BuildStorage;

// Define the mock runtime by combining the `frame_system` and `counter_pallet` pallets.
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Counter: counter_pallet,
    }
);

// Implement the `frame_system::Config` trait for the mock runtime.
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = MockBlock<Runtime>;
    type AccountId = u64;
}

// Implement the `counter_pallet::Config` trait for the mock runtime.
impl counter_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

/// Initializes a test environment with a default genesis configuration and sets the block number to 1.
///
/// # Returns
/// - `sp_io::TestExternalities`: A test externalities instance for running tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Runtime>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        frame_system::Pallet::<Runtime>::set_block_number(1);
    });
    ext
}
```

#### 3.1.6 Compile mock

```bash
cargo t -p counter-pallet -r
```

### 3.2 Write Tests

#### 3.2.1 Add imports

```rust
#![cfg(test)]

use crate::mock::{new_test_ext, Runtime, System};
use crate::Error;
use crate::Event as CounterEvent;
use crate::Values;
use frame_support::{assert_noop, assert_ok};
```

#### 3.2.2 Test if `set_value` works

```rust
#[test]
fn set_value_works() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 42.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            42
        ));

        // Checks that the value has been stored correctly.
        assert_eq!(Values::<Runtime>::get(&1), Some(42));

        // Checks if the ValueStored event has been emitted.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueStored(account, value)) =
                record.event
            {
                account == 1 && value == 42
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueStored event not found");
    });
}
```

#### 3.2.3 Test if `get_value` works with `none`

```rust
#[test]
fn get_value_works_with_none() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 55.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            55
        ));

        // Account 1 retrieves its own value by passing `None`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            None
        ));

        // Checks if the ValueRetrieved event was emitted with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 1 && value == 55
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}
```

#### 3.2.4 Test if `get_value` works with `some`

```rust
#[test]
fn get_value_works_with_some() {
    new_test_ext().execute_with(|| {
        // Account 2 stores the value 99.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(2).into(),
            99
        ));

        // Account 1 retrieves the value stored for account 2 by passing `Some(2)`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            Some(2)
        ));

        // Checks that the ValueRetrieved event was sent to account 2 with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 2 && value == 99
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}
```

#### 3.2.5 Test if `get_value` fails

```rust
#[test]
fn get_value_fails_when_no_value_set() {
    new_test_ext().execute_with(|| {
        // Tries to retrieve the value of an account (3) that has never stored a value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(1).into(), Some(3)),
            Error::<Runtime>::NoneValue
        );

        // Tries to retrieve the value of the account itself (4), with no stored value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(4).into(), None),
            Error::<Runtime>::NoneValue
        );
    });
}
```

#### 2.2.6 Complete Counter Tests

```rust
#![cfg(test)]

//! # Tests for Counter Pallet
//!
//! This module contains unit tests for the `counter_pallet`, ensuring its functionality works as expected.
//! The tests cover the following scenarios:
//!
//! - **`set_value`**: Verifies that a value is correctly stored and that the `ValueStored` event is emitted.
//! - **`get_value`**: Tests the retrieval of stored values, both for the caller's account (`None`) and for another account (`Some(account)`).
//! - **Error Handling**: Ensures the `NoneValue` error is returned when attempting to retrieve a value that has not been set.
//!
//! ## Test Cases
//! - `set_value_works`: Tests storing a value and verifies the `ValueStored` event.
//! - `get_value_works_with_none`: Tests retrieving the caller's stored value using `None`.
//! - `get_value_works_with_some`: Tests retrieving another account's stored value using `Some(account)`.
//! - `get_value_fails_when_no_value_set`: Tests error handling when retrieving a value that has not been set.

use crate::mock::{new_test_ext, Runtime, System};
use crate::Error;
use crate::Event as CounterEvent;
use crate::Values;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_works() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 42.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            42
        ));

        // Verify the value is stored correctly.
        assert_eq!(Values::<Runtime>::get(&1), Some(42));

        // Check if the `ValueStored` event was emitted.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueStored(account, value)) =
                record.event
            {
                account == 1 && value == 42
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueStored event not found");
    });
}

#[test]
fn get_value_works_with_none() {
    new_test_ext().execute_with(|| {
        // Account 1 stores the value 55.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(1).into(),
            55
        ));

        // Account 1 retrieves its own value by passing `None`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            None
        ));

        // Check if the `ValueRetrieved` event was emitted with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 1 && value == 55
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}

#[test]
fn get_value_works_with_some() {
    new_test_ext().execute_with(|| {
        // Account 2 stores the value 99.
        assert_ok!(crate::Pallet::<Runtime>::set_value(
            frame_system::RawOrigin::Signed(2).into(),
            99
        ));

        // Account 1 retrieves the value stored for account 2 by passing `Some(2)`.
        assert_ok!(crate::Pallet::<Runtime>::get_value(
            frame_system::RawOrigin::Signed(1).into(),
            Some(2)
        ));

        // Check if the `ValueRetrieved` event was emitted for account 2 with the correct value.
        let event_found = System::events().iter().any(|record| {
            if let crate::mock::RuntimeEvent::Counter(CounterEvent::ValueRetrieved(
                account,
                value,
            )) = record.event
            {
                account == 2 && value == 99
            } else {
                false
            }
        });
        assert!(event_found, "Expected ValueRetrieved event not found");
    });
}

#[test]
fn get_value_fails_when_no_value_set() {
    new_test_ext().execute_with(|| {
        // Attempt to retrieve the value of an account (3) that has never stored a value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(1).into(), Some(3)),
            Error::<Runtime>::NoneValue
        );

        // Attempt to retrieve the value of the caller's account (4), which has no stored value.
        assert_noop!(
            crate::Pallet::<Runtime>::get_value(frame_system::RawOrigin::Signed(4).into(), None),
            Error::<Runtime>::NoneValue
        );
    });
}
```

#### 3.2.7 Run Tests

```bash
cargo t -p counter-pallet -r
```

## 4. [WIP] Create Benchmarks for Counter Pallet

> [!IMPORTANT]
> Benchmarks are essential for determining the extrinsics weights that will be used in the real runtime.

### 4.1 Create `benchmarking.rs`

Create `benchmarks.rs`:

```rust
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    set_value {
        let value = 100u32;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), value)
    verify {
        assert!(Values::<T>::contains_key(&caller));
    }

    get_value_existing {
        let caller: T::AccountId = whitelisted_caller();
        Values::<T>::insert(&caller, 100u32);
    }: _(RawOrigin::Signed(caller.clone()), Some(caller.clone()))
    verify {
        // Verificação implícita pelo sucesso da chamada
    }

    get_value_nonexistent {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), None)
    verify {
        // Verificação implícita pelo tratamento de erro
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::mock::new_test_ext(),
    crate::mock::Runtime,
); 
```

### 4.2 Update `Cargo.toml`

```toml
[dependencies]
frame-benchmarking = { workspace = true, optional = true }

[features]
runtime-benchmarks = [
  "frame-benchmarking",
  "frame-support/runtime-benchmarks",
  "frame-system/runtime-benchmarks",
]
```

### 4.3 Update `lib.rs`

```rust
// ...
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
// ...
```

### 4.4 Run Benchmarks

```bash
cargo test -p counter-pallet --features runtime-benchmarks -r
```

### 4.5 Generate Weights

### 4.6 Weights Integration

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
