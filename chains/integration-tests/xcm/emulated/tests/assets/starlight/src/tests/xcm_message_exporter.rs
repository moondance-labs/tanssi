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

use pallet_xcm::ExecutionError;
use {
    frame_support::{assert_err, weights::Weight},
    pallet_xcm::Error,
    primitives::AccountId,
    sp_runtime::DispatchError,
    starlight_emulated_chain::StarlightRelayPallet,
    starlight_runtime::xcm_config,
    starlight_system_emulated_network::StarlightRelay as Starlight,
    xcm::{latest::prelude::*, v5::Location, VersionedXcm},
    xcm_emulator::{Chain, TestExt},
};

#[test]
fn test_message_exporter_disabled_for_origin_account() {
    Starlight::execute_with(|| {
        // The only test we can do is with signed runtime origins since we are ensuring local origin in xcm config
        let origin = <Starlight as Chain>::RuntimeOrigin::signed(AccountId::from(
            tanssi_emulated_integration_tests_common::accounts::ALICE,
        ));

        let message = Xcm(vec![Instruction::ExportMessage {
            network: NetworkId::Ethereum { chain_id: 1 },
            destination: Junctions::Here,
            xcm: Xcm(vec![]),
        }]);

        assert_eq!(
            <Starlight as StarlightRelayPallet>::XcmPallet::execute(
                origin,
                Box::new(VersionedXcm::V5(message)),
                Weight::from_parts(0, 0)
            )
            .unwrap_err()
            .error,
            DispatchError::from(
                Error::<<Starlight as Chain>::Runtime>::LocalExecutionIncompleteWithError {
                    index: 0,
                    error: ExecutionError::Overflow,
                }
            )
        );
    });
}

#[test]
fn test_message_exporter_validate_should_fail() {
    let mut location = Some(Location {
        parents: 1,
        interior: Junctions::Here,
    });

    let mut message = Some(Xcm(vec![Instruction::ExportMessage {
        network: NetworkId::Ethereum { chain_id: 1 },
        destination: Junctions::Here,
        xcm: Xcm(vec![]),
    }]));

    assert_err!(
        <xcm_config::XcmConfig as xcm_executor::Config>::MessageExporter::validate(
            &mut location,
            &mut message
        ),
        SendError::NotApplicable
    );
}
