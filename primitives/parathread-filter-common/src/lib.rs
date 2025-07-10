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
#![cfg_attr(not(feature = "std"), no_std)]
use {
    sp_std::{collections::btree_set::BTreeSet, marker::PhantomData},
    tp_traits::{GetSessionContainerChains, ParaId, ParathreadHelper},
};

// Common implementation of the ParathreadHelper trait for all chains
// supporting pallet_session and pallet_registrar.
pub struct ExcludeAllParathreadsFilter<Runtime>(PhantomData<Runtime>);
impl<Runtime: pallet_session::Config + pallet_registrar::Config> ParathreadHelper
    for ExcludeAllParathreadsFilter<Runtime>
{
    fn get_parathreads_for_session() -> BTreeSet<ParaId> {
        pallet_registrar::Pallet::<Runtime>::session_container_chains(
            pallet_session::Pallet::<Runtime>::current_index().into(),
        )
        .parathreads
        .iter()
        .map(|(id, _)| *id)
        .collect()
    }
}
