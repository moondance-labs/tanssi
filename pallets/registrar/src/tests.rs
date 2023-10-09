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
    crate::{mock::*, Error, Event},
    frame_support::{assert_noop, assert_ok, dispatch::GetDispatchInfo, BoundedVec},
    parity_scale_codec::Encode,
    sp_core::Get,
    sp_runtime::DispatchError,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
    tp_traits::ParaId,
};

const ALICE: u64 = 1;
const BOB: u64 = 2;

#[test]
fn register_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdValidForCollating { para_id: 42.into() }.into());

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn register_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_noop!(
            ParaRegistrar::register(
                RuntimeOrigin::signed(ALICE),
                42.into(),
                empty_genesis_data()
            ),
            Error::<Test>::ParaIdAlreadyRegistered
        );
    });
}

#[test]
fn register_para_id_42_genesis_data_size_too_big() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let genesis_data = ContainerChainGenesisData {
            storage: vec![(vec![], vec![0; 5_000_000]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::signed(ALICE), 42.into(), genesis_data,),
            Error::<Test>::GenesisDataTooBig,
        );
    });
}

#[test]
fn deregister_para_id_from_empty_list() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_42() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
    });
}

#[test]
fn deregister_para_id_42_after_session_changes() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());
    });
}

#[test]
fn deregister_para_id_42_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn deregister_para_id_removes_genesis_data() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let genesis_data = ContainerChainGenesisData {
            storage: vec![(b"key".to_vec(), b"value".to_vec()).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default(),
        };
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            genesis_data.clone(),
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );
        assert_eq!(
            ParaRegistrar::para_genesis_data(ParaId::from(42)),
            Some(genesis_data),
        );

        // Assert after two sessions it goes to the non-pending
        ParaRegistrar::initializer_on_new_session(&2);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdDeregistered { para_id: 42.into() }.into());

        // Genesis data has been deleted
        // TODO: it should probably not be deleted until the next session change when the
        // para id is actually deregistered
        assert_eq!(ParaRegistrar::para_genesis_data(ParaId::from(42)), None,);
    });
}

#[test]
fn register_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::register(RuntimeOrigin::root(), 42.into(), empty_genesis_data()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn deregister_para_id_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::deregister(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn mark_valid_for_collating_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn pause_para_id_42_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Set boot nodes to check their existence later on
        assert_ok!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
            )
        );

        // Enable the container-chain for the first time
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Pause the container-chain
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Check the container-chain is not in PendingParaIds
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![]).unwrap())]
        );

        // Assert that the ParaIdPaused event was emitted
        System::assert_last_event(Event::ParaIdPaused { para_id: 42.into() }.into());

        // Check boot nodes and genesis data were not removed
        assert!(!ParaRegistrar::boot_nodes(ParaId::from(42)).is_empty());
        assert!(ParaRegistrar::para_genesis_data(ParaId::from(42)).is_some());

        // Check the container chain was not selected for the next period
        ParaRegistrar::initializer_on_new_session(&4);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
    });
}

#[test]
fn pause_para_id_42_twice_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        // Enable the container-chain for collating
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(
            ParaRegistrar::pending_registered_para_ids(),
            vec![(2u32, BoundedVec::try_from(vec![42u32.into()]).unwrap())]
        );

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);

        // Pause the container-chain
        assert_ok!(ParaRegistrar::pause_container_chain(
            RuntimeOrigin::root(),
            42.into(),
        ));

        // Try to pause again
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdAlreadyPaused
        );
    });
}

#[test]
fn pause_para_id_42_fails_not_registered() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Try to pause
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::root(), 42.into()),
            Error::<Test>::ParaIdNotRegistered
        );
    });
}

#[test]
fn pause_container_chain_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::pause_container_chain(RuntimeOrigin::signed(1), 42.into()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn genesis_loads_para_ids() {
    new_test_ext_with_genesis(vec![
        (1.into(), empty_genesis_data(), vec![]),
        (2.into(), empty_genesis_data(), vec![]),
        (3.into(), empty_genesis_data(), vec![]),
        (4.into(), empty_genesis_data(), vec![]),
    ])
    .execute_with(|| {
        System::set_block_number(1);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![1.into(), 2.into(), 3.into(), 4.into()]
        );
    });
}

#[test]
fn genesis_sorts_para_ids() {
    new_test_ext_with_genesis(vec![
        (4.into(), empty_genesis_data(), vec![]),
        (2.into(), empty_genesis_data(), vec![]),
        (3.into(), empty_genesis_data(), vec![]),
        (1.into(), empty_genesis_data(), vec![]),
    ])
    .execute_with(|| {
        System::set_block_number(1);
        assert_eq!(
            ParaRegistrar::registered_para_ids(),
            vec![1.into(), 2.into(), 3.into(), 4.into()]
        );
    });
}

