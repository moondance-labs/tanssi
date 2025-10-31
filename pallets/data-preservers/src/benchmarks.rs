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

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use {
    crate::{
        AssignmentProcessor, Assignments, Call, Config, NodeType, Pallet, ParaIdsFilter, Profile,
        Profiles, RegisteredProfile,
    },
    alloc::{collections::btree_set::BTreeSet, vec},
    frame_benchmarking::v2::*,
    frame_support::{
        traits::{
            fungible::{Inspect, Mutate},
            EnsureOrigin, EnsureOriginWithArg,
        },
        BoundedBTreeSet, BoundedVec,
    },
    frame_system::RawOrigin,
    sp_runtime::traits::{Get, Zero},
    tp_traits::{ParaId, StorageDeposit},
};

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
    () => { BTreeSet::new() };
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

const SEED: u32 = 0;

fn create_funded_user<T: Config>(string: &'static str, n: u32, balance_factor: u32) -> T::AccountId
where
    T::Currency: Mutate<T::AccountId>,
{
    let user = account(string, n, SEED);
    let balance = <T::Currency>::minimum_balance() * balance_factor.into();
    let _ = <T::Currency>::set_balance(&user, balance);
    user
}

#[benchmarks(
    where T::Currency: Mutate<T::AccountId>, T::ProfileId: Zero
)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_profile(x: Linear<1, 200>, y: Linear<1, 10>) {
        // x: url len, y: para ids len
        let url = BoundedVec::try_from(vec![b'A'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..y {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let deposit = T::ProfileDeposit::compute_deposit(&profile).expect("deposit to be computed");

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        #[extrinsic_call]
        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()), profile.clone());

        assert_eq!(
            Profiles::<T>::get(T::ProfileId::zero()),
            Some(RegisteredProfile {
                account: caller,
                deposit,
                profile,
                assignment: None,
            })
        );
    }

    #[benchmark]
    fn force_create_profile(x: Linear<1, 200>, y: Linear<1, 10>) {
        // x: url len, y: para ids len
        let url = BoundedVec::try_from(vec![b'A'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..y {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let owner = create_funded_user::<T>("owner", 1, 1_000_000_000u32);
        let origin_force = T::ForceSetProfileOrigin::try_successful_origin()
            .expect("failed to create ForceSetProfileOrigin");

        #[extrinsic_call]
        Pallet::<T>::force_create_profile(
            origin_force as T::RuntimeOrigin,
            profile.clone(),
            owner.clone(),
        );

        assert_eq!(
            Profiles::<T>::get(T::ProfileId::zero()),
            Some(RegisteredProfile {
                account: owner,
                deposit: 0u32.into(),
                profile,
                assignment: None,
            })
        );
    }

    #[benchmark]
    fn update_profile(x: Linear<1, 200>, y: Linear<1, 10>) {
        // x: url len, y: para ids len
        let url = BoundedVec::try_from(vec![b'A'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..2 {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        // x: url len, y: para ids len
        let url = BoundedVec::try_from(vec![b'B'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..y {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let deposit = T::ProfileDeposit::compute_deposit(&profile).expect("deposit to be computed");

        #[extrinsic_call]
        Pallet::<T>::update_profile(
            RawOrigin::Signed(caller.clone()),
            T::ProfileId::zero(),
            profile.clone(),
        );

        assert_eq!(
            Profiles::<T>::get(T::ProfileId::zero()),
            Some(RegisteredProfile {
                account: caller,
                deposit,
                profile,
                assignment: None,
            })
        );
    }

    #[benchmark]
    fn force_update_profile(x: Linear<1, 200>, y: Linear<1, 10>) {
        let url = BoundedVec::try_from(vec![b'A'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..2 {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        // x: url len, y: para ids len
        let url = BoundedVec::try_from(vec![b'B'; x as usize]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..y {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let origin_force = T::ForceSetProfileOrigin::try_successful_origin()
            .expect("failed to create ForceSetProfileOrigin");

        #[extrinsic_call]
        Pallet::<T>::force_update_profile(
            origin_force as T::RuntimeOrigin,
            T::ProfileId::zero(),
            profile.clone(),
        );

        assert_eq!(
            Profiles::<T>::get(T::ProfileId::zero()),
            Some(RegisteredProfile {
                account: caller,
                deposit: 0u32.into(),
                profile,
                assignment: None,
            })
        );
    }

    #[benchmark]
    fn delete_profile() {
        let url = BoundedVec::try_from(vec![b'A'; 10]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..2 {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        #[extrinsic_call]
        Pallet::<T>::delete_profile(RawOrigin::Signed(caller.clone()), T::ProfileId::zero());

        assert_eq!(Profiles::<T>::get(T::ProfileId::zero()), None);
    }

    #[benchmark]
    fn force_delete_profile() {
        let url = BoundedVec::try_from(vec![b'A'; 10]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let mut para_ids = BoundedBTreeSet::new();
        for i in 0..2 {
            para_ids.try_insert(ParaId::from(i)).unwrap();
        }

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(para_ids),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        let origin_force = T::ForceSetProfileOrigin::try_successful_origin()
            .expect("failed to create ForceSetProfileOrigin");

        #[extrinsic_call]
        Pallet::<T>::force_delete_profile(origin_force as T::RuntimeOrigin, T::ProfileId::zero());

        assert_eq!(Profiles::<T>::get(T::ProfileId::zero()), None);
    }

    #[benchmark]
    fn start_assignment() {
        let url = BoundedVec::try_from(vec![b'A'; 10]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        // !!! (Applicable for Dancelight only)
        // The specified ParaId needs to be larger than LOWEST_PUBLIC_ID value in Polkadot SDK.
        // Currently, this value is 2000. We should also avoid setting the value to one of
        // the container chains reserved by root
        let para_id = ParaId::from(2042);

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(bset![para_id]),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        let origin = T::AssignmentOrigin::try_successful_origin(&para_id).unwrap();

        #[extrinsic_call]
        Pallet::<T>::start_assignment(
            origin as T::RuntimeOrigin,
            T::ProfileId::zero(),
            para_id,
            T::AssignmentProcessor::benchmark_assigner_parameter(),
        );

        assert_eq!(
            Assignments::<T>::get(para_id).into_inner(),
            set![T::ProfileId::zero()]
        );
    }

    #[benchmark]
    fn stop_assignment() {
        let url = BoundedVec::try_from(vec![b'A'; 10]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        // !!! (Applicable for Dancelight only)
        // The specified ParaId needs to be larger than LOWEST_PUBLIC_ID value in Polkadot SDK.
        // Currently, this value is 2000. We should also avoid setting the value to one of
        // the container chains reserved by root
        let para_id = ParaId::from(2042);

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(bset![para_id]),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        let origin = T::AssignmentOrigin::try_successful_origin(&para_id).unwrap();

        Pallet::<T>::start_assignment(
            origin.clone() as T::RuntimeOrigin,
            T::ProfileId::zero(),
            para_id,
            T::AssignmentProcessor::benchmark_assigner_parameter(),
        )
        .expect("to assign");

        #[extrinsic_call]
        Pallet::<T>::stop_assignment(origin as T::RuntimeOrigin, T::ProfileId::zero(), para_id);

        assert_eq!(Assignments::<T>::get(para_id).into_inner(), set![]);
    }

    #[benchmark]
    fn force_start_assignment() {
        let url = BoundedVec::try_from(vec![b'A'; 10]).unwrap();
        let urls = BoundedVec::try_from(vec![url]).unwrap();

        let para_id = ParaId::from(42);

        let profile = Profile {
            direct_rpc_urls: urls.clone(),
            proxy_rpc_urls: Default::default(),
            bootnode_url: None,
            para_ids: ParaIdsFilter::Whitelist(bset![para_id]),
            node_type: NodeType::Substrate,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
            additional_info: Default::default(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        #[extrinsic_call]
        Pallet::<T>::force_start_assignment(
            RawOrigin::Root,
            T::ProfileId::zero(),
            para_id,
            T::AssignmentProcessor::benchmark_assignment_witness(),
        );

        assert_eq!(
            Assignments::<T>::get(para_id).into_inner(),
            set![T::ProfileId::zero()]
        );
    }

    #[benchmark]
    fn poke_deposit() {
        // Create initial profile with minimal data (smallest deposit)
        let url = BoundedVec::try_from(vec![b'A'; 1]).unwrap();

        let profile = Profile {
            url,
            para_ids: ParaIdsFilter::AnyParaId,
            mode: ProfileMode::Bootnode,
            assignment_request: T::AssignmentProcessor::benchmark_provider_request(),
        };

        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        Pallet::<T>::create_profile(RawOrigin::Signed(caller.clone()).into(), profile)
            .expect("to create profile");

        // Manually modify the profile to worst case scenario (maximum deposit required)
        let mut reg = Profiles::<T>::get(T::ProfileId::zero()).expect("profile exists");

        // Max URL length
        let max_url_len = T::MaxNodeUrlLen::get() as usize;
        reg.profile.url = BoundedVec::try_from(vec![b'B'; max_url_len]).unwrap();

        // Max para_ids in whitelist
        let max_para_ids_len = T::MaxParaIdsVecLen::get();
        let mut max_para_ids = BoundedBTreeSet::new();
        for i in 0..max_para_ids_len {
            max_para_ids.try_insert(ParaId::from(i)).unwrap();
        }
        reg.profile.para_ids = ParaIdsFilter::Whitelist(max_para_ids);

        Profiles::<T>::insert(T::ProfileId::zero(), reg);

        #[extrinsic_call]
        Pallet::<T>::poke_deposit(RawOrigin::Signed(caller.clone()), T::ProfileId::zero());

        // Verify the deposit was updated
        let updated_reg = Profiles::<T>::get(T::ProfileId::zero()).expect("profile exists");
        let expected_deposit = T::ProfileDeposit::compute_deposit(&updated_reg.profile)
            .expect("deposit to be computed");
        assert_eq!(updated_reg.deposit, expected_deposit);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Test
    );
}
