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

#![cfg(test)]

use {
    crate::{tests::common::*, EthereumSystem, RuntimeEvent, SessionsPerEra, System},
    frame_support::assert_ok,
    xcm::{latest::prelude::*, VersionedLocation},
};
#[test]
fn external_validators_rewards_sends_message_on_era_end() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_validators(vec![])
        .with_external_validators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            // This will call on_era_end for era 0
            run_to_session(sessions_per_era);

            let outbound_msg_queue_event_count = System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::EthereumOutboundQueue(
                            snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                        )
                    )
                })
                .count();

            assert_eq!(
                outbound_msg_queue_event_count, 1,
                "MessageQueued event should be emitted"
            );

            let message_accepted_event_count = System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::EthereumOutboundQueue(
                            snowbridge_pallet_outbound_queue::Event::MessageAccepted { .. },
                        )
                    )
                })
                .count();

            assert_eq!(
                message_accepted_event_count, 1,
                "MessageAccepted event should be emitted"
            );
        });
}

#[test]
fn external_validators_rewards_not_send_message_on_era_end() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_validators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_external_validators(vec![])
        .build()
        .execute_with(|| {
            let token_location: VersionedLocation = Location::here().into();

            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(token_location),
                snowbridge_core::AssetMetadata {
                    name: "dance".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "dance".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            // SessionsPerEra depends on fast-runtime feature, this test should pass regardless
            let sessions_per_era = SessionsPerEra::get();

            // This will call on_era_end for era 0
            run_to_session(sessions_per_era);

            let outbound_msg_queue_event_count = System::events()
                .iter()
                .filter(|r| {
                    matches!(
                        r.event,
                        RuntimeEvent::EthereumOutboundQueue(
                            snowbridge_pallet_outbound_queue::Event::MessageQueued { .. },
                        )
                    )
                })
                .count();

            assert_eq!(
                outbound_msg_queue_event_count, 0,
                "MessageQueued event should not be emitted because there are no external validators"
            );
        });
}
