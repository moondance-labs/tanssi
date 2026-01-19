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

use crate::tests::common::ExtBuilder;
use crate::{xcm_config, Runtime, RuntimeCall, RuntimeEvent, System};
use parity_scale_codec::Encode;
use sp_core::Hasher;
use tanssi_runtime_common::processors::v2::execute_xcm;
use xcm::latest::Error::WeightLimitReached;
use xcm::latest::Instruction::{RefundSurplus, Transact, WithdrawAsset};
use xcm::latest::{
    Asset, AssetId, Fungibility, InstructionError, Junction, Location, OriginKind, Outcome, Weight,
};

#[test]
fn test_execute_xcm() {
    ExtBuilder::default().build().execute_with(|| {
        // Note that there is no way to test Outcome::Error variant as the current xcm executor
        // uses XcmWeighter to only test if the xcm has enough weight.
        // Since we explicitly check the weight (reason being as the weight checking is implementation
        // detail of xcm executor, and we can't assume anything since we work with trait) before we pass
        // xcm to executor that logic will never be triggerred.

        type Hashing = <Runtime as frame_system::Config>::Hashing;

        // Scenario 1: execute xcm that uses more weight than this limit
        let remark = vec![1];
        assert!(matches!(
            execute_xcm::<
                Runtime,
                xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
                <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            >(
                Location::here(),
                Weight::from_all(200),
                vec![Transact {
                    origin_kind: OriginKind::Native,
                    fallback_max_weight: None,
                    call: RuntimeCall::System(frame_system::Call::remark_with_event {
                        remark: remark.clone(),
                    })
                    .encode()
                    .into(),
                }]
                .into(),
            ),
            Err(InstructionError {
                index: 0,
                error: WeightLimitReached(..)
            })
        ));

        assert_eq!(
            System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::System(frame_system::Event::Remarked { hash, .. },) if hash == Hashing::hash(&remark)
                    )
                })
                .count(),
            0
        );

        // Scenario 2: execute xcm that is valid
        let remark = vec![2];
        assert!(matches!(
            execute_xcm::<
                Runtime,
                xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
                <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            >(
                Location::new(
                    0,
                    [Junction::AccountId32 {
                        network: None,
                        id: [0; 32]
                    }]
                ),
                Weight::from_all(400000000),
                vec![Transact {
                    origin_kind: OriginKind::Native,
                    fallback_max_weight: None,
                    call: RuntimeCall::System(frame_system::Call::remark_with_event {
                        remark: remark.clone()
                    })
                    .encode()
                    .into()
                }]
                .into(),
            ),
            Ok(Outcome::Complete { .. })
        ));

        assert_eq!(
            System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::System(frame_system::Event::Remarked {
                            hash,
                            ..
                        },) if hash == Hashing::hash(&remark)
                    )
                })
                .count(),
            1
        );

        // Scenario 3: Xcm starts executing but there is an error at nth instruction
        let remark_before_failure = vec![3];
        let remark_after_failure = vec![4];
        let response = execute_xcm::<
            Runtime,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
        >(
            Location::new(
                0,
                [Junction::AccountId32 {
                    network: None,
                    id: [0; 32]
                }]
            ),
            Weight::from_all(4616410000),
            vec![
                RefundSurplus,
                Transact {
                    origin_kind: OriginKind::Native,
                    fallback_max_weight: None,
                    call: RuntimeCall::System(frame_system::Call::remark_with_event {
                        remark: remark_before_failure.clone()
                    })
                        .encode()
                        .into()
                },
                WithdrawAsset(
                    vec![Asset {
                        id: AssetId(Location::here()),
                        fun: Fungibility::Fungible(1111),
                    }]
                        .into(),
                ),
                Transact {
                    origin_kind: OriginKind::Native,
                    fallback_max_weight: None,
                    call: RuntimeCall::System(frame_system::Call::remark_with_event {
                        remark: remark_after_failure.clone()
                    })
                        .encode()
                        .into()
                },
            ]
                .into(),
        );

        assert!(matches!(
            response,
            Ok(Outcome::Incomplete {
                used: Weight { .. },
                error: InstructionError { index: 2, .. }
            })
        ));

        // Side effect before failure instruction will be preserved
        assert_eq!(
            System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::System(frame_system::Event::Remarked {
                            hash,
                            ..
                        },) if hash == Hashing::hash(&remark_before_failure)
                    )
                })
                .count(),
            1
        );

        assert_eq!(
            System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::System(frame_system::Event::Remarked {
                            hash,
                            ..
                        },) if hash == Hashing::hash(&remark_after_failure)
                    )
                })
                .count(),
            0
        );
    });
}
