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

#![cfg(test)]

use {
    crate::{tests::common::*, Configuration},
    frame_support::assert_ok,
    runtime_parachains::configuration as parachains_configuration,
};

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get().dispute_period,
            6
        );
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get()
                .minimum_validation_upgrade_delay,
            2
        );

        // We need to advance to the first session first
        run_to_session(1u32);
        assert_ok!(
            Configuration::set_minimum_validation_upgrade_delay(root_origin(), 50),
            ()
        );

        run_to_session(2u32);

        assert_ok!(Configuration::set_dispute_period(root_origin(), 20), ());
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get().dispute_period,
            6
        );
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get()
                .minimum_validation_upgrade_delay,
            2
        );

        run_to_session(3u32);
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get().dispute_period,
            6
        );
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get()
                .minimum_validation_upgrade_delay,
            50
        );

        run_to_session(4u32);

        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get().dispute_period,
            20
        );
        assert_eq!(
            parachains_configuration::ActiveConfig::<Runtime>::get()
                .minimum_validation_upgrade_delay,
            50
        );
    });
}
