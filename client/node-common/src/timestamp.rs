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

use std::sync::atomic::{AtomicU64, Ordering};

static TIMESTAMP: AtomicU64 = AtomicU64::new(0);

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making Aura think time has passed.
pub struct MockTimestampInherentDataProvider;

impl MockTimestampInherentDataProvider {
    pub fn advance_timestamp(slot_duration: u64) {
        if TIMESTAMP.load(Ordering::SeqCst) == 0 {
            // Initialize timestamp inherent provider
            TIMESTAMP.store(
                sp_timestamp::Timestamp::current().as_millis(),
                Ordering::SeqCst,
            );
        } else {
            TIMESTAMP.fetch_add(slot_duration, Ordering::SeqCst);
        }
    }

    pub fn load() -> u64 {
        TIMESTAMP.load(Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for MockTimestampInherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        inherent_data.put_data(
            sp_timestamp::INHERENT_IDENTIFIER,
            &TIMESTAMP.load(Ordering::SeqCst),
        )
    }

    async fn try_handle_error(
        &self,
        _identifier: &sp_inherents::InherentIdentifier,
        _error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        // The pallet never reports error.
        None
    }
}
