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


//! Autogenerated weights for cumulus_pallet_parachain_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.0.0
//! DATE: 2025-07-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/container-chain-frontier-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// cumulus_pallet_parachain_system
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/frontier_template_weights/cumulus_pallet_parachain_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for cumulus_pallet_parachain_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_parachain_system::WeightInfo for SubstrateWeight<T> {
	/// Storage: `ParachainSystem::LastDmqMqcHead` (r:1 w:1)
	/// Proof: `ParachainSystem::LastDmqMqcHead` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ProcessedDownwardMessages` (r:0 w:1)
	/// Proof: `ParachainSystem::ProcessedDownwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1000)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn enqueue_inbound_downward_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12`
		//  Estimated: `3517`
		// Minimum execution time: 2_632_000 picoseconds.
		Weight::from_parts(2_769_000, 3517)
			// Standard Error: 13_242
			.saturating_add(Weight::from_parts(154_681_510, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
}