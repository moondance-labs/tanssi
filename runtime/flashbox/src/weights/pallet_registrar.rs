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


//! Autogenerated weights for pallet_registrar
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("flashbox_dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_registrar
// --extrinsic
// *
// --chain=flashbox_dev
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// tmp/flashbox_weights/pallet_registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_registrar.
pub trait WeightInfo {
	fn register(x: u32, y: u32, z: u32, ) -> Weight;
	fn deregister_immediate(x: u32, y: u32, ) -> Weight;
	fn deregister_scheduled(x: u32, y: u32, ) -> Weight;
	fn mark_valid_for_collating(y: u32, ) -> Weight;
	fn pause_container_chain(y: u32, ) -> Weight;
	fn unpause_container_chain(y: u32, ) -> Weight;
	fn register_parathread(x: u32, y: u32, z: u32, ) -> Weight;
	fn set_parathread_params(y: u32, ) -> Weight;
}

/// Weights for pallet_registrar using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	/// The range of component `z` is `[1, 10]`.
	fn register(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `389 + y * (12 ±0)`
		//  Estimated: `3833 + y * (12 ±0) + z * (2 ±0)`
		// Minimum execution time: 53_056_000 picoseconds.
		Weight::from_parts(53_982_000, 3833)
			// Standard Error: 13
			.saturating_add(Weight::from_parts(1_085, 0).saturating_mul(x.into()))
			// Standard Error: 4_016_727
			.saturating_add(Weight::from_parts(165_320_642, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 12).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 2).saturating_mul(z.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:0 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorNoting::LatestAuthor` (r:0 w:1)
	/// Proof: `AuthorNoting::LatestAuthor` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	fn deregister_immediate(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236 + y * (18 ±0)`
		//  Estimated: `6196 + y * (16 ±0)`
		// Minimum execution time: 69_417_000 picoseconds.
		Weight::from_parts(73_069_549, 6196)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(10, 0).saturating_mul(x.into()))
			// Standard Error: 18_372
			.saturating_add(Weight::from_parts(593_326, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
			.saturating_add(Weight::from_parts(0, 16).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:0)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:0)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paused` (r:1 w:0)
	/// Proof: `Registrar::Paused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingToRemove` (r:1 w:1)
	/// Proof: `Registrar::PendingToRemove` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	fn deregister_scheduled(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `396 + y * (4 ±0)`
		//  Estimated: `1879 + y * (4 ±0)`
		// Minimum execution time: 36_743_000 picoseconds.
		Weight::from_parts(37_383_553, 1879)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(5, 0).saturating_mul(x.into()))
			// Standard Error: 9_729
			.saturating_add(Weight::from_parts(423_185, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:1 w:0)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// The range of component `y` is `[1, 50]`.
	fn mark_valid_for_collating(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1332 + y * (44 ±0)`
		//  Estimated: `4734 + y * (45 ±0)`
		// Minimum execution time: 84_723_000 picoseconds.
		Weight::from_parts(122_628_245, 4734)
			// Standard Error: 31_708
			.saturating_add(Weight::from_parts(385_221, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 45).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn pause_container_chain(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + y * (8 ±0)`
		//  Estimated: `1912 + y * (8 ±0)`
		// Minimum execution time: 39_080_000 picoseconds.
		Weight::from_parts(58_281_441, 1912)
			// Standard Error: 14_257
			.saturating_add(Weight::from_parts(165_082, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn unpause_container_chain(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + y * (8 ±0)`
		//  Estimated: `1912 + y * (8 ±0)`
		// Minimum execution time: 46_731_000 picoseconds.
		Weight::from_parts(61_539_685, 1912)
			// Standard Error: 15_082
			.saturating_add(Weight::from_parts(100_743, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(y.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	/// The range of component `z` is `[1, 10]`.
	fn register_parathread(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423 + y * (12 ±0)`
		//  Estimated: `3864 + y * (12 ±0) + z * (3 ±0)`
		// Minimum execution time: 55_704_000 picoseconds.
		Weight::from_parts(56_760_000, 3864)
			// Standard Error: 12
			.saturating_add(Weight::from_parts(1_064, 0).saturating_mul(x.into()))
			// Standard Error: 3_941_828
			.saturating_add(Weight::from_parts(161_286_341, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 12).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 3).saturating_mul(z.into()))
	}
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParathreadParams` (r:1 w:1)
	/// Proof: `Registrar::PendingParathreadParams` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn set_parathread_params(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568 + y * (11 ±0)`
		//  Estimated: `4044 + y * (11 ±0)`
		// Minimum execution time: 33_917_000 picoseconds.
		Weight::from_parts(51_607_731, 4044)
			// Standard Error: 13_944
			.saturating_add(Weight::from_parts(429_112, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 11).saturating_mul(y.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	/// The range of component `z` is `[1, 10]`.
	fn register(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `389 + y * (12 ±0)`
		//  Estimated: `3833 + y * (12 ±0) + z * (2 ±0)`
		// Minimum execution time: 53_056_000 picoseconds.
		Weight::from_parts(53_982_000, 3833)
			// Standard Error: 13
			.saturating_add(Weight::from_parts(1_085, 0).saturating_mul(x.into()))
			// Standard Error: 4_016_727
			.saturating_add(Weight::from_parts(165_320_642, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 12).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 2).saturating_mul(z.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:0 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorNoting::LatestAuthor` (r:0 w:1)
	/// Proof: `AuthorNoting::LatestAuthor` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	fn deregister_immediate(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236 + y * (18 ±0)`
		//  Estimated: `6196 + y * (16 ±0)`
		// Minimum execution time: 69_417_000 picoseconds.
		Weight::from_parts(73_069_549, 6196)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(10, 0).saturating_mul(x.into()))
			// Standard Error: 18_372
			.saturating_add(Weight::from_parts(593_326, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
			.saturating_add(Weight::from_parts(0, 16).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:0)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:0)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paused` (r:1 w:0)
	/// Proof: `Registrar::Paused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingToRemove` (r:1 w:1)
	/// Proof: `Registrar::PendingToRemove` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	fn deregister_scheduled(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `396 + y * (4 ±0)`
		//  Estimated: `1879 + y * (4 ±0)`
		// Minimum execution time: 36_743_000 picoseconds.
		Weight::from_parts(37_383_553, 1879)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(5, 0).saturating_mul(x.into()))
			// Standard Error: 9_729
			.saturating_add(Weight::from_parts(423_185, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:1 w:0)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// The range of component `y` is `[1, 50]`.
	fn mark_valid_for_collating(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1332 + y * (44 ±0)`
		//  Estimated: `4734 + y * (45 ±0)`
		// Minimum execution time: 84_723_000 picoseconds.
		Weight::from_parts(122_628_245, 4734)
			// Standard Error: 31_708
			.saturating_add(Weight::from_parts(385_221, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 45).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn pause_container_chain(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + y * (8 ±0)`
		//  Estimated: `1912 + y * (8 ±0)`
		// Minimum execution time: 39_080_000 picoseconds.
		Weight::from_parts(58_281_441, 1912)
			// Standard Error: 14_257
			.saturating_add(Weight::from_parts(165_082, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(y.into()))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn unpause_container_chain(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + y * (8 ±0)`
		//  Estimated: `1912 + y * (8 ±0)`
		// Minimum execution time: 46_731_000 picoseconds.
		Weight::from_parts(61_539_685, 1912)
			// Standard Error: 15_082
			.saturating_add(Weight::from_parts(100_743, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(y.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 3000000]`.
	/// The range of component `y` is `[1, 50]`.
	/// The range of component `z` is `[1, 10]`.
	fn register_parathread(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423 + y * (12 ±0)`
		//  Estimated: `3864 + y * (12 ±0) + z * (3 ±0)`
		// Minimum execution time: 55_704_000 picoseconds.
		Weight::from_parts(56_760_000, 3864)
			// Standard Error: 12
			.saturating_add(Weight::from_parts(1_064, 0).saturating_mul(x.into()))
			// Standard Error: 3_941_828
			.saturating_add(Weight::from_parts(161_286_341, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 12).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 3).saturating_mul(z.into()))
	}
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingParathreadParams` (r:1 w:1)
	/// Proof: `Registrar::PendingParathreadParams` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `y` is `[1, 50]`.
	fn set_parathread_params(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568 + y * (11 ±0)`
		//  Estimated: `4044 + y * (11 ±0)`
		// Minimum execution time: 33_917_000 picoseconds.
		Weight::from_parts(51_607_731, 4044)
			// Standard Error: 13_944
			.saturating_add(Weight::from_parts(429_112, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 11).saturating_mul(y.into()))
	}
}