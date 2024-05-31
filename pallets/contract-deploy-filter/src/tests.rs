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
            initialize_to_block, new_test_ext, root_origin, ContractDeployFilter, RuntimeEvent,
            RuntimeOrigin, System, Test,
        },
        Config, Error,
    },
    core::str::FromStr,
    frame_support::{assert_noop, assert_ok},
    sp_core::H160,
    sp_runtime::traits::BadOrigin,
};

#[test]
fn allow_address_to_create_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);

        let alice_address: &str = "0x1a642f0e3c3af545e7acbd38b07251b3990914f1";
        let bob_address: &str = "0x2a642f0e3c3af545e7acbd38b07251b3990914f1";

        let alice_h160_address = H160::from_str(alice_address).unwrap();
        let bob_h160_address = H160::from_str(bob_address).unwrap();

        assert_ok!(ContractDeployFilter::allow_address_to_create(
            root_origin(),
            alice_h160_address
        ));

        System::assert_last_event(RuntimeEvent::ContractDeployFilter(
            crate::Event::AllowedAddressToCreate {
                address: alice_h160_address,
            },
        ));

        // fails when trying to allow the same address twice
        assert_noop!(
            ContractDeployFilter::allow_address_to_create(root_origin(), alice_h160_address),
            Error::<Test>::AlreadyAllowed
        );

        // the address is now in the corresponding list
        assert!(ContractDeployFilter::allowed_addresses_to_create()
            .to_vec()
            .contains(&alice_h160_address));

        // fails with an origin that is not root
        assert_noop!(
            ContractDeployFilter::allow_address_to_create(
                RuntimeOrigin::signed(1),
                bob_h160_address
            ),
            BadOrigin
        );
    });
}

#[test]
fn allow_address_to_create_inner_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);

        let alice_address: &str = "0x1a642f0e3c3af545e7acbd38b07251b3990914f1";
        let bob_address: &str = "0x2a642f0e3c3af545e7acbd38b07251b3990914f1";

        let alice_h160_address = H160::from_str(alice_address).unwrap();
        let bob_h160_address = H160::from_str(bob_address).unwrap();

        assert_ok!(ContractDeployFilter::allow_address_to_create_inner(
            root_origin(),
            alice_h160_address
        ));

        System::assert_last_event(RuntimeEvent::ContractDeployFilter(
            crate::Event::AllowedAddressToCreateInner {
                address: alice_h160_address,
            },
        ));

        // fails when trying to allow the same address twice
        assert_noop!(
            ContractDeployFilter::allow_address_to_create_inner(root_origin(), alice_h160_address),
            Error::<Test>::AlreadyAllowed
        );

        // the address is now in the corresponding list
        assert!(ContractDeployFilter::allowed_addresses_to_create_inner()
            .to_vec()
            .contains(&alice_h160_address));

        // fails with an origin that is not root
        assert_noop!(
            ContractDeployFilter::allow_address_to_create_inner(
                RuntimeOrigin::signed(1),
                bob_h160_address
            ),
            BadOrigin
        );
    });
}

#[test]
fn remove_allowed_address_to_create_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);

        let alice_address: &str = "0x1a642f0e3c3af545e7acbd38b07251b3990914f1";
        let bob_address: &str = "0x2a642f0e3c3af545e7acbd38b07251b3990914f1";

        let alice_h160_address = H160::from_str(alice_address).unwrap();
        let bob_h160_address = H160::from_str(bob_address).unwrap();

        assert_ok!(ContractDeployFilter::allow_address_to_create(
            root_origin(),
            alice_h160_address
        ));

        assert_ok!(ContractDeployFilter::allow_address_to_create(
            root_origin(),
            bob_h160_address
        ));

        assert!(ContractDeployFilter::allowed_addresses_to_create()
            .to_vec()
            .contains(&alice_h160_address));
        assert!(ContractDeployFilter::allowed_addresses_to_create()
            .to_vec()
            .contains(&bob_h160_address));

        assert_ok!(ContractDeployFilter::remove_allowed_address_to_create(
            root_origin(),
            alice_h160_address
        ));

        System::assert_last_event(RuntimeEvent::ContractDeployFilter(
            crate::Event::RemovedAllowedAddressToCreate {
                address: alice_h160_address,
            },
        ));

        // Alice is not allowed anymore
        assert!(!ContractDeployFilter::allowed_addresses_to_create()
            .to_vec()
            .contains(&alice_h160_address));

        // Fails when trying to remove an address that is not present in the list
        assert_noop!(
            ContractDeployFilter::remove_allowed_address_to_create(
                root_origin(),
                alice_h160_address
            ),
            Error::<Test>::NotPresentInAllowedAddresses
        );

        // Fails when trying to remove without root
        assert_noop!(
            ContractDeployFilter::remove_allowed_address_to_create(
                RuntimeOrigin::signed(1),
                bob_h160_address
            ),
            BadOrigin
        );
    });
}

