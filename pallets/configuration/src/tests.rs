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
    crate::{mock::*, HostConfiguration, PendingConfigs},
    frame_support::assert_ok,
    sp_std::vec,
};

#[test]
fn config_sets_values_from_genesis() {
    let custom_config = HostConfiguration {
        max_collators: 100,
        min_orchestrator_collators: 40,
        max_orchestrator_collators: 40,
        collators_per_container: 20,
    };
    new_test_ext_with_genesis(custom_config.clone()).execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config(), custom_config);
    });
}

#[test]
fn config_sets_default_values() {
    let default_config = HostConfiguration {
        max_collators: 100,
        min_orchestrator_collators: 2,
        max_orchestrator_collators: 5,
        collators_per_container: 2,
    };
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config(), default_config);
    });
}

#[test]
fn config_set_value() {
    new_test_ext_with_genesis(HostConfiguration {
        max_collators: 0,
        min_orchestrator_collators: 0,
        max_orchestrator_collators: 0,
        collators_per_container: 0,
    })
    .execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_ok!(
            Configuration::set_max_collators(RuntimeOrigin::root(), 50),
            ()
        );

        assert_eq!(
            PendingConfigs::<Test>::get(),
            vec![(
                2,
                HostConfiguration {
                    max_collators: 50,
                    min_orchestrator_collators: 0,
                    max_orchestrator_collators: 0,
                    collators_per_container: 0,
                }
            )]
        );

        // The session delay is set to 2, and one session is 5 blocks,
        // so the change should not happen until block 11
        assert_eq!(Configuration::config().max_collators, 0);
        run_to_block(2);
        assert_eq!(Configuration::config().max_collators, 0);
        // First block of session 1
        run_to_block(6);
        assert_eq!(Configuration::config().max_collators, 0);
        run_to_block(10);
        assert_eq!(Configuration::config().max_collators, 0);
        // First block of session 2
        run_to_block(11);
        assert_eq!(Configuration::config().max_collators, 50);
    });
}

#[test]
fn config_set_many_values_same_block() {
    new_test_ext_with_genesis(HostConfiguration {
        max_collators: 0,
        min_orchestrator_collators: 0,
        max_orchestrator_collators: 0,
        collators_per_container: 0,
    })
    .execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_ok!(
            Configuration::set_max_collators(RuntimeOrigin::root(), 50),
            ()
        );
        assert_ok!(
            Configuration::set_min_orchestrator_collators(RuntimeOrigin::root(), 20),
            ()
        );
        assert_ok!(
            Configuration::set_collators_per_container(RuntimeOrigin::root(), 10),
            ()
        );

        assert_eq!(
            PendingConfigs::<Test>::get(),
            vec![(
                2,
                HostConfiguration {
                    max_collators: 50,
                    min_orchestrator_collators: 20,
                    max_orchestrator_collators: 20,
                    collators_per_container: 10,
                }
            )]
        );

        // The session delay is set to 2, and one session is 5 blocks,
        // so the change should not happen until block 11
        run_to_block(10);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        // First block of session 2
        run_to_block(11);
        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().collators_per_container, 10);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
    });
}

#[test]
fn config_set_many_values_different_blocks() {
    new_test_ext_with_genesis(HostConfiguration {
        max_collators: 0,
        min_orchestrator_collators: 0,
        max_orchestrator_collators: 0,
        collators_per_container: 0,
    })
    .execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_ok!(
            Configuration::set_max_collators(RuntimeOrigin::root(), 50),
            ()
        );
        run_to_block(2);
        assert_ok!(
            Configuration::set_min_orchestrator_collators(RuntimeOrigin::root(), 20),
            ()
        );
        run_to_block(3);
        assert_ok!(
            Configuration::set_collators_per_container(RuntimeOrigin::root(), 10),
            ()
        );

        assert_eq!(
            PendingConfigs::<Test>::get(),
            vec![(
                2,
                HostConfiguration {
                    max_collators: 50,
                    min_orchestrator_collators: 20,
                    max_orchestrator_collators: 20,
                    collators_per_container: 10,
                }
            )]
        );

        // The session delay is set to 2, and one session is 5 blocks,
        // so the change should not happen until block 11
        run_to_block(10);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        // First block of session 2
        run_to_block(11);
        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
        assert_eq!(Configuration::config().collators_per_container, 10);
    });
}

#[test]
fn config_set_many_values_different_sessions() {
    new_test_ext_with_genesis(HostConfiguration {
        max_collators: 0,
        min_orchestrator_collators: 0,
        max_orchestrator_collators: 0,
        collators_per_container: 0,
    })
    .execute_with(|| {
        run_to_block(1);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        assert_ok!(
            Configuration::set_max_collators(RuntimeOrigin::root(), 50),
            ()
        );
        run_to_block(6);
        assert_ok!(
            Configuration::set_min_orchestrator_collators(RuntimeOrigin::root(), 20),
            ()
        );
        assert_eq!(Configuration::config().max_collators, 0);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        run_to_block(11);
        assert_ok!(
            Configuration::set_collators_per_container(RuntimeOrigin::root(), 10),
            ()
        );

        assert_eq!(
            PendingConfigs::<Test>::get(),
            vec![
                (
                    3,
                    HostConfiguration {
                        max_collators: 50,
                        min_orchestrator_collators: 20,
                        max_orchestrator_collators: 20,
                        collators_per_container: 0,
                    }
                ),
                (
                    4,
                    HostConfiguration {
                        max_collators: 50,
                        min_orchestrator_collators: 20,
                        max_orchestrator_collators: 20,
                        collators_per_container: 10,
                    }
                )
            ]
        );

        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 0);
        assert_eq!(Configuration::config().collators_per_container, 0);
        run_to_block(16);
        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
        assert_eq!(Configuration::config().collators_per_container, 0);
        run_to_block(21);
        assert_eq!(Configuration::config().max_collators, 50);
        assert_eq!(Configuration::config().min_orchestrator_collators, 20);
        assert_eq!(Configuration::config().collators_per_container, 10);
    });
}
