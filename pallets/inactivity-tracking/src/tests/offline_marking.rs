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

use {super::*, crate::EnableMarkingOffline};

#[test]
fn enabling_and_disabling_offline_marking_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(EnableMarkingOffline::<Test>::get(), false);
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(EnableMarkingOffline::<Test>::get(), true);
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            false
        ));
        assert_eq!(EnableMarkingOffline::<Test>::get(), false);
    });
}

#[test]
fn enabling_and_disabling_offline_marking_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::enable_offline_marking(RuntimeOrigin::signed(COLLATOR_1), true),
            BadOrigin
        );
    });
}
#[test]
fn set_offline_works() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), false);
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_ok!(Pallet::<Test>::set_offline(RuntimeOrigin::signed(
            COLLATOR_1
        )));
        System::assert_last_event(
            Event::CollatorStatusUpdated {
                collator: COLLATOR_1,
                is_offline: true,
            }
            .into(),
        );
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), true);
    });
}
#[test]
fn set_offline_fails_if_offline_marking_is_not_enabled() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::set_offline(RuntimeOrigin::signed(COLLATOR_1)),
            Error::<Test>::MarkingOfflineNotEnabled
        );
    });
}
#[test]
fn set_offline_fails_if_collator_is_not_in_eligible_candidates() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Test>::set_offline(RuntimeOrigin::signed(COLLATOR_3)),
            Error::<Test>::CollatorNotInSortedEligibleCandidates
        );
    });
}

#[test]
fn set_offline_fails_for_offline_collators() {
    ExtBuilder.build().execute_with(|| {
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        OfflineCollators::<Test>::insert(COLLATOR_1, true);
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), true);
        assert_noop!(
            Pallet::<Test>::set_offline(RuntimeOrigin::signed(COLLATOR_1)),
            Error::<Test>::CollatorNotOnline
        );
    });
}

#[test]
fn set_offline_fails_if_collator_is_invulnerable() {
    ExtBuilder.build().execute_with(|| {
        assert_ok!(Pallet::<Test>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Test>::set_offline(RuntimeOrigin::signed(COLLATOR_2)),
            Error::<Test>::MarkingInvulnerableOfflineInvalid
        );
    });
}

#[test]
fn set_online_works() {
    ExtBuilder.build().execute_with(|| {
        OfflineCollators::<Test>::insert(COLLATOR_1, true);
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), true);
        assert_ok!(Pallet::<Test>::set_online(RuntimeOrigin::signed(
            COLLATOR_1
        )));
        System::assert_last_event(
            Event::CollatorStatusUpdated {
                collator: COLLATOR_1,
                is_offline: false,
            }
            .into(),
        );
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), false);
    });
}

#[test]
fn set_online_fails_for_online_collators() {
    ExtBuilder.build().execute_with(|| {
        assert_eq!(OfflineCollators::<Test>::get(COLLATOR_1), false);
        assert_noop!(
            Pallet::<Test>::set_online(RuntimeOrigin::signed(COLLATOR_1)),
            Error::<Test>::CollatorNotOffline
        );
    });
}
