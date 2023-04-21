//! Runtime API for Registrar pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_registrar::ContainerChainGenesisData;
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait RegistrarApi<ParaId> where
        ParaId: parity_scale_codec::Codec,
    {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId>;

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData>;
    }
}
