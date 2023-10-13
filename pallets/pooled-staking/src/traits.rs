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

use {
    core::{fmt::Debug, marker::PhantomData},
    frame_system::pallet_prelude::BlockNumberFor,
    parity_scale_codec::FullCodec,
    scale_info::TypeInfo,
    sp_runtime::traits::{CheckedAdd, Get},
};

/// Allows to get the current instant and check if some duration is elapsed.
pub trait Timer {
    /// Type for the instant. Must implement some traits to be used easily with
    /// the Pooled Staking pallet.
    type Instant: FullCodec + TypeInfo + Clone + Debug + Eq;

    /// Get the current instant.
    fn now() -> Self::Instant;

    /// Check if the timer started at `started` is elapsed.
    fn is_elapsed(start: &Self::Instant) -> bool;

    /// Returns an instant that will make `is_elapsed` true.
    #[cfg(feature = "runtime-benchmarks")]
    fn elapsed_instant() -> Self::Instant;

    /// Skip to a state where `now` will make `is_elapsed` true.
    #[cfg(feature = "runtime-benchmarks")]
    fn skip_to_elapsed();
}

/// A timer using block numbers.
/// `T` is the Runtime type while `G` is a getter for the delay.
pub struct BlockNumberTimer<T, G>(PhantomData<(T, G)>);

impl<T, G> Timer for BlockNumberTimer<T, G>
where
    T: frame_system::Config,
    G: Get<BlockNumberFor<T>>,
{
    type Instant = BlockNumberFor<T>;

    fn now() -> Self::Instant {
        frame_system::Pallet::<T>::block_number()
    }

    fn is_elapsed(start: &Self::Instant) -> bool {
        let delay = G::get();
        let Some(end) = start.checked_add(&delay) else {
            return false;
        };
        end <= Self::now()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn elapsed_instant() -> Self::Instant {
        let delay = G::get();
        Self::now()
            .checked_add(&delay)
            .expect("overflow when computing valid elapsed instant")
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn skip_to_elapsed() {
        frame_system::Pallet::<T>::set_block_number(Self::elapsed_instant());
    }
}

/// Allows knowing if some account is eligible to be a candidate.
pub trait IsCandidateEligible<AccountId> {
    /// Is the provided account id eligible?
    fn is_candidate_eligible(a: &AccountId) -> bool;

    /// Make the provided account id eligible if `eligible` is true, and not
    /// eligible if false.
    #[cfg(feature = "runtime-benchmarks")]
    fn make_candidate_eligible(a: &AccountId, eligible: bool);
}

impl<AccountId> IsCandidateEligible<AccountId> for () {
    fn is_candidate_eligible(_: &AccountId) -> bool {
        true
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn make_candidate_eligible(_: &AccountId, _: bool) {}
}
