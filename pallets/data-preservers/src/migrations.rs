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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

use {
    super::*,
    tp_traits::{apply, derive_scale_codec, derive_storage_traits},
};

/// Old profile structure.
/// Keep it until migration is removed.
#[apply(derive_scale_codec)]
#[derive(RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct OldProfile<T: Config> {
    pub url: BoundedVec<u8, T::MaxStringLen>,
    pub para_ids: ParaIdsFilter<T>,
    pub mode: ProfileMode,
    pub assignment_request: ProviderRequestOf<T>,
}

#[apply(derive_storage_traits)]
#[derive(MaxEncodedLen, DecodeWithMemTracking)]
pub enum ProfileMode {
    Bootnode,
    Rpc { supports_ethereum_rpcs: bool },
}

/// Profile with additional data:
/// - the account id which created (and manage) the profile
/// - the amount deposited to register the profile
#[apply(derive_scale_codec)]
#[derive(RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct OldRegisteredProfile<T: Config> {
    pub account: T::AccountId,
    pub deposit: BalanceOf<T>,
    pub profile: OldProfile<T>,
    /// There can be at most 1 assignment per profile.
    pub assignment: Option<(ParaId, AssignmentWitnessOf<T>)>,
}

pub fn migrate_profiles_content<T: Config>(_available_weight: Weight) -> Weight {
    let mut count = 0;

    crate::Profiles::<T>::translate(|_key, profile: OldRegisteredProfile<T>| {
        count += 1;

        let OldRegisteredProfile {
            account,
            deposit,
            profile,
            assignment,
        } = profile;

        let mut direct_rpc_urls = BoundedVec::new();
        let mut bootnode_url = None;
        let mut node_type = NodeType::Substrate;

        match profile.mode {
            ProfileMode::Bootnode => {
                bootnode_url = Some(profile.url);
            }
            ProfileMode::Rpc {
                supports_ethereum_rpcs,
            } => {
                direct_rpc_urls
                    .try_push(profile.url)
                    .expect("limit to be at least 1");

                if supports_ethereum_rpcs {
                    node_type = NodeType::Frontier
                };
            }
        }

        Some(RegisteredProfile {
            account,
            deposit,
            assignment,
            profile: Profile {
                para_ids: profile.para_ids,
                assignment_request: profile.assignment_request,
                direct_rpc_urls,
                proxy_rpc_urls: BoundedVec::new(),
                bootnode_url,
                node_type,
                additional_info: BoundedVec::new(),
            },
        })
    });

    let db_weights = T::DbWeight::get();
    db_weights.reads_writes(count, count)
}
