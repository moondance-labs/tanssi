#![cfg_attr(not(feature = "std"), no_std)]
use {
    cumulus_primitives_core::ParaId, parity_scale_codec::Codec, sp_api::NumberFor, sp_std::vec::Vec,
};

sp_api::decl_runtime_apis! {
    /// API necessary for block authorship with aura.
    pub trait TanssiAuthorityAssignmentApi<AuthorityId: Codec> {
        /// Returns the slot duration for Aura.
        ///
        /// Currently, only the value provided by this type at genesis will be used.
        fn para_id_authorities(para_id: ParaId, parent_number: &NumberFor<Block>) -> Option<Vec<AuthorityId>>;

        /// Return the current set of authorities.
        fn check_para_id_assignment(authority: AuthorityId, parent_number: &NumberFor<Block>) -> Option<ParaId>;
    }
}
