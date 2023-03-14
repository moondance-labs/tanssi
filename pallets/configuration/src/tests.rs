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
    });
}
