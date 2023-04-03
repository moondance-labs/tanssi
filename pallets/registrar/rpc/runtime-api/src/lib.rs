#![cfg_attr(not(feature = "std"), no_std)]
//! Runtime API for CollatorAssignment pallet. Can be used by collators to check
//! which parachain will they be collating, as well as the current assignment of
//! collators to parachains and parachains to collators.
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    /// The API to query account nonce (aka transaction index).
    pub trait RegistrarApi<ParaId> where
        ParaId: parity_scale_codec::Codec,
    {
        /// Return the registered parachain ids
        fn parachains() -> Vec<ParaId>;
    }
}
