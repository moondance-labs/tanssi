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
    crate::{Call, Config, Pallet},
    frame_benchmarking::{account, benchmarks},
    frame_system::RawOrigin,
    sp_std::vec,
    tp_container_chain_genesis_data::ContainerChainGenesisData,
};
benchmarks! {
    where_clause { where T::SessionIndex: From<u32> }

    register {
        // We make it dependent on the size of the runtime
        let x in 5..3_000_000;
        // ..and on the nummber of parachains already registered
        let y in 1..50;

        let caller: T::AccountId = account("account id", 0u32, 0u32);
        let storage =  ContainerChainGenesisData {
            storage: vec![(vec![], vec![0; x as usize]).into()],
            name: Default::default(),
            id: Default::default(),
            fork_id: Default::default(),
            extensions: Default::default(),
            properties: Default::default()
        };

        for i in 1..y {
            Pallet::<T>::register(RawOrigin::Root.into(), i.into(), storage.clone())?;
        }

    }: _(RawOrigin::Root, Default::default(), storage)
    verify {
       assert!(Pallet::<T>::pending_registered_para_ids().len()>0);

    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
