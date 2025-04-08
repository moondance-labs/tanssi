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

use emulated_integration_tests_common::build_genesis_storage;
use sp_core::storage::Storage;
use {
    cumulus_primitives_core::Junctions::X1,
    tanssi_emulated_integration_tests_common::accounts::{ALICE, BOB},
    xcm::prelude::*,
    xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    xcm_executor::traits::ConvertLocation,
};

pub fn genesis() -> Storage {
    let genesis_config = dancebox_runtime::RuntimeGenesisConfig {
        balances: dancebox_runtime::BalancesConfig {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (
                    dancebox_runtime::AccountId::from(ALICE),
                    210_000 * dancebox_runtime::UNIT,
                ),
                (
                    dancebox_runtime::AccountId::from(BOB),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // Give some balance to the relay chain account
                (
                    ParentIsPreset::<dancebox_runtime::AccountId>::convert_location(
                        &Location::parent(),
                    )
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // And to sovereigns
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2001u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2002u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
            ],
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig {
            parachain_id: 2000u32.into(),
            ..Default::default()
        },
        polkadot_xcm: dancebox_runtime::PolkadotXcmConfig {
            safe_xcm_version: 3.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut storage =
        build_genesis_storage(&genesis_config, dancebox_runtime::WASM_BINARY.unwrap());

    storage
        .top
        .insert(b"__mock_is_xcm_test".to_vec(), b"1".to_vec());

    storage
}
