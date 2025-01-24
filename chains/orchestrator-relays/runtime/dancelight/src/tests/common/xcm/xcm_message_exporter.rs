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
    super::{
        super::{AccountId, ExtBuilder, ALICE},
        mocknets::{DancelightRelay as Dancelight, DancelightRelayPallet},
    },
    frame_support::weights::Weight,
    pallet_xcm::Error,
    sp_runtime::DispatchError,
    xcm::{latest::prelude::*, VersionedXcm},
    xcm_emulator::Chain,
};

#[test]
fn test_message_export_disabled() {
    ExtBuilder::default().build().execute_with(|| {
        // The only test we can do is with signed runtime origins since we are ensuring local origin in xcm config
        let origin = <Dancelight as Chain>::RuntimeOrigin::signed(AccountId::from(ALICE));

        let message = Xcm(vec![Instruction::ExportMessage {
            network: NetworkId::Ethereum { chain_id: 1 },
            destination: Junctions::Here,
            xcm: Xcm(vec![]),
        }]);

        assert_eq!(
            <Dancelight as DancelightRelayPallet>::XcmPallet::execute(
                origin,
                Box::new(VersionedXcm::V4(message)),
                Weight::from_parts(0, 0)
            )
            .unwrap_err()
            .error,
            DispatchError::from(Error::<<Dancelight as Chain>::Runtime>::LocalExecutionIncomplete)
        );
    });
}
