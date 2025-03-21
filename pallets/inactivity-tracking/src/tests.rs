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
use crate::ActiveCollatorsForCurrentSession;
use sp_core::ConstU32;
use {
    crate::{
        mock::*, ActiveCollators, AuthorNotingHook, Config, EnableInactivityTracking,
        NodeActivityTrackingHelper, Pallet,
    },
    frame_support::{assert_noop, assert_ok, pallet_prelude::Get},
    sp_runtime::{BoundedVec, DispatchError::BadOrigin},
    tp_traits::{AuthorNotingInfo, GetSessionIndex},
};

fn get_active_collators(block: u32) -> AuthorNotingInfo<AccountId> {
    AuthorNotingInfo {
        block_number: block,
        author: COLLATOR_1,
        para_id: CONTAINER_CHAIN_ID_1,
    }
}

#[test]
fn enabling_and_disabling_inactivty_tracking_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(EnableInactivityTracking::<Test>::get(), false);

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(EnableInactivityTracking::<Test>::get(), true);

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));
        assert_eq!(EnableInactivityTracking::<Test>::get(), false);
    });
}

#[test]
fn enabling_and_disabling_inactivty_tracking_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_inactivity_tracking_status(RuntimeOrigin::signed(COLLATOR_1), true),
            BadOrigin
        );
    });
}

#[test]
fn inactivity_tracking_handler_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );

        let max_inactive_sessions: u32 = <Test as Config>::MaxInactiveSessions::get();

        roll_to(max_inactive_sessions as u64 * SESSION_BLOCK_LENGTH);
        assert_eq!(
            <Test as Config>::CurrentSessionIndex::session_index(),
            max_inactive_sessions
        );

        for session_id in 0..max_inactive_sessions {
            let active_collators = BoundedVec::truncate_from(vec![COLLATOR_1]);
            ActiveCollators::<Test>::insert(session_id, active_collators.clone());
            assert_eq!(ActiveCollators::<Test>::get(session_id), active_collators);
        }

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            true
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );

        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            false
        ));

        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_2
            ),
            false
        );
        assert_eq!(
            <Pallet::<Test> as NodeActivityTrackingHelper<AccountId>>::is_node_inactive(
                &COLLATOR_1
            ),
            false
        );
    });
}

#[test]
fn active_collators_noting_for_current_session_works() {
    ExtBuilder::default().build().execute_with(|| {
        let current_session_active_collator_record: BoundedVec<AccountId, ConstU32<5>> =
            BoundedVec::truncate_from(vec![COLLATOR_1]);
        assert_ok!(Pallet::<Test>::set_inactivity_tracking_status(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(ActiveCollatorsForCurrentSession::<Test>::get().len(), 0);
        roll_to(2);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(2),
        ]);
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
        roll_to(3);
        <Pallet<Test> as AuthorNotingHook<AccountId>>::on_container_authors_noted(&[
            get_active_collators(3),
        ]);
        assert_eq!(
            ActiveCollatorsForCurrentSession::<Test>::get(),
            current_session_active_collator_record
        );
    });
}
