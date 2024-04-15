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
            initialize_to_block, new_test_ext, Invulnerables, RootAccount, RuntimeEvent,
            RuntimeOrigin, System, Test,
        },
        Error,
    },
    frame_support::{assert_noop, assert_ok},
    sp_runtime::traits::BadOrigin,
};

#[test]
fn basic_setup_works() {
    new_test_ext().execute_with(|| {
        // genesis should sort input
        assert_eq!(Invulnerables::invulnerables(), vec![1, 2]);
    });
}

#[test]
fn add_invulnerable_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);
        assert_eq!(Invulnerables::invulnerables(), vec![1, 2]);
        let new = 3;

        // function runs
        assert_ok!(Invulnerables::add_invulnerable(
            RuntimeOrigin::signed(RootAccount::get()),
            new
        ));

        System::assert_last_event(RuntimeEvent::Invulnerables(
            crate::Event::InvulnerableAdded { account_id: new },
        ));

        // same element cannot be added more than once
        assert_noop!(
            Invulnerables::add_invulnerable(RuntimeOrigin::signed(RootAccount::get()), new),
            Error::<Test>::AlreadyInvulnerable
        );

        // new element is now part of the invulnerables list
        assert!(Invulnerables::invulnerables().to_vec().contains(&new));

        // cannot add with non-root
        assert_noop!(
            Invulnerables::add_invulnerable(RuntimeOrigin::signed(1), new),
            BadOrigin
        );
    });
}

#[test]
fn add_invulnerable_does_not_work_if_not_registered() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);
        assert_eq!(Invulnerables::invulnerables(), vec![1, 2]);
        let new = 42;

        assert_noop!(
            Invulnerables::add_invulnerable(RuntimeOrigin::signed(RootAccount::get()), new),
            Error::<Test>::NoKeysRegistered
        );
    });
}

#[test]
fn invulnerable_limit_works() {
    new_test_ext().execute_with(|| {
        assert_eq!(Invulnerables::invulnerables(), vec![1, 2]);

        // MaxInvulnerables: u32 = 20
        for ii in 3..=21 {
            if ii < 21 {
                assert_ok!(Invulnerables::add_invulnerable(
                    RuntimeOrigin::signed(RootAccount::get()),
                    ii
                ));
            } else {
                assert_noop!(
                    Invulnerables::add_invulnerable(RuntimeOrigin::signed(RootAccount::get()), ii),
                    Error::<Test>::TooManyInvulnerables
                );
            }
        }
        let expected: Vec<u64> = (1..=20).collect();
        assert_eq!(Invulnerables::invulnerables(), expected);
    });
}

#[test]
fn remove_invulnerable_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);
        assert_eq!(Invulnerables::invulnerables(), vec![1, 2]);

        assert_ok!(Invulnerables::add_invulnerable(
            RuntimeOrigin::signed(RootAccount::get()),
            4
        ));
        assert_ok!(Invulnerables::add_invulnerable(
            RuntimeOrigin::signed(RootAccount::get()),
            3
        ));

        assert_eq!(Invulnerables::invulnerables(), vec![1, 2, 4, 3]);

        assert_ok!(Invulnerables::remove_invulnerable(
            RuntimeOrigin::signed(RootAccount::get()),
            2
        ));

        System::assert_last_event(RuntimeEvent::Invulnerables(
            crate::Event::InvulnerableRemoved { account_id: 2 },
        ));
        assert_eq!(Invulnerables::invulnerables(), vec![1, 4, 3]);

        // cannot remove invulnerable not in the list
        assert_noop!(
            Invulnerables::remove_invulnerable(RuntimeOrigin::signed(RootAccount::get()), 2),
            Error::<Test>::NotInvulnerable
        );

        // cannot remove without privilege
        assert_noop!(
            Invulnerables::remove_invulnerable(RuntimeOrigin::signed(1), 3),
            BadOrigin
        );
    });
}