#[test]
fn remove_allowed_address_to_create_inner_works() {
    new_test_ext().execute_with(|| {
        initialize_to_block(1);

        let alice_address: &str = "0x1a642f0e3c3af545e7acbd38b07251b3990914f1";
        let bob_address: &str = "0x2a642f0e3c3af545e7acbd38b07251b3990914f1";

        let alice_h160_address = H160::from_str(alice_address).unwrap();
        let bob_h160_address = H160::from_str(bob_address).unwrap();

        assert_ok!(ContractDeployFilter::allow_address_to_create_inner(
            root_origin(),
            alice_h160_address
        ));

        assert_ok!(ContractDeployFilter::allow_address_to_create_inner(
            root_origin(),
            bob_h160_address
        ));

        assert!(ContractDeployFilter::allowed_addresses_to_create_inner()
            .to_vec()
            .contains(&alice_h160_address));
        assert!(ContractDeployFilter::allowed_addresses_to_create_inner()
            .to_vec()
            .contains(&bob_h160_address));

        assert_ok!(
            ContractDeployFilter::remove_allowed_address_to_create_inner(
                root_origin(),
                alice_h160_address
            )
        );

        System::assert_last_event(RuntimeEvent::ContractDeployFilter(
            crate::Event::RemovedAllowedAddressToCreateInner {
                address: alice_h160_address,
            },
        ));

        // Alice is not allowed anymore
        assert!(!ContractDeployFilter::allowed_addresses_to_create_inner()
            .to_vec()
            .contains(&alice_h160_address));

        // Fails when trying to remove an address that is not present in the list
        assert_noop!(
            ContractDeployFilter::remove_allowed_address_to_create_inner(
                root_origin(),
                alice_h160_address
            ),
            Error::<Test>::NotPresentInAllowedAddresses
        );

        // Fails when trying to remove without root
        assert_noop!(
            ContractDeployFilter::remove_allowed_address_to_create_inner(
                RuntimeOrigin::signed(1),
                bob_h160_address
            ),
            BadOrigin
        );
    });
}

#[test]
fn max_allowed_create_limit_works() {
    new_test_ext().execute_with(|| {
        let limit = <Test as Config>::MaxAllowedCreate::get();

        // MaxAllowedCreate: u32 = 20
        for address in 1..=limit + 1 {
            if address < limit + 1 {
                assert_ok!(ContractDeployFilter::allow_address_to_create(
                    root_origin(),
                    H160::from_low_u64_be(address.into())
                ));
            } else {
                assert_noop!(
                    ContractDeployFilter::allow_address_to_create(
                        root_origin(),
                        H160::from_low_u64_be(address.into())
                    ),
                    Error::<Test>::TooManyAllowedAddresses
                );
            }
        }
        assert_eq!(
            ContractDeployFilter::allowed_addresses_to_create().len(),
            limit as usize
        );
    });
}

#[test]
fn max_allowed_create_inner_limit_works() {
    new_test_ext().execute_with(|| {
        let limit = <Test as Config>::MaxAllowedCreateInner::get();

        // MaxAllowedCreateInner: u32 = 20
        for address in 1..=limit + 1 {
            if address < limit + 1 {
                assert_ok!(ContractDeployFilter::allow_address_to_create_inner(
                    root_origin(),
                    H160::from_low_u64_be(address.into())
                ));
            } else {
                assert_noop!(
                    ContractDeployFilter::allow_address_to_create_inner(
                        root_origin(),
                        H160::from_low_u64_be(address.into())
                    ),
                    Error::<Test>::TooManyAllowedAddresses
                );
            }
        }
        assert_eq!(
            ContractDeployFilter::allowed_addresses_to_create_inner().len(),
            limit as usize
        );
    });
}
