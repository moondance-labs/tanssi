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
    crate::{Config, Error},
    core::{fmt::Debug, marker::PhantomData},
    frame_system::pallet_prelude::BlockNumberFor,
    parity_scale_codec::FullCodec,
    scale_info::TypeInfo,
    sp_core::U256,
    sp_runtime::traits::{CheckedAdd, CheckedMul, CheckedSub, Get, Zero},
    sp_std::convert::TryInto,
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

/// Error returned by math operations which can overflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverflowError;

impl<T: Config> From<OverflowError> for Error<T> {
    fn from(_: OverflowError) -> Self {
        Error::MathOverflow
    }
}

/// Error returned by math operations which can underflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderflowError;

impl<T: Config> From<UnderflowError> for Error<T> {
    fn from(_: UnderflowError) -> Self {
        Error::MathUnderflow
    }
}

/// Helper to compute ratios by multiplying then dividing by some values, while
/// performing the intermediary computation using a bigger type to avoid
/// overflows.
pub trait MulDiv: Sized {
    /// Multiply self by `a` then divide the result by `b`.
    /// Computation will be performed in a bigger type to avoid overflows.
    /// After the division, will return `None` if the result is to big for
    /// the real type or if `b` is zero.
    fn mul_div(self, a: Self, b: Self) -> Result<Self, OverflowError>;
}

macro_rules! impl_mul_div {
    ($type:ty, $bigger:ty) => {
        impl MulDiv for $type {
            fn mul_div(self, a: Self, b: Self) -> Result<Self, OverflowError> {
                if b.is_zero() {
                    return Err(OverflowError);
                }

                if self.is_zero() {
                    return Ok(<$type>::zero());
                }

                let s: $bigger = self.into();
                let a: $bigger = a.into();
                let b: $bigger = b.into();

                let r: $bigger = s * a / b;

                r.try_into().map_err(|_| OverflowError)
            }
        }
    };
}

impl_mul_div!(u8, u16);
impl_mul_div!(u16, u32);
impl_mul_div!(u32, u64);
impl_mul_div!(u64, u128);
impl_mul_div!(u128, U256);

/// Returns directly an error on overflow.
pub trait ErrAdd: CheckedAdd {
    /// Returns directly an error on overflow.
    fn err_add(&self, v: &Self) -> Result<Self, OverflowError> {
        self.checked_add(v).ok_or(OverflowError)
    }
}

impl<T: CheckedAdd> ErrAdd for T {}

/// Returns directly an error on underflow.
pub trait ErrSub: CheckedSub {
    /// Returns directly an error on underflow.
    fn err_sub(&self, v: &Self) -> Result<Self, UnderflowError> {
        self.checked_sub(v).ok_or(UnderflowError)
    }
}

impl<T: CheckedSub> ErrSub for T {}

/// Returns directly an error on overflow.
pub trait ErrMul: CheckedMul {
    /// Returns directly an error on overflow.
    fn err_mul(&self, v: &Self) -> Result<Self, OverflowError> {
        self.checked_mul(v).ok_or(OverflowError)
    }
}

impl<T: CheckedMul> ErrMul for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_div() {
        assert_eq!(42u128.mul_div(0, 0), Err(OverflowError));
        assert_eq!(42u128.mul_div(1, 0), Err(OverflowError));

        assert_eq!(u128::MAX.mul_div(2, 4), Ok(u128::MAX / 2));
        assert_eq!(u128::MAX.mul_div(2, 2), Ok(u128::MAX));
        assert_eq!(u128::MAX.mul_div(4, 2), Err(OverflowError));
    }
}
