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
    crate::{mock::*, *},
    frame_support::{assert_noop, assert_ok, pallet_prelude::*},
};

const ALICE: u64 = 1;
const BOB: u64 = 2;

#[test]
fn set_boot_nodes_bad_origin() {
    new_test_ext().execute_with(|| {
        // Para 1001 has no manager, Alice cannot set boot nodes
        assert_noop!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(ALICE),
            1001.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
    );
    });
}

#[test]
fn set_boot_nodes_by_root_no_manager() {
    new_test_ext().execute_with(|| {
        // Para 1001 has no manager, root can set boot nodes
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::root(),
            1001.into(),
            boot_nodes.clone(),
        ));
        assert_eq!(DataPreservers::boot_nodes(ParaId::from(1001)), boot_nodes);
    });
}

#[test]
fn set_boot_nodes_by_root_with_manager() {
    new_test_ext().execute_with(|| {
        // Set ALICE as manager of para 1002
        MockData::mutate(|m| {
            m.container_chain_managers.insert(1002.into(), Some(ALICE));
        });
        // Root can set bootnodes
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::root(),
            1002.into(),
            boot_nodes.clone()
        ));
        assert_eq!(DataPreservers::boot_nodes(ParaId::from(1002)), boot_nodes);
    });
}

#[test]
fn set_boot_nodes_by_para_id_registrar() {
    new_test_ext().execute_with(|| {
        // Set ALICE as manager of para 1002
        MockData::mutate(|m| {
            m.container_chain_managers.insert(1002.into(), Some(ALICE));
        });
        // Alice can set bootnodes
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(ALICE),
            1002.into(),
            boot_nodes.clone(),
        ));
        assert_eq!(DataPreservers::boot_nodes(ParaId::from(1002)), boot_nodes);
    });
}

#[test]
fn set_boot_nodes_by_invalid_user_no_manager() {
    new_test_ext().execute_with(|| {
        // Para 1001 has no manager
        MockData::mutate(|m| {
            m.container_chain_managers.insert(1002.into(), Some(ALICE));
        });
        // Bob cannot set the bootnodes
        assert_noop!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(BOB),
            1001.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
        );
    });
}

#[test]
fn set_boot_nodes_by_invalid_user() {
    new_test_ext().execute_with(|| {
        // Set ALICE as manager of para 1002
        MockData::mutate(|m| {
            m.container_chain_managers.insert(1002.into(), Some(ALICE));
        });
        // Bob cannot set the bootnodes
        assert_noop!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(BOB),
            1002.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
        );

        assert_noop!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(BOB),
            1003.into(),
            vec![
                b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9".to_vec().try_into().unwrap()
            ].try_into().unwrap()
        ),
        DispatchError::BadOrigin
        );
    });
}

#[test]
fn set_boot_nodes_by_invalid_user_bad_para_id() {
    new_test_ext().execute_with(|| {
        // Para 1003 does not exist, only root can set bootnodes
        assert_noop!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::signed(BOB),
            1003.into(),
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
    // Para 1003 does not exist, only root can set bootnodes
    // This is allowed in case we want to set bootnodes before registering the chain
    new_test_ext().execute_with(|| {
        let boot_nodes: BoundedVec<BoundedVec<_, _>, _> = vec![
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .unwrap(),
        ]
        .try_into()
        .unwrap();
        assert_ok!(DataPreservers::set_boot_nodes(
            RuntimeOrigin::root(),
            1003.into(),
            boot_nodes.clone(),
        ));
        assert_eq!(DataPreservers::boot_nodes(ParaId::from(1003)), boot_nodes);
    });
}
