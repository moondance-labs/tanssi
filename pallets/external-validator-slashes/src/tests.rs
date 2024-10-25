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
    crate::mock::{new_test_ext, ExternalValidatorSlashes, RuntimeOrigin, Test},
    frame_support::{assert_noop, assert_ok},
};

#[test]
fn root_can_inject_manual_offence() {
    new_test_ext().execute_with(|| {
        start_era(0, 0);
        assert_ok!(ExternalValidatorSlashes::force_inject_slash(
            RuntimeOrigin::root(),
            0,
            1u64,
            Perbill::from_percent(75)
        ));
        assert_eq!(
            Slashes::<Test>::get(3),
            vec![Slash {
                validator: 1,
                percentage: Perbill::from_percent(75),
                confirmed: false,
                reporters: vec![],
                slash_id: 0
            }]
        );
        assert_eq!(NextSlashId::<Test>::get(), 1);
    });
}

#[test]
fn cannot_inject_future_era_offence() {
    new_test_ext().execute_with(|| {
        start_era(0, 0);
        assert_noop!(
            ExternalValidatorSlashes::force_inject_slash(
                RuntimeOrigin::root(),
                1,
                1u64,
                Perbill::from_percent(75)
            ),
            Error::<Test>::ProvidedFutureEra
        );
    });
}

#[test]
fn cannot_inject_era_offence_too_far_in_the_past() {
    new_test_ext().execute_with(|| {
        start_era(10, 0);
        //Bonding period is 5, we cannot inject slash for era 4
        assert_noop!(
            ExternalValidatorSlashes::force_inject_slash(
                RuntimeOrigin::root(),
                1,
                4u64,
                Perbill::from_percent(75)
            ),
            Error::<Test>::ProvidedNonSlashableEra
        );
    });
}

#[test]
fn root_can_cance_deferred_slash() {
    new_test_ext().execute_with(|| {
        start_era(1, 0);
        assert_ok!(ExternalValidatorSlashes::force_inject_slash(
            RuntimeOrigin::root(),
            0,
            1u64,
            Perbill::from_percent(75)
        ));
        assert_ok!(ExternalValidatorSlashes::cancel_deferred_slash(
            RuntimeOrigin::root(),
            3,
            vec![0]
        ));

        assert_eq!(Slashes::<Test>::get(3), vec![]);
    });
}

#[test]
fn root_cannot_cancel_deferred_slash_if_outside_deferring_period() {
    new_test_ext().execute_with(|| {
        start_era(1, 0);
        assert_ok!(ExternalValidatorSlashes::force_inject_slash(
            RuntimeOrigin::root(),
            0,
            1u64,
            Perbill::from_percent(75)
        ));

        start_era(4, 0);

        assert_noop!(
            ExternalValidatorSlashes::cancel_deferred_slash(RuntimeOrigin::root(), 0, vec![0]),
            Error::<Test>::DeferPeriodIsOver
        );
    });
}

#[test]
fn test_after_bonding_period_we_can_remove_slashes() {
    new_test_ext().execute_with(|| {
        start_era(0, 0);
        start_era(1, 1);

        // we are storing a tuple (era index, start_session_block)
        assert_eq!(BondedEras::<Test>::get(), [(0, 0), (1, 1)]);
        assert_ok!(ExternalValidatorSlashes::force_inject_slash(
            RuntimeOrigin::root(),
            0,
            1u64,
            Perbill::from_percent(75)
        ));

        assert_eq!(
            Slashes::<Test>::get(3),
            vec![Slash {
                validator: 1,
                percentage: Perbill::from_percent(75),
                confirmed: false,
                reporters: vec![],
                slash_id: 0
            }]
        );

        Pallet::<Test>::on_era_start(3, 3);

        start_era(8, 8);

        // whenever we start the 6th era, we can remove everything from era 3
        Pallet::<Test>::on_era_start(9, 9);

        assert_eq!(Slashes::<Test>::get(3), vec![]);
    });
}

#[test]
fn test_on_offence_injects_offences() {
    new_test_ext().execute_with(|| {
        start_era(0, 0);
        start_era(1, 1);
        Pallet::<Test>::on_offence(
            &[OffenceDetails {
                // 1 and 2 are invulnerables
                offender: (3, ()),
                reporters: vec![],
            }],
            &[Perbill::from_percent(75)],
            0,
        );
        // current era (1) + defer period + 1
        let slash_era = 0
            .saturating_add(crate::mock::DeferPeriod::get())
            .saturating_add(One::one());

        assert_eq!(
            Slashes::<Test>::get(slash_era),
            vec![Slash {
                validator: 3,
                percentage: Perbill::from_percent(75),
                confirmed: false,
                reporters: vec![],
                slash_id: 0
            }]
        );
    });
}

#[test]
fn test_on_offence_does_not_work_for_invulnerables() {
    new_test_ext().execute_with(|| {
        start_era(0, 0);
        start_era(1, 1);
        // account 1 invulnerable
        Pallet::<Test>::on_offence(
            &[OffenceDetails {
                offender: (1, ()),
                reporters: vec![],
            }],
            &[Perbill::from_percent(75)],
            0,
        );
        // current era (1) + defer period + 1
        let slash_era = 1
            .saturating_add(crate::mock::DeferPeriod::get())
            .saturating_add(One::one());

        assert_eq!(Slashes::<Test>::get(slash_era), vec![]);
    });
}

fn start_era(era_index: EraIndex, session_index: SessionIndex) {
    Pallet::<Test>::on_era_start(era_index, session_index);
    crate::mock::MockEraIndexProvider::with_era(era_index);
}
