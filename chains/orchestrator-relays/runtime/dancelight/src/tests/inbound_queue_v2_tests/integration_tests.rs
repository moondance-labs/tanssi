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
        tests::common::{mock_snowbridge_message_proof, root_origin, ExtBuilder, ALICE, BOB, UNIT},
        xcm_config::UniversalLocation,
        AccountId, EthereumInboundQueueV2, EthereumSystemV2, ExternalValidators, Runtime,
    },
    alloy_sol_types::SolEvent,
    dancelight_runtime_constants::snowbridge::{EthereumLocation, EthereumNetwork},
    frame_support::assert_ok,
    frame_system::pallet_prelude::OriginFor,
    hex_literal::hex,
    keyring::Sr25519Keyring,
    parity_scale_codec::Encode,
    snowbridge_core::{Channel, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_inbound_queue_primitives::v2::{
        EthereumAsset::ForeignTokenERC20, IGatewayV2, Payload,
    },
    snowbridge_verification_primitives::{EventProof, Log},
    sp_core::H256,
    sp_runtime::{traits::MaybeEquivalence, DispatchError},
    tanssi_runtime_common::processors::v2::RawPayload,
    tp_bridge::symbiotic_message_processor::{
        InboundCommand, Message as SymbioticMessage, Payload as SymbioticPayload, MAGIC_BYTES,
    },
    xcm::{
        latest::prelude::{Junctions::*, *},
        VersionedXcm,
    },
};

#[test]
fn test_inbound_queue_message_symbiotic_passing() {
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

        // TODO: Check if we need this
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
            event_log: Log {
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

#[test]
fn test_inbound_queue_message_xcm_passing() {
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

        let dummy_proof = mock_snowbridge_message_proof();
        let token_location = Location::here();

        assert_ok!(EthereumSystemV2::register_token(
            root_origin(),
            Box::new(token_location.clone().into()),
            Box::new(token_location.clone().into()),
            snowbridge_core::AssetMetadata {
                name: "relay".as_bytes().to_vec().try_into().unwrap(),
                symbol: "relay".as_bytes().to_vec().try_into().unwrap(),
                decimals: 12,
            },
            1
        ));

        let token_location_reanchored = token_location
            .clone()
            .reanchored(&EthereumLocation::get(), &UniversalLocation::get())
            .expect("unable to reanchor token");

        let tanssi_token_id =
            snowbridge_pallet_system::NativeToForeignId::<Runtime>::get(&token_location_reanchored).unwrap();

        let tanssi_token_transfer_value = 3_500_000_000_000u128;
        let assets = vec![
            ForeignTokenERC20 { token_id: tanssi_token_id.into(), value: tanssi_token_transfer_value },
        ];

        println!("assets: {:?}", assets);

        let execution_fee = 1_500_000_000_000u128;

        let instructions = vec![
            WithdrawAsset(
                (Location::here(), tanssi_token_transfer_value).into()
            ),
            BuyExecution {
                fees: (Location::here(), execution_fee).into(),
                weight_limit: WeightLimit::Unlimited,
            },
            DepositAsset {
                assets: AllCounted(1).into(),
                beneficiary: Location::new(
                    0,
                    [AccountId32 { network: None, id: BOB.into() }],
                ),
            },
        ];

        let xcm: Xcm<()> = instructions.into();
        let versioned_message_xcm = VersionedXcm::V5(xcm);

        let xcm_bytes = RawPayload::Xcm(versioned_message_xcm.encode());

        println!("xcm_bytes: {:?}", xcm_bytes.encode());

        assert_eq!(EthereumInboundQueueV2::submit(OriginFor::<Runtime>::signed(AccountId::new([0; 32])), Box::new(EventProof {
            event_log: Log {
                address: <Runtime as snowbridge_pallet_inbound_queue::Config>::GatewayAddress::get(),
                topics: vec![hex!("550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c").into()],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000eda338e4dc46038493b885327842fd3e301cab3900000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000032ee841b8000000000000000000000000000000000000000000000000000000015d3ef79800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000040e95142d5aca3299068a3d9b4a659f9589559382d0a130a1d7cedc67d6c3d401d00000000000000000000000000000000000000000000000000000246139ca800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000045000901050c00040000000b00b841e82e03130000000b0098f73e5d01000d0102040001010005050505050505050505050505050505050505050505050505050505050505050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            },
            proof: dummy_proof.clone(),
        })), Ok(()));

        let events = frame_system::Pallet::<Runtime>::events();
        for e in events {
            println!("Event: {:?}", e.event);
        }
    });
}
