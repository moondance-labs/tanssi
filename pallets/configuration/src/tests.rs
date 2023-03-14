use crate::{mock::*, Error, Event, HostConfiguration};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

#[test]
fn config_sets_values_from_genesis() {
    let custom_config = HostConfiguration {
        max_collators: 100,
        moondance_collators: 40,
        collators_per_container: 20,
    };
    new_test_ext_with_genesis(custom_config.clone()).execute_with(|| {
        System::set_block_number(1);
        assert_eq!(Configuration::config(), custom_config);
    });
}

#[test]
fn config_set_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_eq!(Configuration::config().max_collators, 0);
        assert_ok!(
            Configuration::set_max_collators(RuntimeOrigin::root(), 50),
            ()
        );
        // The session delay is set to 2, so the change should not happen until block 3
        assert_eq!(Configuration::config().max_collators, 0);
        System::set_block_number(2);
        assert_eq!(Configuration::config().max_collators, 0);
        System::set_block_number(3);
        // TODO: the session delay is 2 so this should work, but it doesnt
        // This is probably because the configuration pallet is missing on_initialize or
        // on_finalize hooks
        assert_eq!(Configuration::config().max_collators, 50);
        System::set_block_number(4);
        assert_eq!(Configuration::config().max_collators, 50);
        System::set_block_number(5);
        assert_eq!(Configuration::config().max_collators, 50);
        System::set_block_number(6);
        assert_eq!(Configuration::config().max_collators, 50);
        System::set_block_number(7);
        assert_eq!(Configuration::config().max_collators, 50);
    });
}
