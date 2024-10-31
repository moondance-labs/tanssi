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
    crate::{
        mock::{
            new_test_ext, run_to_block, run_to_session, ExternalValidators, HookCall, Mock,
            RootAccount, RuntimeEvent, RuntimeOrigin, Session, System, Test,
        },
        Error,
    },
    frame_support::{assert_noop, assert_ok},
    sp_runtime::traits::BadOrigin,
    tp_traits::ValidatorProvider,
};

#[test]
fn basic_setup_works() {
    new_test_ext().execute_with(|| {
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
    });
}

#[test]
fn add_whitelisted_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        let new = 3;

        // function runs
        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            new
        ));

        System::assert_last_event(RuntimeEvent::ExternalValidators(
            crate::Event::WhitelistedValidatorAdded { account_id: new },
        ));

        // same element cannot be added more than once
        assert_noop!(
            ExternalValidators::add_whitelisted(RuntimeOrigin::signed(RootAccount::get()), new),
            Error::<Test>::AlreadyWhitelisted
        );

        // new element is now part of the invulnerables list
        assert!(ExternalValidators::whitelisted_validators()
            .to_vec()
            .contains(&new));

        // cannot add with non-root
        assert_noop!(
            ExternalValidators::add_whitelisted(RuntimeOrigin::signed(1), new),
            BadOrigin
        );
    });
}

#[test]
fn add_whitelisted_does_not_work_if_not_registered() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        let new = 42;

        assert_noop!(
            ExternalValidators::add_whitelisted(RuntimeOrigin::signed(RootAccount::get()), new),
            Error::<Test>::NoKeysRegistered
        );
    });
}

#[test]
fn validator_limit_works() {
    new_test_ext().execute_with(|| {
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);

        // MaxExternalValidators: u32 = 20
        for ii in 3..=21 {
            if ii < 21 {
                assert_ok!(ExternalValidators::add_whitelisted(
                    RuntimeOrigin::signed(RootAccount::get()),
                    ii
                ));
            } else {
                assert_noop!(
                    ExternalValidators::add_whitelisted(
                        RuntimeOrigin::signed(RootAccount::get()),
                        ii
                    ),
                    Error::<Test>::TooManyWhitelisted
                );
            }
        }
        let expected: Vec<u64> = (1..=20).collect();
        assert_eq!(ExternalValidators::whitelisted_validators(), expected);
    });
}

#[test]
fn remove_whitelisted_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);

        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            4
        ));
        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            3
        ));

        assert_eq!(
            ExternalValidators::whitelisted_validators(),
            vec![1, 2, 4, 3]
        );

        assert_ok!(ExternalValidators::remove_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            2
        ));

        System::assert_last_event(RuntimeEvent::ExternalValidators(
            crate::Event::WhitelistedValidatorRemoved { account_id: 2 },
        ));
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 4, 3]);

        // cannot remove invulnerable not in the list
        assert_noop!(
            ExternalValidators::remove_whitelisted(RuntimeOrigin::signed(RootAccount::get()), 2),
            Error::<Test>::NotWhitelisted
        );

        // cannot remove without privilege
        assert_noop!(
            ExternalValidators::remove_whitelisted(RuntimeOrigin::signed(1), 3),
            BadOrigin
        );
    });
}

#[test]
fn whitelisted_and_external_order() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        assert_ok!(ExternalValidators::set_external_validators(vec![50, 51]));

        run_to_session(6);
        let validators = Session::validators();
        assert_eq!(validators, vec![1, 2, 50, 51]);
    });
}

#[test]
fn validator_provider_returns_all_validators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        assert_ok!(ExternalValidators::set_external_validators(vec![50, 51]));

        run_to_session(6);
        let validators_new_session = Session::validators();
        let validators_provider = <ExternalValidators as ValidatorProvider<u64>>::validators();
        assert_eq!(validators_new_session, validators_provider);
    });
}

#[test]
fn can_skip_external_validators() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        assert_ok!(ExternalValidators::set_external_validators(vec![50, 51]));
        assert_ok!(ExternalValidators::skip_external_validators(
            RuntimeOrigin::signed(RootAccount::get()),
            true
        ));

        run_to_session(6);
        let validators = Session::validators();
        assert_eq!(validators, vec![1, 2]);
    });
}

#[test]
fn duplicate_validators_are_deduplicated() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![1, 2]);
        assert_ok!(ExternalValidators::set_external_validators(vec![2]));

        run_to_session(6);
        let validators = Session::validators();
        assert_eq!(validators, vec![1, 2]);
    });
}

#[test]
fn duplicate_validator_order_is_preserved() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        // Whitelisted validators have priority, so their ordering should be respected
        // Need to manually remove and add each whitelisted because there is no "set_whitelisted"
        assert_ok!(ExternalValidators::remove_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            1
        ));
        assert_ok!(ExternalValidators::remove_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            2
        ));
        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            3
        ));
        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            1
        ));
        assert_ok!(ExternalValidators::add_whitelisted(
            RuntimeOrigin::signed(RootAccount::get()),
            2
        ));
        assert_eq!(ExternalValidators::whitelisted_validators(), vec![3, 1, 2]);
        assert_ok!(ExternalValidators::set_external_validators(vec![
            3, 2, 1, 4
        ]));

        run_to_session(6);
        let validators = Session::validators();
        assert_eq!(validators, vec![3, 1, 2, 4]);
    });
}

#[test]
fn era_hooks() {
    new_test_ext().execute_with(|| {
        run_to_session(14);

        let expected_calls = vec![
            HookCall::OnEraStart { era: 0, session: 0 },
            HookCall::OnEraEnd { era: 0 },
            HookCall::OnEraStart { era: 1, session: 6 },
            HookCall::OnEraEnd { era: 1 },
            HookCall::OnEraStart {
                era: 2,
                session: 12,
            },
        ];

        assert_eq!(Mock::mock().called_hooks, expected_calls);
    });
}
