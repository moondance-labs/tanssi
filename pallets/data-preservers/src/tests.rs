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
    std::collections::BTreeSet,
};

const ALICE: u64 = 1;
const BOB: u64 = 2;
const CHARLIE: u64 = 3;

fn profile_deposit(profile: &Profile<Test>) -> BalanceOf<Test> {
    <Test as Config>::ProfileDeposit::compute_deposit(profile)
        .expect("compute_deposit shouldn't fail")
}

macro_rules! bset {
    ( $($value:expr),* $(,)? ) => {
        {
            let mut set = BoundedBTreeSet::new();
            $(
                set.try_insert($value).expect("max bound reached");
            )*
            set
        }
    }
}

macro_rules! set {
    ( $($value:expr),* $(,)? ) => {
        {
            let mut set = BTreeSet::new();
            $(
                set.insert($value);
            )*
            set
        }
    }
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
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408); // 1_000 base deposit + 51 * 8 bytes deposit

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );

                assert_eq!(NextProfileId::<Test>::get(), 1);

                assert_eq!(
                    events(),
                    vec![Event::ProfileCreated {
                        account: ALICE,
                        profile_id: 0,
                        deposit,
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
                    assignment_request: ProviderRequest::Free,
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
                    assignment_request: ProviderRequest::Free,
                };

                // Set some profile at next id. (this shouldn't occur but we protect from it
                // anyway)
                Profiles::<Test>::insert(
                    0,
                    RegisteredProfile {
                        account: ALICE,
                        deposit: 0,
                        profile: profile.clone(),
                        assignment: None,
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
                    assignment_request: ProviderRequest::Free,
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
                        profile,
                        assignment: None,
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
                    assignment_request: ProviderRequest::Free,
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
                    assignment_request: ProviderRequest::Free,
                    mode: ProfileMode::Bootnode,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
                };

                let deposit2 = profile_deposit(&profile2);
                assert_eq!(deposit2, 1_765);
                assert!(deposit < deposit2);

                assert_ok!(DataPreservers::update_profile(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    profile2.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: deposit2,
                        profile: profile2,
                        assignment: None,
                    })
                );

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit,
                        },
                        Event::ProfileUpdated {
                            profile_id: 0,
                            old_deposit: deposit,
                            new_deposit: deposit2,
                        }
                    ]
                );
            });
    }

    #[test]
    fn can_decrease_deposit() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_765);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    assignment_request: ProviderRequest::Free,
                    mode: ProfileMode::Bootnode,
                };

                let deposit2 = profile_deposit(&profile2);
                assert_eq!(deposit2, 1408);
                assert!(deposit2 < deposit);

                assert_ok!(DataPreservers::update_profile(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    profile2.clone(),
                ));

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: ALICE,
                        deposit: deposit2,
                        profile: profile2,
                        assignment: None,
                    })
                );

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit,
                        },
                        Event::ProfileUpdated {
                            profile_id: 0,
                            old_deposit: deposit,
                            new_deposit: deposit2,
                        }
                    ]
                );
            });
    }

    #[test]
    fn unknown_profile_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
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
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
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
            .with_balances(vec![(ALICE, 1_410)]) // `profile`
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
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
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
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
                        profile: profile2,
                        assignment: None,
                    })
                );

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: ALICE,
                            profile_id: 0,
                            deposit,
                        },
                        Event::ProfileUpdated {
                            profile_id: 0,
                            old_deposit: deposit,
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
                    assignment_request: ProviderRequest::Free,
                };

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(ALICE),
                    profile.clone(),
                ));

                let profile2 = Profile {
                    url: b"test2".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Rpc {
                        supports_ethereum_rpcs: false,
                    },
                    assignment_request: ProviderRequest::Free,
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
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

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
                            deposit,
                        },
                        Event::ProfileDeleted {
                            profile_id: 0,
                            released_deposit: deposit,
                        }
                    ]
                );
            });
    }

    #[test]
    fn unknown_profile_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
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
            .with_balances(vec![(ALICE, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
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
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

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
                            deposit,
                        },
                        Event::ProfileDeleted {
                            profile_id: 0,
                            released_deposit: deposit,
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
                    assignment_request: ProviderRequest::Free,
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

    #[test]
    fn cant_delete_assigned_profile() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                let para_id = ParaId::from(1002);
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_noop!(
                    DataPreservers::delete_profile(RuntimeOrigin::signed(BOB), 0,),
                    Error::<Test>::CantDeleteAssignedProfile
                );
            });
    }

    #[test]
    fn cant_force_delete_assigned_profile() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };

                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                let para_id = ParaId::from(1002);
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_noop!(
                    DataPreservers::force_delete_profile(RuntimeOrigin::root(), 0),
                    Error::<Test>::CantDeleteAssignedProfile
                );
            });
    }
}

