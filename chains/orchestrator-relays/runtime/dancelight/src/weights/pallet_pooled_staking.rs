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


//! Autogenerated weights for pallet_pooled_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-12-03, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dancelight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_pooled_staking
// --extrinsic
// *
// --chain=dancelight-dev
// --steps
// 16
// --repeat
// 1
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/dancelight_weights/pallet_pooled_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_pooled_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pooled_staking::WeightInfo for SubstrateWeight<T> {
	/// Storage: `PooledStaking::Pools` (r:12 w:5)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::PendingOperations` (r:1 w:1)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	fn request_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1693`
		//  Estimated: `32046`
		// Minimum execution time: 223_276_000 picoseconds.
		Weight::from_parts(223_276_000, 32046)
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: `PooledStaking::PendingOperations` (r:100 w:100)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::Pools` (r:1000 w:800)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn execute_pending_operations(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537 + b * (390 ±0)`
		//  Estimated: `3593 + b * (25880 ±0)`
		// Minimum execution time: 223_896_000 picoseconds.
		Weight::from_parts(923_209_974, 3593)
			// Standard Error: 7_529_273
			.saturating_add(Weight::from_parts(62_374_026, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((11_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((9_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 25880).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:13 w:9)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::PendingOperations` (r:1 w:1)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	fn request_undelegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `725`
		//  Estimated: `34634`
		// Minimum execution time: 126_682_000 picoseconds.
		Weight::from_parts(126_682_000, 34634)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:300 w:100)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn claim_manual_rewards(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378 + b * (456 ±0)`
		//  Estimated: `6196 + b * (7764 ±0)`
		// Minimum execution time: 70_722_000 picoseconds.
		Weight::from_parts(46_724_962, 6196)
			// Standard Error: 152_961
			.saturating_add(Weight::from_parts(35_593_574, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7764).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:4 w:1)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	fn rebalance_hold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `981`
		//  Estimated: `11342`
		// Minimum execution time: 132_493_000 picoseconds.
		Weight::from_parts(132_493_000, 11342)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:600 w:100)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:100 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 100]`.
	fn update_candidate_position(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432 + b * (551 ±0)`
		//  Estimated: `6287 + b * (15528 ±0)`
		// Minimum execution time: 57_935_000 picoseconds.
		Weight::from_parts(37_904_366, 6287)
			// Standard Error: 220_015
			.saturating_add(Weight::from_parts(30_769_001, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 15528).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:12 w:8)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn swap_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `32046`
		// Minimum execution time: 97_472_000 picoseconds.
		Weight::from_parts(97_472_000, 32046)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:9 w:5)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn distribute_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1531`
		//  Estimated: `24282`
		// Minimum execution time: 156_958_000 picoseconds.
		Weight::from_parts(156_958_000, 24282)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}