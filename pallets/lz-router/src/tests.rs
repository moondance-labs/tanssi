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
    crate::{mock::*, types::RoutingConfig, Error, Event, Pallet, RoutingConfigs},
    frame_support::{assert_err, assert_noop, assert_ok},
    snowbridge_inbound_queue_primitives::v2::MessageProcessorError,
    sp_runtime::{BoundedBTreeSet, BoundedVec},
    tp_bridge::layerzero_message::InboundMessage,
};

/// Helper to create a routing config
fn create_routing_config(
    whitelisted_senders: Vec<(u32, Vec<u8>)>,
    pallet_index: u8,
    call_index: u8,
) -> RoutingConfig<Test> {
    let mut senders: BoundedBTreeSet<_, _> = BoundedBTreeSet::new();
    for (endpoint, address) in whitelisted_senders {
        let addr: BoundedVec<u8, _> = address.try_into().expect("address too long");
        senders
            .try_insert((endpoint, addr))
            .expect("too many whitelisted senders");
    }

    RoutingConfig {
        whitelisted_senders: senders,
        notification_destination: (pallet_index, call_index),
    }
}

/// Helper to create an inbound message
fn create_inbound_message(
    source_address: Vec<u8>,
    source_endpoint: u32,
    destination_chain: u32,
    message: Vec<u8>,
) -> InboundMessage {
    InboundMessage {
        lz_source_address: source_address.try_into().expect("address too long"),
        lz_source_endpoint: source_endpoint,
        destination_chain,
        message,
    }
}

mod update_routing_config {
    use super::*;

    #[test]
    fn update_routing_config_works() {
        ExtBuilder.build().execute_with(|| {
            let config = create_routing_config(vec![(30101, vec![0x12, 0x34, 0x56])], 79, 0);

            // Signed origin with account 2000 -> treated as Parachain(2000)
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(2000),
                config.clone()
            ));

            // Check storage
            assert_eq!(RoutingConfigs::<Test>::get(2000), Some(config.clone()));

            // Check event
            assert_eq!(
                lz_router_events(),
                vec![Event::RoutingConfigUpdated {
                    chain_id: 2000,
                    new_config: config,
                    old_config: None,
                }]
            );
        });
    }

    #[test]
    fn update_routing_config_updates_existing() {
        ExtBuilder.build().execute_with(|| {
            let config1 = create_routing_config(vec![(30101, vec![0x12, 0x34])], 79, 0);
            let config2 = create_routing_config(vec![(30101, vec![0xAB, 0xCD])], 80, 1);

            // Create initial config
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(2000),
                config1.clone()
            ));

            // Check storage has initial config
            assert_eq!(RoutingConfigs::<Test>::get(2000), Some(config1.clone()));

            // Update to new config
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(2000),
                config2.clone()
            ));

            // Check storage has new config
            assert_eq!(RoutingConfigs::<Test>::get(2000), Some(config2.clone()));

            // Check events include both updates
            let all_events = lz_router_events();
            assert_eq!(all_events.len(), 2);
            assert_eq!(
                all_events[1],
                Event::RoutingConfigUpdated {
                    chain_id: 2000,
                    new_config: config2,
                    old_config: Some(config1),
                }
            );
        });
    }

    #[test]
    fn update_routing_config_fails_with_same_config() {
        ExtBuilder.build().execute_with(|| {
            let config = create_routing_config(vec![(30101, vec![0x12, 0x34])], 79, 0);

            // Create initial config
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(2000),
                config.clone()
            ));

            // Try to set the same config again
            assert_noop!(
                LzRouter::update_routing_config(RuntimeOrigin::signed(2000), config),
                Error::<Test>::SameConfigAlreadyExists
            );
        });
    }
}

mod handle_inbound_message {
    use super::*;