mod start_assignment {
    use super::*;

    #[test]
    fn start_assignment_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert_eq!(Assignments::<Test>::get(para_id).into_inner(), set![0]);

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: Some((para_id, AssignmentWitness::Free)),
                    })
                );
            });
    }

    #[test]
    fn start_assignment_works_with_payment() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2_224);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::SomeKindOfPayment { extra: 42 }
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert_eq!(Assignments::<Test>::get(para_id), set![0]);

                let payed = 1337 + 42;

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: Some((
                            para_id,
                            AssignmentWitness::SomeKindOfPayment {
                                payed,
                                payer: ALICE
                            }
                        )),
                    })
                );

                assert_eq!(Balances::free_balance(ALICE), 1_000_000_000_000 - payed);
                assert_eq!(
                    Balances::free_balance(BOB),
                    1_000_000_000_000 - deposit + payed
                );
            });
    }

    #[test]
    fn start_assignment_already_assigned() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2_224);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::SomeKindOfPayment { extra: 42 }
                ));

                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        para_id,
                        AssignerParameter::SomeKindOfPayment { extra: 42 }
                    ),
                    Error::<Test>::ProfileAlreadyAssigned
                );
            });
    }

    #[test]
    fn start_assignment_not_whitelisted_para_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::Whitelist(bset![ParaId::from(42)]),
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2479);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });

                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        para_id,
                        AssignerParameter::SomeKindOfPayment { extra: 42 }
                    ),
                    Error::<Test>::ProfileIsNotElligibleForParaId
                );
            });
    }

    #[test]
    fn no_profile() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let para_id = ParaId::from(1002);

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });

                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        para_id,
                        AssignerParameter::Free
                    ),
                    Error::<Test>::UnknownProfileId
                );
            });
    }

    #[test]
    fn not_para_manager() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(BOB),
                        0,
                        para_id,
                        AssignerParameter::Free
                    ),
                    DispatchError::BadOrigin
                );
            });
    }

    #[test]
    fn request_param_mismatch() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        para_id,
                        AssignerParameter::SomeKindOfPayment { extra: 42 }
                    ),
                    Error::<Test>::AssignmentPaymentRequestParameterMismatch
                );
            });
    }

    #[test]
    fn payment_failure() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_300), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2_224);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_noop!(
                    DataPreservers::start_assignment(
                        RuntimeOrigin::signed(ALICE),
                        0,
                        para_id,
                        AssignerParameter::SomeKindOfPayment { extra: 42 }
                    ),
                    TokenError::FundsUnavailable
                );
            });
    }
}

mod stop_assignment {
    use super::*;

