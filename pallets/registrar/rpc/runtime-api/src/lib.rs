#![cfg_attr(not(feature = "std"), no_std)]
//! Runtime API for Registrar pallet
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait RegistrarApi<ParaId> where
        ParaId: parity_scale_codec::Codec,
    {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId>;
    }
}