#[test]
#[should_panic = "Duplicate para_id: 2"]
fn genesis_error_on_duplicate() {
    new_test_ext_with_genesis(vec![
        (2.into(), empty_genesis_data(), vec![]),
        (3.into(), empty_genesis_data(), vec![]),
        (4.into(), empty_genesis_data(), vec![]),
        (2.into(), empty_genesis_data(), vec![]),
    ])
    .execute_with(|| {
        System::set_block_number(1);
    });
}

#[test]
#[should_panic = "genesis data for para_id 2 is too large: 5000024 bytes"]
fn genesis_error_genesis_data_size_too_big() {
    let genesis_data = ContainerChainGenesisData {
        storage: vec![(vec![], vec![0; 5_000_000]).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    };
    new_test_ext_with_genesis(vec![(2.into(), genesis_data, vec![])]).execute_with(|| {
        System::set_block_number(1);
    });
}

#[test]
fn register_without_mark_valid_for_collating() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);

        // Assert after two sessions registered para ids are still empty
        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
    });
}

#[test]
fn mark_valid_for_collating_twice() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 42.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_invalid_para_id() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 1.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn mark_valid_for_collating_already_valid_para_id() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        // Assert that the correct event was deposited
        System::assert_last_event(Event::ParaIdRegistered { para_id: 42.into() }.into());
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));

        ParaRegistrar::initializer_on_new_session(&2);
        assert_eq!(ParaRegistrar::registered_para_ids(), vec![42.into()]);
        assert_eq!(ParaRegistrar::pending_registered_para_ids(), vec![]);
        assert_noop!(
            ParaRegistrar::mark_valid_for_collating(RuntimeOrigin::root(), 42.into(),),
            Error::<Test>::ParaIdNotInPendingVerification
        );
    });
}

#[test]
fn deregister_returns_bond() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let bond = DepositAmount::get();
        let balance_before = Balances::free_balance(ALICE);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        assert_ok!(ParaRegistrar::mark_valid_for_collating(
            RuntimeOrigin::root(),
            42.into(),
        ));
        assert_eq!(Balances::free_balance(ALICE), balance_before - bond);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));

        assert_eq!(Balances::free_balance(ALICE), balance_before);
    });
}

#[test]
fn can_deregister_before_valid_for_collating() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into(),));
    });
}

#[test]
fn set_boot_nodes_bad_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
    );
    });
}

#[test]
fn set_boot_nodes_by_para_id_registrar() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
        ).unwrap();
        assert_ok!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        )
    );
    });
}

#[test]
fn set_boot_nodes_by_invalid_user() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data(),
        ).unwrap();
        assert_noop!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::signed(BOB),
            42.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
    );
    });
}

#[test]
fn set_boot_nodes_bad_para_id() {
    // This is allowed in case we want to set bootnodes before registering the chain
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::root(),
            42.into(),
            boot_nodes.clone(),
        ));
        assert_eq!(ParaRegistrar::boot_nodes(ParaId::from(42)), boot_nodes);
    });
}

#[test]
fn boot_nodes_removed_on_deregister() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ParaRegistrar::register(
            RuntimeOrigin::signed(ALICE),
            42.into(),
            empty_genesis_data()
        ));
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(ParaRegistrar::set_boot_nodes(
            RuntimeOrigin::root(),
            42.into(),
            boot_nodes.clone(),
        ));
        assert_eq!(ParaRegistrar::boot_nodes(ParaId::from(42)), boot_nodes);

        assert_ok!(ParaRegistrar::deregister(RuntimeOrigin::root(), 42.into()));
        assert_eq!(ParaRegistrar::boot_nodes(ParaId::from(42)), vec![]);
    });
}

#[test]
fn weights_assigned_to_extrinsics_are_correct() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            crate::Call::<Test>::register {
                para_id: 42.into(),
                genesis_data: empty_genesis_data()
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::register(
                empty_genesis_data().encoded_size() as u32,
                <Test as crate::Config>::MaxLengthParaIds::get(),
                0
            )
        );

        assert_eq!(
            crate::Call::<Test>::deregister { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::deregister(
                <Test as crate::Config>::MaxGenesisDataSize::get(),
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );

        assert_eq!(
            crate::Call::<Test>::mark_valid_for_collating { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::mark_valid_for_collating(
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );

        assert_eq!(
            crate::Call::<Test>::set_boot_nodes {
                para_id: 42.into(),
                boot_nodes: vec![].try_into().unwrap()
            }
            .get_dispatch_info()
            .weight,
            <() as crate::weights::WeightInfo>::set_boot_nodes(
                <Test as crate::Config>::MaxBootNodeUrlLen::get(),
                0
            )
        );

        assert_eq!(
            crate::Call::<Test>::pause_container_chain { para_id: 42.into() }
                .get_dispatch_info()
                .weight,
            <() as crate::weights::WeightInfo>::pause_container_chain(
                <Test as crate::Config>::MaxLengthParaIds::get()
            )
        );
    });
}
