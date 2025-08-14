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
    crate::Config,
    frame_support::{pallet_prelude::Weight, traits::Get},
    tp_traits::GetSessionIndex,
};

// Migrating offline collator storage from bool to Option<u32>.
pub fn migrate_offline_collators_storage<T: Config>(_available_weight: Weight) -> Weight {
    let mut count = 0;
    crate::OfflineCollators::<T>::translate(|_key, value: bool| {
        count += 1;
        if value {
            Some(T::CurrentSessionIndex::session_index())
        } else {
            None
        }
    });
    log::info!("Migrated {} offline collators records", count);
    T::DbWeight::get().reads_writes(count, count)
}
