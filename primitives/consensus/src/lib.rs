#![cfg_attr(not(feature = "std"), no_std)]
use {cumulus_primitives_core::ParaId, parity_scale_codec::Codec, sp_std::vec::Vec};

sp_api::decl_runtime_apis! {
    /// API necessary for block authorship with Tanssi.
    pub trait TanssiAuthorityAssignmentApi<AuthorityId: Codec> {
        /// Returns the authorities for a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<AuthorityId>>;

        /// Returns the paraId for which an authority is assigned (if any)
        fn check_para_id_assignment(authority: AuthorityId) -> Option<ParaId>;
    }
}
