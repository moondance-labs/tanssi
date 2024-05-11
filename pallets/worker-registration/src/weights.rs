#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for .
pub trait WeightInfo {

}

/// Weights for  using the Substrate node and recommended hardware.
pub struct DaosWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for DaosWeight<T> {
}

// For backwards compatibility and tests
impl WeightInfo for () {
}