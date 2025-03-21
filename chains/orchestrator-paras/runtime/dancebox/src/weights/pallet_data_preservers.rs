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
//! DATE: 2025-02-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_data_preservers
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
// tmp/dancebox_weights/pallet_data_preservers.rs

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
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn create_profile(_x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `4177`
		// Minimum execution time: 63_269_000 picoseconds.
		Weight::from_parts(64_906_837, 4177)
			// Standard Error: 4_386
			.saturating_add(Weight::from_parts(59_927, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_create_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `4177`
		// Minimum execution time: 16_093_000 picoseconds.
		Weight::from_parts(16_306_800, 4177)
			// Standard Error: 90
			.saturating_add(Weight::from_parts(1_076, 0).saturating_mul(x.into()))
			// Standard Error: 1_884
			.saturating_add(Weight::from_parts(75_586, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn update_profile(_x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `4177`
		// Minimum execution time: 68_440_000 picoseconds.
		Weight::from_parts(69_883_266, 4177)
			// Standard Error: 4_159
			.saturating_add(Weight::from_parts(67_656, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_update_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `4177`
		// Minimum execution time: 56_243_000 picoseconds.
		Weight::from_parts(57_300_580, 4177)
			// Standard Error: 218
			.saturating_add(Weight::from_parts(1_084, 0).saturating_mul(x.into()))
			// Standard Error: 4_549
			.saturating_add(Weight::from_parts(68_508, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `4177`
		// Minimum execution time: 55_658_000 picoseconds.
		Weight::from_parts(56_853_000, 4177)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn force_delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `4177`
		// Minimum execution time: 55_458_000 picoseconds.
		Weight::from_parts(56_377_000, 4177)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `373`
		//  Estimated: `4177`
		// Minimum execution time: 30_881_000 picoseconds.
		Weight::from_parts(31_804_000, 4177)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn stop_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `4177`
		// Minimum execution time: 34_263_000 picoseconds.
		Weight::from_parts(35_151_000, 4177)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(712), added: 3187, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	fn force_start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `4177`
		// Minimum execution time: 23_333_000 picoseconds.
		Weight::from_parts(23_890_000, 4177)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}