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

#![cfg_attr(not(feature = "std"), no_std)]

use {
    sp_core::U256,
    sp_runtime::traits::{CheckedAdd, CheckedMul, CheckedSub, Zero},
};

/// Error returned by math operations which can overflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverflowError;

/// Error returned by math operations which can underflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderflowError;

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