    #[test]
    fn handle_inbound_message_works() {
        ExtBuilder.build().execute_with(|| {
            let source_address = vec![0x12, 0x34, 0x56];
            let source_endpoint = 30101u32;
            let destination_chain = 2000u32;

            // First, set up a routing config for the destination chain
            let config =
                create_routing_config(vec![(source_endpoint, source_address.clone())], 79, 0);
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(u64::from(destination_chain)),
                config
            ));

            // Clear events from config setup
            System::reset_events();
            clear_sent_xcm();

            // Create and handle inbound message
            let message = create_inbound_message(
                source_address.clone(),
                source_endpoint,
                destination_chain,
                b"hello world".to_vec(),
            );

            assert_ok!(Pallet::<Test>::handle_inbound_message(message.clone()));

            // Check that XCM was sent
            let sent = sent_xcm();
            assert_eq!(sent.len(), 1);
            assert_eq!(
                sent[0].0,
                xcm::latest::Location::new(
                    0,
                    [xcm::latest::Junction::Parachain(destination_chain)]
                )
            );

            // Check event
            assert_eq!(
                lz_router_events(),
                vec![Event::InboundMessageRouted {
                    chain_id: destination_chain,
                    message,
                }]
            );
        });
    }

    #[test]
    fn handle_inbound_message_fails_no_routing_config() {
        ExtBuilder.build().execute_with(|| {
            let message = create_inbound_message(
                vec![0x12, 0x34],
                30101,
                2000, // No routing config for this chain
                b"hello".to_vec(),
            );

            // Should fail because no routing config exists for chain 2000
            assert_err!(
                Pallet::<Test>::handle_inbound_message(message),
                MessageProcessorError::ProcessMessage(Error::<Test>::NoRoutingConfig.into())
            );
        });
    }

    #[test]
    fn handle_inbound_message_fails_not_whitelisted() {
        ExtBuilder.build().execute_with(|| {
            let whitelisted_address = vec![0x12, 0x34];
            let non_whitelisted_address = vec![0xAB, 0xCD];
            let source_endpoint = 30101u32;
            let destination_chain = 2000u32;

            // Set up routing config with specific whitelisted sender
            let config = create_routing_config(vec![(source_endpoint, whitelisted_address)], 79, 0);
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(u64::from(destination_chain)),
                config
            ));

            // Try to send message from non-whitelisted address - should fail
            let message = create_inbound_message(
                non_whitelisted_address,
                source_endpoint,
                destination_chain,
                b"hello".to_vec(),
            );

            assert_err!(
                Pallet::<Test>::handle_inbound_message(message),
                MessageProcessorError::ProcessMessage(Error::<Test>::NotWhitelistedSender.into())
            );
        });
    }

    #[test]
    fn handle_inbound_message_fails_wrong_endpoint() {
        ExtBuilder.build().execute_with(|| {
            let source_address = vec![0x12, 0x34];
            let whitelisted_endpoint = 30101u32;
            let wrong_endpoint = 30110u32;
            let destination_chain = 2000u32;

            // Set up routing config with specific endpoint
            let config =
                create_routing_config(vec![(whitelisted_endpoint, source_address.clone())], 79, 0);
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(u64::from(destination_chain)),
                config
            ));

            // Try to send message from wrong endpoint - should fail (not whitelisted)
            let message = create_inbound_message(
                source_address,
                wrong_endpoint,
                destination_chain,
                b"hello".to_vec(),
            );

            assert_err!(
                Pallet::<Test>::handle_inbound_message(message),
                MessageProcessorError::ProcessMessage(Error::<Test>::NotWhitelistedSender.into())
            );
        });
    }

    #[test]
    fn handle_inbound_message_with_multiple_whitelisted_senders() {
        ExtBuilder.build().execute_with(|| {
            let sender1 = (30101u32, vec![0x11, 0x22]);
            let sender2 = (30110u32, vec![0x33, 0x44]);
            let sender3 = (30184u32, vec![0x55, 0x66]);
            let destination_chain = 2000u32;

            // Set up routing config with multiple whitelisted senders
            let config = create_routing_config(
                vec![sender1.clone(), sender2.clone(), sender3.clone()],
                79,
                0,
            );
            assert_ok!(LzRouter::update_routing_config(
                RuntimeOrigin::signed(u64::from(destination_chain)),
                config
            ));

            // Message from sender2 should work
            let message = create_inbound_message(
                sender2.1.clone(),
                sender2.0,
                destination_chain,
                b"from sender2".to_vec(),
            );
            assert_ok!(Pallet::<Test>::handle_inbound_message(message));

            // Message from sender3 should work
            let message = create_inbound_message(
                sender3.1.clone(),
                sender3.0,
                destination_chain,
                b"from sender3".to_vec(),
            );
            assert_ok!(Pallet::<Test>::handle_inbound_message(message));

            // Check that 2 XCM messages were sent
            assert_eq!(sent_xcm().len(), 2);
        });
    }
}
