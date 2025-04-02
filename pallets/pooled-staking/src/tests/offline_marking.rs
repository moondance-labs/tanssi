use {
    super::*,
    crate::{
        candidate::EligibleCandidate, EnableMarkingOffline, Error, OfflineCollators, Pallet,
        SortedEligibleCandidates,
    },
    frame_support::{assert_noop, assert_ok},
    sp_runtime::{BoundedVec, DispatchError::BadOrigin},
};

#[test]
fn enabling_and_disabling_offline_marking_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(EnableMarkingOffline::<Runtime>::get(), false);

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_eq!(EnableMarkingOffline::<Runtime>::get(), true);

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            false
        ));
        assert_eq!(EnableMarkingOffline::<Runtime>::get(), false);
    });
}

#[test]
fn enabling_and_disabling_offline_marking_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Runtime>::enable_offline_marking(
                RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1),
                true
            ),
            BadOrigin
        );
    });
}

#[test]
fn set_offline_fails_if_offline_marking_is_not_enabled() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Runtime>::set_offline(RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1)),
            Error::<Runtime>::MarkingOfflineNotEnabled
        );
    });
}

#[test]
fn set_offline_fails_if_collator_is_not_in_eligible_candidates() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Runtime>::set_offline(RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1)),
            Error::<Runtime>::CollatorDoesNotExist
        );
    });
}

#[test]
fn set_offline_fails_if_collator_is_invulnerable() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Runtime>::set_offline(RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1)),
            Error::<Runtime>::MarkingInvulnerableOfflineInvalid
        );
    });
}

#[test]
fn set_offline_works() {
    ExtBuilder::default().build().execute_with(|| {
        let share = InitialAutoCompoundingShareValue::get();
        let candidate = EligibleCandidate {
            candidate: ACCOUNT_CANDIDATE_2,
            stake: 1 * share,
        };
        SortedEligibleCandidates::<Runtime>::put(BoundedVec::truncate_from(
            vec![candidate.clone()],
        ));
        assert_eq!(
            OfflineCollators::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            false
        );
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            true
        );

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_ok!(Pallet::<Runtime>::set_offline(RuntimeOrigin::signed(
            ACCOUNT_CANDIDATE_2
        )));
        System::assert_last_event(RuntimeEvent::Staking(crate::Event::CollatorOffline {
            collator: ACCOUNT_CANDIDATE_2,
        }));
        assert_eq!(
            OfflineCollators::<Runtime>::get().contains(&candidate),
            true
        );
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().contains(&candidate),
            false
        );
    });
}

#[test]
fn set_online_fails_if_collator_is_not_offline() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Runtime>::set_online(RuntimeOrigin::signed(ACCOUNT_CANDIDATE_1)),
            Error::<Runtime>::CollatorDoesNotExist
        );
    });
}

#[test]
fn set_online_works() {
    ExtBuilder::default().build().execute_with(|| {
        let share = InitialAutoCompoundingShareValue::get();
        let candidate = EligibleCandidate {
            candidate: ACCOUNT_CANDIDATE_2,
            stake: 1 * share,
        };

        OfflineCollators::<Runtime>::put(BoundedVec::truncate_from(vec![candidate.clone()]));
        assert_eq!(
            OfflineCollators::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            true
        );
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            false
        );

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_ok!(Pallet::<Runtime>::set_online(RuntimeOrigin::signed(
            ACCOUNT_CANDIDATE_2
        )));
        System::assert_last_event(RuntimeEvent::Staking(crate::Event::CollatorOnline {
            collator: ACCOUNT_CANDIDATE_2,
        }));
        assert_eq!(
            OfflineCollators::<Runtime>::get().contains(&candidate),
            false
        );
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().contains(&candidate),
            true
        );
    });
}

#[test]
fn notify_inactive_collator_fails_if_offline_marking_is_not_enabled() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Pallet::<Runtime>::notify_inactive_collator(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_2
            ),
            Error::<Runtime>::MarkingOfflineNotEnabled
        );
    });
}

#[test]
fn notify_inactive_collator_fails_if_collator_is_invulnerable() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Runtime>::notify_inactive_collator(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_1
            ),
            Error::<Runtime>::MarkingInvulnerableOfflineInvalid
        );
    });
}

#[test]
fn notify_inactive_collator_fails_if_collator_is_active() {
    ExtBuilder::default().build().execute_with(|| {
        let share = InitialAutoCompoundingShareValue::get();

        let candidate = EligibleCandidate {
            candidate: ACCOUNT_CANDIDATE_3,
            stake: 1 * share,
        };
        SortedEligibleCandidates::<Runtime>::put(BoundedVec::truncate_from(
            vec![candidate.clone()],
        ));
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            true
        );

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_noop!(
            Pallet::<Runtime>::notify_inactive_collator(
                RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
                ACCOUNT_CANDIDATE_3
            ),
            Error::<Runtime>::CollatorCannotBeNotifiedAsInactive
        );
    });
}

#[test]
fn notify_inactive_collator_works() {
    ExtBuilder::default().build().execute_with(|| {
        let share = InitialAutoCompoundingShareValue::get();

        let candidate = EligibleCandidate {
            candidate: ACCOUNT_CANDIDATE_2,
            stake: 1 * share,
        };
        SortedEligibleCandidates::<Runtime>::put(BoundedVec::truncate_from(
            vec![candidate.clone()],
        ));
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get()
                .into_inner()
                .contains(&candidate),
            true
        );

        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_ok!(Pallet::<Runtime>::notify_inactive_collator(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
            ACCOUNT_CANDIDATE_2
        ));

        System::assert_last_event(RuntimeEvent::Staking(crate::Event::CollatorOffline {
            collator: ACCOUNT_CANDIDATE_2,
        }));
        assert_eq!(
            OfflineCollators::<Runtime>::get().contains(&candidate),
            true
        );
        assert_eq!(
            SortedEligibleCandidates::<Runtime>::get().contains(&candidate),
            false
        );
    });
}
