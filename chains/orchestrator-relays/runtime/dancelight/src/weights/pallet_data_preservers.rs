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


//! Autogenerated weights for pallet_data_preservers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-01-06, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `192.168.1.114`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dancelight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_data_preservers
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
// tmp/dancelight_weights/pallet_data_preservers.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_data_preservers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_data_preservers::WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn create_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `216`
		//  Estimated: `4151`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_514_351, 4151)
			// Standard Error: 27_613
			.saturating_add(Weight::from_parts(10_213, 0).saturating_mul(x.into()))
			// Standard Error: 582_475
			.saturating_add(Weight::from_parts(295_482, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_create_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `4151`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_354_610, 4151)
			// Standard Error: 2_602
			.saturating_add(Weight::from_parts(202, 0).saturating_mul(x.into()))
			// Standard Error: 54_890
			.saturating_add(Weight::from_parts(44_132, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn update_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `4151`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(36_611_720, 4151)
			// Standard Error: 4_570
			.saturating_add(Weight::from_parts(4_865, 0).saturating_mul(x.into()))
			// Standard Error: 96_417
			.saturating_add(Weight::from_parts(215_140, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_update_profile(x: u32, _y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `4151`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(36_034_836, 4151)
			// Standard Error: 18_909
			.saturating_add(Weight::from_parts(1_191, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	fn delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `4151`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4151)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	fn force_delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `4151`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4151)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ContainerRegistrar::ParaManager` (r:1 w:0)
	/// Proof: `ContainerRegistrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `511`
		//  Estimated: `4151`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4151)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ContainerRegistrar::ParaManager` (r:1 w:0)
	/// Proof: `ContainerRegistrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn stop_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568`
		//  Estimated: `4151`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 4151)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(686), added: 3161, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn force_start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `4151`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 4151)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}