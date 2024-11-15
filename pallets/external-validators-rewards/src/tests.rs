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
    crate::{self as pallet_external_validators_rewards, mock::*},
    sp_std::collections::btree_map::BTreeMap,
    tp_traits::{ActiveEraInfo, OnEraStart},
};

#[test]
fn basic_setup_works() {
    new_test_ext().execute_with(|| {
        // Mock::mutate(|mock| mock.active_era = Some(ActiveEraInfo { index: 0, start: None}));
        let storage_eras =
            pallet_external_validators_rewards::RewardPointsForEra::<Test>::iter().count();
        assert_eq!(storage_eras, 0);
    });
}

#[test]
fn can_reward_validators() {
    new_test_ext().execute_with(|| {
        Mock::mutate(|mock| {
            mock.active_era = Some(ActiveEraInfo {
                index: 1,
                start: None,
            })
        });
        ExternalValidatorsRewards::reward_by_ids([(1, 10), (3, 30), (5, 50)]);
        ExternalValidatorsRewards::reward_by_ids([(1, 10), (3, 10), (5, 10)]);

        let storage_eras =
            pallet_external_validators_rewards::RewardPointsForEra::<Test>::iter().count();
        assert_eq!(storage_eras, 1);

        let era_points = pallet_external_validators_rewards::RewardPointsForEra::<Test>::get(1);
        let mut expected_map = BTreeMap::new();
        expected_map.insert(1, 20);
        expected_map.insert(3, 40);
        expected_map.insert(5, 60);
        assert_eq!(era_points.individual, expected_map);
        assert_eq!(era_points.total, 20 + 40 + 60);
    })
}

#[test]
fn history_limit() {
    new_test_ext().execute_with(|| {
        Mock::mutate(|mock| {
            mock.active_era = Some(ActiveEraInfo {
                index: 1,
                start: None,
            })
        });
        ExternalValidatorsRewards::reward_by_ids([(1, 10), (3, 30), (5, 50)]);

        let storage_eras =
            pallet_external_validators_rewards::RewardPointsForEra::<Test>::iter().count();
        assert_eq!(storage_eras, 1);

        ExternalValidatorsRewards::on_era_start(10, 0);
        let storage_eras =
            pallet_external_validators_rewards::RewardPointsForEra::<Test>::iter().count();
        assert_eq!(storage_eras, 1, "shouldn't erase data yet");

        ExternalValidatorsRewards::on_era_start(11, 0);
        let storage_eras =
            pallet_external_validators_rewards::RewardPointsForEra::<Test>::iter().count();
        assert_eq!(storage_eras, 0, "data should be erased now");
    })
}
