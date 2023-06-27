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
    crate::mock::{new_test_ext, session_change_validators, Initializer, System, Test},
};

#[test]
fn session_0_is_instantly_applied() {
    new_test_ext().execute_with(|| {
        Initializer::test_trigger_on_new_session(
            false,
            0,
            Vec::new().into_iter(),
            Some(Vec::new().into_iter()),
        );

        let v = BufferedSessionChanges::<Test>::get();
        assert!(v.is_none());

        assert_eq!(session_change_validators(), Some((0, Vec::new())));
    });
}

#[test]
fn session_change_before_initialize_is_still_buffered_after() {
    new_test_ext().execute_with(|| {
        Initializer::test_trigger_on_new_session(
            false,
            1,
            Vec::new().into_iter(),
            Some(Vec::new().into_iter()),
        );

        let now = System::block_number();
        Initializer::on_initialize(now);

        // Session change validators are applied after on_finalize
        assert_eq!(session_change_validators(), None);

        let v = BufferedSessionChanges::<Test>::get();
        assert!(v.is_some());
    });
}

#[test]
fn session_change_applied_on_finalize() {
    new_test_ext().execute_with(|| {
        Initializer::on_initialize(1);
        Initializer::test_trigger_on_new_session(
            false,
            1,
            Vec::new().into_iter(),
            Some(Vec::new().into_iter()),
        );

        Initializer::on_finalize(1);

        // Session change validators are applied after on_finalize
        assert_eq!(session_change_validators(), Some((1, Vec::new())));

        assert!(BufferedSessionChanges::<Test>::get().is_none());
    });
}
