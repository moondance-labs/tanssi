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
    crate::mock::{new_test_ext, session_change_validators, Initializer},
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

        assert_eq!(session_change_validators(), Some((0, Vec::new())));
    });
}

#[test]
fn session_change_applied() {
    new_test_ext().execute_with(|| {
        Initializer::test_trigger_on_new_session(
            false,
            1,
            Vec::new().into_iter(),
            Some(Vec::new().into_iter()),
        );

        // Session change validators are applied
        assert_eq!(session_change_validators(), Some((1, Vec::new())));
    });
}
