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

/*
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
        assert_ok!(Pallet::<Runtime>::enable_offline_marking(
            RuntimeOrigin::root(),
            true
        ));
        assert_ok!(Pallet::<Runtime>::notify_inactive_collator(
            RuntimeOrigin::signed(ACCOUNT_DELEGATOR_1),
            ACCOUNT_CANDIDATE_2
        ));
    });
}
*/
