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

use crate::symbiotic_message_processor::{
    Command, Message, Payload, SymbioticMessageProcessor, MAGIC_BYTES,
};
use crate::tests::common::ExtBuilder;
use crate::{ExternalValidators, Runtime};
use frame_support::pallet_prelude::*;
use keyring::AccountKeyring;
use snowbridge_core::Channel;
use snowbridge_router_primitives::inbound::envelope::Envelope;
use snowbridge_router_primitives::inbound::MessageProcessor;
use sp_core::{H160, H256};
use sp_runtime::DispatchError;

#[test]
fn test_symbiotic_message_processor() {
    ExtBuilder::default().build().execute_with(|| {
        let default_channel = Channel {
            agent_id: H256::default(),
            para_id: 0.into(),
        };

        let envelope_with_invalid_payload = Envelope {
            channel_id: H256::default().into(),
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: vec![0, 1, 2],
        };

        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(
                &default_channel,
                &envelope_with_invalid_payload
            ),
            false
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope_with_invalid_payload
            ),
            Err(DispatchError::Other("unable to parse the payload"))
        );

        let payload_with_incorrect_magic_bytes = Payload {
            magic_bytes: [1, 2, 3, 4],
            message: Message::V1(Command::<Runtime>::ReceiveValidators { validators: vec![] }),
        };
        let envelope = Envelope {
            channel_id: H256::default().into(),
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_with_incorrect_magic_bytes.encode(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            false
        );

        // No external validators are set right now
        assert_eq!(
            ExternalValidators::validators(),
            ExternalValidators::whitelisted_validators()
        );

        let payload_validators = vec![
            AccountKeyring::Alice.to_account_id(),
            AccountKeyring::Charlie.to_account_id(),
            AccountKeyring::Bob.to_account_id(),
        ];

        let payload_with_correct_magic_bytes = Payload {
            magic_bytes: MAGIC_BYTES,
            message: Message::V1(Command::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
            }),
        };
        let envelope = Envelope {
            channel_id: H256::default().into(),
            gateway: H160::default(),
            message_id: Default::default(),
            nonce: 0,
            payload: payload_with_correct_magic_bytes.encode(),
        };
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::can_process_message(&default_channel, &envelope),
            true
        );
        assert_eq!(
            SymbioticMessageProcessor::<Runtime>::process_message(
                default_channel.clone(),
                envelope
            ),
            Ok(())
        );

        let expected_validators = [
            ExternalValidators::whitelisted_validators(),
            payload_validators,
        ]
        .concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}
