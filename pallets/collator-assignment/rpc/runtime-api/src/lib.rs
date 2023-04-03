#![cfg_attr(not(feature = "std"), no_std)]
//! Runtime API for CollatorAssignment pallet. Can be used by collators to check
//! which parachain will they be collating, as well as the current assignment of
//! collators to parachains and parachains to collators.
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    /// The API to query account nonce (aka transaction index).
    pub trait CollatorAssignmentApi<AccountId, ParaId> where
        AccountId: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
    {
        /// Return the parachain that the given `AccountId` is collating for.
        /// Returns `None` if the `AccountId` is not collating.
        fn current_collator_parachain_assignment(account: AccountId) -> Option<ParaId>;
        /// Return the parachain that the given `AccountId` will be collating for
        /// in the next session change.
        /// Returns `None` if the `AccountId` will not be collating.
        fn future_collator_parachain_assignment(account: AccountId) -> Option<ParaId>;
        /// Return the list of collators of the given `ParaId`.
        /// Returns `None` if the `ParaId` is not in the registrar.
        fn parachain_collators(para_id: ParaId) -> Option<Vec<AccountId>>;
    }
}
