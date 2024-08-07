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


//! Autogenerated weights for pallet_stream_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_stream_payment
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
// tmp/dancebox_weights/pallet_stream_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_stream_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_stream_payment::WeightInfo for SubstrateWeight<T> {
	/// Storage: `StreamPayment::NextStreamId` (r:1 w:1)
	/// Proof: `StreamPayment::NextStreamId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::Streams` (r:0 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn open_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `3604`
		// Minimum execution time: 94_938_000 picoseconds.
		Weight::from_parts(96_429_000, 3604)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn close_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `6196`
		// Minimum execution time: 152_062_000 picoseconds.
		Weight::from_parts(153_944_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn perform_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `6196`
		// Minimum execution time: 88_941_000 picoseconds.
		Weight::from_parts(89_907_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn request_change_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `6196`
		// Minimum execution time: 126_957_000 picoseconds.
		Weight::from_parts(128_607_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_change_delayed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `3729`
		// Minimum execution time: 15_138_000 picoseconds.
		Weight::from_parts(15_458_000, 3729)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn accept_requested_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `654`
		//  Estimated: `6196`
		// Minimum execution time: 118_697_000 picoseconds.
		Weight::from_parts(119_820_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_change_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3767`
		// Minimum execution time: 11_471_000 picoseconds.
		Weight::from_parts(11_826_000, 3767)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn immediately_change_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `6196`
		// Minimum execution time: 118_226_000 picoseconds.
		Weight::from_parts(119_904_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}