    #[test]
    fn stop_assignment_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_ok!(DataPreservers::stop_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        },
                        Event::AssignmentStopped {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert!(Assignments::<Test>::get(para_id).is_empty());

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );
            });
    }

    #[test]
    fn stop_assignment_works_after_para_deregistration() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                DataPreservers::para_deregistered(para_id);
                assert!(Assignments::<Test>::get(para_id).is_empty());

                assert_ok!(DataPreservers::stop_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        },
                        Event::AssignmentStopped {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );
            });
    }

    #[test]
    fn stop_assignment_can_be_called_by_profile_owner() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_ok!(DataPreservers::stop_assignment(
                    RuntimeOrigin::signed(BOB),
                    0,
                    para_id,
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        },
                        Event::AssignmentStopped {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert!(Assignments::<Test>::get(para_id).is_empty());

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );
            });
    }

    #[test]
    fn stop_assignment_can_be_called_by_root() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_ok!(DataPreservers::stop_assignment(
                    RuntimeOrigin::root(),
                    0,
                    para_id,
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        },
                        Event::AssignmentStopped {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert!(Assignments::<Test>::get(para_id).is_empty());

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );
            });
    }

    #[test]
    fn stop_assignment_cannot_be_called_by_non_manager() {
        ExtBuilder::default()
            .with_balances(vec![
                (ALICE, 1_000_000_000_000),
                (BOB, 1_000_000_000_000),
                (CHARLIE, 1_000_000_000_000),
            ])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_noop!(
                    DataPreservers::stop_assignment(RuntimeOrigin::signed(CHARLIE), 0, para_id,),
                    DispatchError::BadOrigin
                );
            });
    }

    #[test]
    fn stop_assignment_works_with_payment() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2_224);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::SomeKindOfPayment { extra: 42 }
                ));

                assert_ok!(DataPreservers::stop_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        },
                        Event::AssignmentStopped {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert!(Assignments::<Test>::get(para_id).is_empty());

                let payed = (1337 + 42) * 2;

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: None,
                    })
                );

                assert_eq!(Balances::free_balance(ALICE), 1_000_000_000_000 - payed);
                assert_eq!(
                    Balances::free_balance(BOB),
                    1_000_000_000_000 - deposit + payed
                );
            });
    }

    #[test]
    fn unknown_profile_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_noop!(
                    DataPreservers::stop_assignment(RuntimeOrigin::signed(ALICE), 1, para_id,),
                    Error::<Test>::UnknownProfileId,
                );
            });
    }

    #[test]
    fn profile_not_assigned() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });

                assert_noop!(
                    DataPreservers::stop_assignment(RuntimeOrigin::signed(ALICE), 0, para_id,),
                    Error::<Test>::ProfileNotAssigned,
                );
            });
    }

    #[test]
    fn wrong_para_id() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);
                let para_id2 = ParaId::from(1003);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                    m.container_chain_managers.insert(para_id2, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_noop!(
                    DataPreservers::stop_assignment(RuntimeOrigin::signed(ALICE), 0, para_id2,),
                    Error::<Test>::WrongParaId
                );
            });
    }

    #[test]
    fn payment_failure() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_500), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::SomeKindOfPayment { amount: 1337 },
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 2_224);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::SomeKindOfPayment { extra: 42 }
                ));

                assert_noop!(
                    DataPreservers::stop_assignment(RuntimeOrigin::signed(ALICE), 0, para_id,),
                    TokenError::FundsUnavailable
                );
            });
    }
}

mod force_start_assignment {
    use super::*;

    #[test]
    fn force_start_assignment_works() {
        ExtBuilder::default()
            .with_balances(vec![(ALICE, 1_000_000_000_000), (BOB, 1_000_000_000_000)])
            .build()
            .execute_with(|| {
                let profile = Profile {
                    url: b"test".to_vec().try_into().unwrap(),
                    para_ids: ParaIdsFilter::AnyParaId,
                    mode: ProfileMode::Bootnode,
                    assignment_request: ProviderRequest::Free,
                };
                let deposit = profile_deposit(&profile);
                assert_eq!(deposit, 1_408);

                let para_id = ParaId::from(1002);

                assert_ok!(DataPreservers::create_profile(
                    RuntimeOrigin::signed(BOB),
                    profile.clone(),
                ));

                // Set ALICE as manager of para 1002
                MockData::mutate(|m| {
                    m.container_chain_managers.insert(para_id, Some(ALICE));
                });
                assert_ok!(DataPreservers::start_assignment(
                    RuntimeOrigin::signed(ALICE),
                    0,
                    para_id,
                    AssignerParameter::Free
                ));

                assert_eq!(
                    events(),
                    vec![
                        Event::ProfileCreated {
                            account: BOB,
                            profile_id: 0,
                            deposit,
                        },
                        Event::AssignmentStarted {
                            profile_id: 0,
                            para_id,
                        }
                    ]
                );

                assert_eq!(Assignments::<Test>::get(para_id).into_inner(), set![0]);

                assert_eq!(
                    Profiles::<Test>::get(0),
                    Some(RegisteredProfile {
                        account: BOB,
                        deposit,
                        profile,
                        assignment: Some((para_id, AssignmentWitness::Free)),
                    })
                );
            });
    }
}
