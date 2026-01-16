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
use crate::{xcm_config, Runtime};
use tanssi_runtime_common::processors::v2::execute_xcm;
use xcm::latest::Error::WeightLimitReached;
use xcm::latest::Instruction::{RefundSurplus, WithdrawAsset};
use xcm::latest::{Asset, AssetId, Fungibility, InstructionError, Location, Outcome, Weight};

#[test]
fn test_execute_xcm() {
    ExtBuilder::default().build().execute_with(|| {
        // Note that there is no way to test Outcome::Error variant as the current xcm executor
        // uses XcmWeighter to only test if the xcm has enough weight.
        // Since we explicitly check the weight (reason being as the weight checking is implementation
        // detail of xcm executor, and we can't assume anything since we work with trait) before we pass
        // xcm to executor that logic will never be triggerred.

        // Scenario 1: execute xcm that uses more weight than this limit
        assert!(matches!(
            execute_xcm::<
                Runtime,
                xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
                <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            >(
                Location::here(),
                Weight::from_all(200),
                vec![RefundSurplus].into(),
            ),
            Err(InstructionError {
                index: 0,
                error: WeightLimitReached(..)
            })
        ));

        // Scenario 2: execute xcm that is valid
        assert!(matches!(
            execute_xcm::<
                Runtime,
                xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
                <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
            >(
                Location::here(),
                Weight::from_all(20000000),
                vec![RefundSurplus].into(),
            ),
            Ok(Outcome::Complete { .. })
        ));

        // Scenario 3: Xcm starts executing but there is an error at nth instruction
        let response = execute_xcm::<
            Runtime,
            xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
            <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
        >(
            Location::here(),
            Weight::from_all(4616410000),
            vec![
                RefundSurplus,
                RefundSurplus,
                WithdrawAsset(
                    vec![Asset {
                        id: AssetId(Location::here()),
                        fun: Fungibility::Fungible(1111),
                    }]
                    .into(),
                ),
                RefundSurplus,
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
    });
}
