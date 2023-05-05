//! Runtime API for CollatorAssignment pallet. Can be used by collators to check
//! which parachain will they be collating, as well as the current assignment of
//! collators to parachains and parachains to collators.

#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait CollatorAssignmentApi<AccountId, ParaId, AuthorityId> where
        AccountId: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
        AuthorityId: parity_scale_codec::Codec,
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

        /// Return the parachain that the given `AuthorityId` is collating for.
        /// Returns `None` if the `AuthorityId` is not collating.
        fn current_authority_parachain_assignment(authority: AuthorityId) -> Option<ParaId>;

        /// Return the list of authorities of the given `ParaId`.
        /// Returns `None` if the `ParaId` is not in the registrar.
        fn parachain_authorities(para_id: ParaId) -> Option<Vec<AuthorityId>>;

    }
}
