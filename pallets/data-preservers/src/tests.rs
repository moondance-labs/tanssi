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
    sp_runtime::TokenError,
};

const ALICE: u64 = 1;
const BOB: u64 = 2;

#[test]
fn set_boot_nodes_bad_origin() {
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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
    ExtBuilder::default().build().execute_with(|| {
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

mod create_profile {
    use super::*;

    #[test]
    fn create_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: 1_357, // 1_000 base deposit + 51 * 7 bytes deposit
                        profile
                    })
                );

                assert_eq!(NextProfileId::<Test>::get(), 1);

                assert_eq!(
                    events(),
                    vec![Event::ProfileCreated {
                        account: ALICE,
                        profile_id: 0,
                        deposit: 1_357,
                    }]
                );
            });
    }

    #[test]
    fn insufficient_balance_for_deposit() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_356)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_noop!(
                    DataPreservers::create_profile(RuntimeOrigin::signed(ALICE), profile.clone(),),
                    TokenError::FundsUnavailable
                );

                assert_eq!(Profiles::<Test>::get(0), None);
                assert_eq!(NextProfileId::<Test>::get(), 0);
                assert_eq!(events(), vec![],);
            });
    }

    #[test]
    fn protection_for_existing_profile() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                // Set some profile at next id. (this shouldn't occur but we protect from it
                // anyway)
                Profiles::<Test>::insert(
                    0,
                    RegisteredProfile {
                        account: ALICE,
                        deposit: 0,
                        profile: profile.clone(),
                    },
                );

                assert_noop!(
                    DataPreservers::create_profile(RuntimeOrigin::signed(ALICE), profile.clone(),),
                    Error::<Test>::NextProfileIdShouldBeAvailable
                );
            });
    }

    #[test]
    fn forced_create_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::force_create_profile(
                    RuntimeOrigin::root(),
                    profile.clone(),
                    ALICE,
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: 0, // no deposit when forced
                        profile
                    })
                );

                assert_eq!(NextProfileId::<Test>::get(), 1);

                assert_eq!(
                    events(),
                    vec![Event::ProfileCreated {
                        account: ALICE,
                        profile_id: 0,
                        deposit: 0, // no deposit when forced
                    }]
                );
            });
    }

    #[test]
    fn forced_create_profile_filter() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_noop!(
                    DataPreservers::force_create_profile(
                        RuntimeOrigin::signed(BOB),
                        profile.clone(),
                        ALICE,
                    ),
                    sp_runtime::DispatchError::BadOrigin
                );
            });
    }
}

mod update_profile {
    use super::*;

    #[test]
    fn update_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_ok!(DataPreservers::update_profile(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    profile2.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: 1_714, // 1_000 base deposit + 51 * 14 bytes deposit
                        profile: profile2
                    })
                );

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit: 1_357,
                        },
                        Event::ProfileUpdated {
                            profile_id: 0,
                            old_deposit: 1_357,
                            new_deposit: 1_714,
                        }
                    ]
                );
            });
    }

    #[test]
    fn unknown_profile_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_400)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_noop!(
                    DataPreservers::update_profile(
                        RuntimeOrigin::signed(ALICE),
                        1, // wrong profile id
                        profile2.clone(),
                    ),
                    Error::<Test>::UnknownProfileId
                );
            });
    }

    #[test]
    fn wrong_user() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_400)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_noop!(
                    DataPreservers::update_profile(
                        RuntimeOrigin::signed(BOB), // not the profile's owner
                        0,
                        profile2.clone(),
                    ),
                    sp_runtime::DispatchError::BadOrigin,
                );
            });
    }

    #[test]
    fn insufficient_balance_for_new_deposit() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_400)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_noop!(
                    DataPreservers::update_profile(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        profile2.clone(),
                    ),
                    TokenError::FundsUnavailable
                );
            });
    }

    #[test]
    fn forced_update_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_ok!(DataPreservers::force_update_profile(
                    RuntimeOrigin::root(),
                    0,
                    profile2.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: 0, // forced update release deposit
                        profile: profile2
                    })
                );

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit: 1_357,
                        },
                        Event::ProfileUpdated {
                            profile_id: 0,
                            old_deposit: 1_357,
                            new_deposit: 0,
                        }
                    ]
                );
            });
    }

    #[test]
    fn forced_update_profile_filter() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(vec![ParaId::from(42)].try_into().unwrap()),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                };

                assert_noop!(
                    DataPreservers::force_update_profile(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        profile2.clone(),
                    ),
                    sp_runtime::DispatchError::BadOrigin,
                );
            });
    }
}

mod delete_profile {
    use super::*;

    #[test]
    fn delete_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_ok!(DataPreservers::delete_profile(
                    RuntimeOrigin::signed(ALICE),
                    0,
                ));

                assert_eq!(Profiles::<Test>::get(0), None);

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit: 1_357,
                        },
                        Event::ProfileDeleted {
                            profile_id: 0,
                            released_deposit: 1_357,
                        }
                    ]
                );
            });
    }

    #[test]
    fn unknown_profile_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_400)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_noop!(
                    DataPreservers::delete_profile(
                        RuntimeOrigin::signed(ALICE),
                        1, // wrong profile id
                    ),
                    Error::<Test>::UnknownProfileId
                );
            });
    }

    #[test]
    fn wrong_user() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_400)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_noop!(
                    DataPreservers::delete_profile(
                        RuntimeOrigin::signed(BOB), // not the profile's owner
                        0,
                    ),
                    sp_runtime::DispatchError::BadOrigin,
                );
            });
    }

    #[test]
    fn forced_delete_profile_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_ok!(DataPreservers::force_delete_profile(
                    RuntimeOrigin::root(),
                    0,
                ));

                assert_eq!(Profiles::<Test>::get(0), None);

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit: 1_357,
                        },
                        Event::ProfileDeleted {
                            profile_id: 0,
                            released_deposit: 1_357,
                        }
                    ]
                );
            });
    }

    #[test]
    fn forced_delete_profile_filter() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_noop!(
                    DataPreservers::force_delete_profile(RuntimeOrigin::signed(ALICE), 0),
                    sp_runtime::DispatchError::BadOrigin,
                );
            });
    }
}
