// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! # LayerZero Receiver Example Pallet
//!
//! This pallet serves as a **reference implementation** to demonstrate how container chains
//! can receive LayerZero messages forwarded from the relay chain via XCM.
//!
//! ## Overview
//!
//! When a LayerZero message arrives at the relay chain (via the Ethereum bridge), the relay
//! chain's `pallet_lz_router` forwards it to the destination container chain using
//! XCM Transact. This pallet shows how a container chain can receive and process such messages.
//!
//! ## Implementation Requirements
//!
//! To receive LayerZero messages, a container chain pallet needs:
//!
//! 1. **One extrinsic** that accepts an XCM origin from the parent chain (relay)
//! 2. **Accept the message payload** - The relay chain sends the LayerZero message as a
//!    `Vec<u8>` containing the raw bytes from LayerZero
//!
//! The pallet name can be anything - what matters is:
//! - The pallet index and call index are registered in the relay chain's forwarding config
//! - The extrinsic accepts parent (relay) origin via XCM
//! - The extrinsic accepts a `Vec<u8>` payload parameter
//!
//! ## XCM Configuration
//!
//! For this pallet to work, the container chain's XCM config must include:
//! - `pallet_xcm::XcmPassthrough<RuntimeOrigin>` in `XcmOriginToTransactDispatchOrigin`
//! - `AllowExplicitUnpaidExecutionFrom<Equals<ParentLocation>>` in `XcmBarrier`
//!
//! ## Example Usage
//!
//! ```ignore
//! // In your runtime's lib.rs:
//! impl pallet_lz_receiver_example::Config for Runtime {
//!     type ParentOrigin = pallet_xcm::EnsureXcm<Equals<xcm_config::ParentLocation>>;
//! }
//! ```
//!
//! ## Customization
//!
//! This is just an example. In production, you would likely:
//! - Parse the message payload according to your application's protocol
//! - Execute business logic based on the message content
//! - Store state or trigger other pallets based on the message

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use alloc::vec::Vec;
    use frame_support::{pallet_prelude::*, traits::EnsureOrigin};
    use frame_system::pallet_prelude::*;
    use xcm::latest::Location;

    /// Checks that the origin is the parent location (relay chain)
    fn is_parent(location: &Location) -> bool {
        matches!(location.unpack(), (1, []))
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config<RuntimeEvent: From<Event<Self>>> {
        /// Origin that is allowed to send LayerZero messages.
        ///
        /// This should be configured to only allow the parent chain (relay).
        /// Use `pallet_xcm::EnsureXcm<Equals<ParentLocation>>` for this.
        type ParentOrigin: EnsureOrigin<
            <Self as frame_system::Config>::RuntimeOrigin,
            Success = Location,
        >;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A LayerZero message was received from the relay chain.
        MessageReceived {
            /// The raw message payload from LayerZero.
            /// This contains the bytes that were sent from the LayerZero source chain.
            payload: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The origin is not the parent chain (relay).
        NotParentOrigin,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Receive a LayerZero message forwarded from the relay chain.
        ///
        /// This extrinsic is called via XCM Transact from the relay chain's
        /// `pallet_lz_router`. The relay chain encodes the call as
        /// `(pallet_index, call_index, payload)`.
        ///
        /// # Origin
        ///
        /// Must be the parent chain (relay) origin via XCM. This is enforced by
        /// the `ParentOrigin` type configured in the runtime.
        ///
        /// # Parameters
        ///
        /// - `payload`: The raw LayerZero message bytes (`Vec<u8>`). This is the message
        ///   content that was sent from the LayerZero source chain. The container chain
        ///   application should parse and handle this according to its protocol.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn receive_message(origin: OriginFor<T>, payload: Vec<u8>) -> DispatchResult {
            // Ensure the origin is from the parent chain (relay)
            let origin_location = T::ParentOrigin::ensure_origin(origin)?;
            ensure!(is_parent(&origin_location), Error::<T>::NotParentOrigin);

            // In a real implementation, you would parse the payload and execute
            // business logic here. This example just emits an event.
            Self::deposit_event(Event::MessageReceived { payload });

            Ok(())
        }
    }
}
