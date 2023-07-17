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
    crate::mock::{new_test_ext, AuthorityMapping, Test},
};

#[test]
fn session_0_fills_in_first_mapping() {
    new_test_ext().execute_with(|| {
        AuthorityMapping::initializer_on_new_session(&0, &[(1, 1u64.into())]);

        let v = AuthorityIdMapping::<Test>::get(0).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v.get(&1u64.into()), Some(&1u64));
    });
}

#[test]
fn session_1_fills_in_second_mapping_but_does_not_remove_first() {
    new_test_ext().execute_with(|| {
        AuthorityMapping::initializer_on_new_session(&0, &[(1, 1u64.into())]);

        AuthorityMapping::initializer_on_new_session(&1, &[(1, 2u64.into())]);

        let v0 = AuthorityIdMapping::<Test>::get(0).unwrap();
        assert_eq!(v0.len(), 1);
        assert_eq!(v0.get(&1u64.into()), Some(&1u64));

        let v1 = AuthorityIdMapping::<Test>::get(1).unwrap();
        assert_eq!(v1.len(), 1);
        assert_eq!(v1.get(&2u64.into()), Some(&1u64));
    });
}

#[test]
fn session_2_fills_in_third_mapping_removes_first_not_second() {
    new_test_ext().execute_with(|| {
        AuthorityMapping::initializer_on_new_session(&0, &[(1, 1u64.into())]);

        AuthorityMapping::initializer_on_new_session(&1, &[(1, 2u64.into())]);

        AuthorityMapping::initializer_on_new_session(&2, &[(1, 3u64.into())]);

        assert!(AuthorityIdMapping::<Test>::get(0).is_none());

        let v1 = AuthorityIdMapping::<Test>::get(1).unwrap();
        assert_eq!(v1.len(), 1);
        assert_eq!(v1.get(&2u64.into()), Some(&1u64));

        let v2 = AuthorityIdMapping::<Test>::get(2).unwrap();
        assert_eq!(v2.len(), 1);
        assert_eq!(v2.get(&3u64.into()), Some(&1u64));
    });
}
