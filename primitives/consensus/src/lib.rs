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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

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
