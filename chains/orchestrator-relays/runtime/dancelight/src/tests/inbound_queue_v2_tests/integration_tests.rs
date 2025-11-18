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
    crate::{
        tests::common::{mock_snowbridge_message_proof, ExtBuilder, ALICE, BOB, UNIT},
        AccountId, EthereumInboundQueueV2, ExternalValidators, Runtime,
    },
    alloy_sol_types::SolEvent,
    frame_system::pallet_prelude::OriginFor,
    hex_literal::hex,
    keyring::Sr25519Keyring,
    parity_scale_codec::Encode,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::v2::{IGatewayV2, Payload},
    snowbridge_verification_primitives::{EventProof, Log},
    sp_core::H256,
    sp_runtime::DispatchError,
    tanssi_runtime_common::processors::v2::RawPayload,
    tp_bridge::symbiotic_message_processor::{
        InboundCommand, Message as SymbioticMessage, Payload as SymbioticPayload, MAGIC_BYTES,
    },
};

#[test]
fn test_inbound_queue_message_passing() {
    ExtBuilder::default()
        .with_validators(
            vec![]
        )
        .with_external_validators(
            vec![
                (AccountId::from(ALICE), 210 * UNIT),
                (AccountId::from(BOB), 100 * UNIT),
            ]
        ).build().execute_with(|| {
        let current_nonce = 1;

        snowbridge_pallet_system::Channels::<Runtime>::set(PRIMARY_GOVERNANCE_CHANNEL, Some(Channel {
            agent_id: Default::default(),
            para_id: Default::default(),
        }));

        let dummy_proof = mock_snowbridge_message_proof();

        let payload_validators = vec![
            Sr25519Keyring::Charlie.to_account_id(),
            Sr25519Keyring::Ferdie.to_account_id(),
            Sr25519Keyring::BobStash.to_account_id()
        ];

        let payload = SymbioticPayload {
            magic_bytes: MAGIC_BYTES,
            message: SymbioticMessage::V1(InboundCommand::<Runtime>::ReceiveValidators {
                validators: payload_validators.clone(),
                external_index: 0u64,
            }),
        };

        let symbiotic_bytes = RawPayload::Symbiotic(payload.encode());

        println!("symbiotic_bytes: {:?}", symbiotic_bytes.encode());

        // use ethers::abi::{encode, Token};
        // let data = encode(&[
        //     Token::Uint(nonce.into()),
        //     Token::Tuple(vec![
        //         Token::Address(hex_literal::hex!("EDa338E4dC46038493b885327842fD3E301CaB39").into()),
        //         Token::Array(vec![]),
        //         Token::Tuple(vec![
        //             Token::Uint(0.into()),
        //             Token::Bytes(symbiotic_bytes.clone()),
        //         ]),
        //         Token::Bytes(vec![]),
        //         Token::Uint(0.into()),
        //         Token::Uint(0.into()),
        //         Token::Uint(0.into()),
        //     ])
        // ]);

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log{
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000eda338e4dc46038493b885327842fd3e301cab3900000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007201bd017015003800000c90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe221cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07cfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let expected_validators = [ExternalValidators::whitelisted_validators(), payload_validators].concat();
        assert_eq!(ExternalValidators::validators(), expected_validators);
    });
}
