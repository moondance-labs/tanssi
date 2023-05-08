#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::{Codec, Decode, Encode};
use sp_std::vec::Vec;
use cumulus_primitives_core::ParaId;

sp_api::decl_runtime_apis! {
	/// API necessary for block authorship with aura.
	pub trait TanssiAuthorityAssignmentApi<AuthorityId: Codec> {
		/// Returns the slot duration for Aura.
		///
		/// Currently, only the value provided by this type at genesis will be used.
		fn para_id_authorities(para_id: ParaId) -> Option<Vec<AuthorityId>>;

		/// Return the current set of authorities.
		fn check_para_id_assignment(authority: AuthorityId) -> Option<ParaId>;
	}
}