use crate::tests::common::xcm::mocknets::BridgeHubRococoPara as BridgeHubRococo;
use crate::tests::common::xcm::mocknets::BridgeHubWestendPara as BridgeHubWestend;
use crate::tests::common::xcm::mocknets::AssetHubWestendPara as AssetHubWestend;

use bridge_hub_westend_emulated_chain::BridgeHubWestendParaPallet;
use frame_support::pallet_prelude::TypeInfo;
use hex_literal::hex;
use snowbridge_core::{inbound::InboundQueueFixture, outbound::OperatingMode};
use sp_core::H256;
use sp_runtime::{DispatchError::Token, TokenError::FundsUnavailable};

use crate::assert_expected_events;
use crate::tests::common::xcm::core_buyer_common::ROCOCO_ED;
use crate::tests::common::xcm::mocknets::RococoRelay as Rococo;
use crate::tests::common::xcm::mocknets::WestendRelay as Westend;
use bridge_hub_westend_runtime::EthereumInboundQueue;
use crate::tests::common::xcm::mocknets::RococoRelayPallet;
use crate::tests::common::xcm::mocknets::WestendRelayPallet;
use crate::tests::common::xcm::mocknets::AssetHubWestendParaSender as AssetHubWestendSender;
use crate::tests::common::xcm::mocknets::AssetHubWestendParaReceiver as AssetHubWestendReceiver;

use snowbridge_router_primitives::inbound::{
	Command, Destination, MessageV1, VersionedXcmMessage as VersionedMessage,
};

use crate::VersionedXcm;
use frame_support::assert_ok;
use parity_scale_codec::{Decode, Encode};
use snowbridge_pallet_system;
use staging_xcm::{
    latest::prelude::{Junctions::*, *},
    VersionedLocation,
};
use xcm_emulator::bx;
use xcm_emulator::Chain;
use xcm_emulator::Parachain;
use xcm_emulator::RelayChain;
use xcm_emulator::TestExt;

use super::mocknets::DanceboxPara;
const INITIAL_FUND: u128 = 5_000_000_000 * ROCOCO_ED;
pub const CHAIN_ID: u64 = 11155111;
const TREASURY_ACCOUNT: [u8; 32] =
    hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000");
pub const WETH: [u8; 20] = hex!("87d1f7fdfEe7f651FaBc8bFCB6E086C278b77A7d");
const ETHEREUM_DESTINATION_ADDRESS: [u8; 20] = hex!("44a57ee2f2FCcb85FDa2B0B18EBD0D8D2333700e");
const INSUFFICIENT_XCM_FEE: u128 = 1000;
const XCM_FEE: u128 = 4_000_000_000;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum ControlCall {
    #[codec(index = 3)]
    CreateAgent,
    #[codec(index = 4)]
    CreateChannel { mode: OperatingMode },
}

#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum SnowbridgeControl {
    #[codec(index = 83)]
    Control(ControlCall),
}

/// Create an agent on Ethereum. An agent is a representation of an entity in the Polkadot
/// ecosystem (like a parachain) on Ethereum.
#[test]
#[ignore]
fn create_agent() {
    let origin_para: u32 = 1001;
    // Fund the origin parachain sovereign account so that it can pay execution fees.
    BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

    let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
    let destination = Rococo::child_location_of(BridgeHubRococo::para_id()).into();

    let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
    // Construct XCM to create an agent for para 1001
    let remote_xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit: Unlimited,
            check_origin: None,
        },
        DescendOrigin(Parachain(origin_para).into()),
        Transact {
            require_weight_at_most: 3000000000.into(),
            origin_kind: OriginKind::Xcm,
            call: create_agent_call.encode().into(),
        },
    ]));

    // Rococo Global Consensus
    // Send XCM message from Relay Chain to Bridge Hub source Parachain
    Rococo::execute_with(|| {
        assert_ok!(<Rococo as RococoRelayPallet>::XcmPallet::send(
            sudo_origin,
            bx!(destination),
            bx!(remote_xcm),
        ));

        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;
        // Check that the Transact message was sent
        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    BridgeHubRococo::execute_with(|| {
        type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
        // Check that a message was sent to Ethereum to create the agent
        assert_expected_events!(
            BridgeHubRococo,
            vec![
                RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::CreateAgent {
                    ..
                }) => {},
            ]
        );
    });
}

// Create a channel for a consensus system. A channel is a bidirectional messaging channel
/// between BridgeHub and Ethereum.
#[test]
#[ignore]
fn create_channel() {
    let origin_para: u32 = 1001;
    // Fund AssetHub sovereign account so that it can pay execution fees.
    BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

    let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
    let destination: VersionedLocation =
        Rococo::child_location_of(BridgeHubRococo::para_id()).into();

    let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
    // Construct XCM to create an agent for para 1001
    let create_agent_xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit: Unlimited,
            check_origin: None,
        },
        DescendOrigin(Parachain(origin_para).into()),
        Transact {
            require_weight_at_most: 3000000000.into(),
            origin_kind: OriginKind::Xcm,
            call: create_agent_call.encode().into(),
        },
    ]));

    let create_channel_call = SnowbridgeControl::Control(ControlCall::CreateChannel {
        mode: OperatingMode::Normal,
    });
    // Construct XCM to create a channel for para 1001
    let create_channel_xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit: Unlimited,
            check_origin: None,
        },
        DescendOrigin(Parachain(origin_para).into()),
        Transact {
            require_weight_at_most: 3000000000.into(),
            origin_kind: OriginKind::Xcm,
            call: create_channel_call.encode().into(),
        },
    ]));

    // Rococo Global Consensus
    // Send XCM message from Relay Chain to Bridge Hub source Parachain
    Rococo::execute_with(|| {
        assert_ok!(<Rococo as RococoRelayPallet>::XcmPallet::send(
            sudo_origin.clone(),
            bx!(destination.clone()),
            bx!(create_agent_xcm),
        ));

        assert_ok!(<Rococo as RococoRelayPallet>::XcmPallet::send(
            sudo_origin,
            bx!(destination),
            bx!(create_channel_xcm),
        ));

        type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

        assert_expected_events!(
            Rococo,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });

    BridgeHubRococo::execute_with(|| {
        type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

        // Check that the Channel was created
        assert_expected_events!(
            BridgeHubRococo,
            vec![
                RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::CreateChannel {
                    ..
                }) => {},
            ]
        );
    });
}

use asset_hub_westend_emulated_chain::AssetHubWestendParaPallet as AssetHubWestendPallet;
use frame_support::parameter_types;
use frame_support::weights::WeightToFee;
use snowbridge_core::AssetMetadata;
use snowbridge_core::TokenIdOf;

use snowbridge_router_primitives::inbound::GlobalConsensusEthereumConvertsFor;
use staging_xcm_executor::traits::ConvertLocation;
parameter_types! {
    pub EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 11155111 };
    pub EthereumUniversalLocation: InteriorLocation = [GlobalConsensus(EthereumNetwork::get())].into();
    pub const RelayNetwork: NetworkId = NetworkId::Westend;
    pub UniversalLocation: InteriorLocation =
    [GlobalConsensus(RelayNetwork::get()), Parachain(1002)].into();
    pub AssetHubFromEthereum: Location = Location::new(1,[GlobalConsensus(Westend),Parachain(1000)]);
}

use crate::tests::common::xcm::mocknets::BridgeHubWestendPara;
use crate::tests::common::xcm::mocknets::BridgeHubWestendParaSender;
use crate::tests::common::xcm::mocknets::DanceboxPara as Dancebox;
use crate::tests::common::xcm::mocknets::DanceboxParaPallet;

#[test]
fn transfer_relay_token() {
    let bridgehub_para: u32 = 1002;
    // fund assethub
    let assethub_sovereign = BridgeHubWestend::sovereign_account_id_of(
		BridgeHubWestend::sibling_location_of(AssetHubWestend::para_id()),
	);


    
	BridgeHubWestend::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);
    let origin_para: u32 = 2000;
    let rococo_sudo_origin = <Westend as Chain>::RuntimeOrigin::root();
    let rococo_bridgehub_destination: VersionedLocation =
        Westend::child_location_of(BridgeHubWestend::para_id()).into();
    let dancebox_sovereign = BridgeHubWestend::sovereign_account_id_of(
        BridgeHubWestend::sibling_location_of(DanceboxPara::para_id()),
    );
    BridgeHubWestend::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

    let amount_to_transfer_to_ethereum = INITIAL_FUND*1000;

    let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
    // Construct XCM to create an agent for para 1001
    let create_agent_xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit: Unlimited,
            check_origin: None,
        },
        DescendOrigin(Parachain(origin_para).into()),
        Transact {
            require_weight_at_most: 3000000000.into(),
            origin_kind: OriginKind::Xcm,
            call: create_agent_call.encode().into(),
        },
    ]));

    let create_channel_call = SnowbridgeControl::Control(ControlCall::CreateChannel {
        mode: OperatingMode::Normal,
    });
    // Construct XCM to create a channel for para 1001
    let create_channel_xcm = VersionedXcm::from(Xcm(vec![
        UnpaidExecution {
            weight_limit: Unlimited,
            check_origin: None,
        },
        DescendOrigin(Parachain(origin_para).into()),
        Transact {
            require_weight_at_most: 3000000000.into(),
            origin_kind: OriginKind::Xcm,
            call: create_channel_call.encode().into(),
        },
    ]));

    // Westend Global Consensus
    // Send XCM message from Relay Chain to Bridge Hub source Parachain
    Westend::execute_with(|| {
        assert_ok!(<Westend as WestendRelayPallet>::XcmPallet::send(
            rococo_sudo_origin.clone(),
            bx!(rococo_bridgehub_destination.clone()),
            bx!(create_agent_xcm),
        ));

        assert_ok!(<Westend as WestendRelayPallet>::XcmPallet::send(
            rococo_sudo_origin,
            bx!(rococo_bridgehub_destination),
            bx!(create_channel_xcm),
        ));

        type RuntimeEvent = <Westend as Chain>::RuntimeEvent;

        assert_expected_events!(
            Westend,
            vec![
                RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
            ]
        );
    });


    BridgeHubWestend::execute_with(|| {
        type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

        // Check that the Channel was created
        assert_expected_events!(
            BridgeHubWestend,
            vec![
                RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::CreateChannel {
                    ..
                }) => {},
            ]
        );
    });

    // THE CHANNEL DANCEBOX <> BH IS CREATED


 
    // XcmPallet send arguments
    let sudo_origin = <Dancebox as Chain>::RuntimeOrigin::root();

    let bridgehub_destination: VersionedLocation =
        Location::new(1, Parachain(bridgehub_para)).into();

    let buy_execution_fee_amount = westend_runtime_constants::fee::WeightToFee::weight_to_fee(
        &Weight::from_parts(10_000_000_000, 300_000),
    );

    println!("buy execution fee amount {:?}", buy_execution_fee_amount);

    let buy_execution_fee = Asset {
        id: Location::parent().into(),
        fun: Fungible(buy_execution_fee_amount*1000),
    };

    let destination = Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]);
    let assets: Assets = vec![Asset {
        id: AssetId(Location::new(1, GlobalConsensus(RelayNetwork::get()))),
        fun: Fungible(amount_to_transfer_to_ethereum),
    }]
    .into();

    // this we got it from ebe
    let devolved =
        staging_xcm_builder::ensure_is_remote(UniversalLocation::get(), destination).unwrap();
    let (remote_network, remote_location) = devolved;

    let remote_xcm = Xcm(vec![
        ReserveAssetDeposited(assets),
        DepositAsset {
            assets: Wild(AllCounted(2)),
            beneficiary: AccountKey20 {
                network: Some(EthereumNetwork::get()),
                key: [1; 20],
            }.into(),
        },
        SetTopic([1; 32]),
    ]);
    let xcm: staging_xcm::VersionedXcm<()> = VersionedXcm::from(Xcm(vec![
        WithdrawAsset(vec![buy_execution_fee.clone()].into()),
        BuyExecution {
            fees: buy_execution_fee.clone(),
            weight_limit: Unlimited,
        },
        ExportMessage {
            network: remote_network,
            destination: remote_location,
            xcm: remote_xcm,
        },
    ]));

    BridgeHubWestend::fund_accounts(vec![(dancebox_sovereign.clone(), INITIAL_FUND*10)]);

    let asset_id: Location = Location {
        parents: 1,
        interior: [].into(),
    };
    let expected_asset_id: Location = Location {
        parents: 1,
        interior: [GlobalConsensus(Westend)].into(),
    };

    let expected_token_id = TokenIdOf::convert_location(&expected_asset_id).unwrap();

    let expected_token_id = TokenIdOf::convert_location(&expected_asset_id).unwrap();

    let ethereum_sovereign: crate::AccountId =
        GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&Location::new(
            2,
            [GlobalConsensus(EthereumNetwork::get())],
        ))
        .unwrap()
        .into();

    // Register token
    BridgeHubWestend::execute_with(|| {
        type RuntimeOrigin = <BridgeHubWestend as Chain>::RuntimeOrigin;
        type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

        assert_ok!(
            <BridgeHubWestendPara as BridgeHubWestendParaPallet>::Balances::force_set_balance(
                RuntimeOrigin::root(),
                sp_runtime::MultiAddress::Id(BridgeHubWestendParaSender::get()),
                INITIAL_FUND * 10,
            )
        );

        assert_ok!(
            <BridgeHubWestendPara as BridgeHubWestendParaPallet>::EthereumSystem::register_token(
                RuntimeOrigin::root(),
                Box::new(VersionedLocation::V4(asset_id.clone())),
                AssetMetadata {
                    name: "wnd".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "wnd".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                },
            )
        );
        // Check that a message was sent to Ethereum to create the agent
        assert_expected_events!(
            BridgeHubWestend,
            vec![RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::RegisterToken { .. }) => {},]
        );
    });

 
    Dancebox::execute_with(|| {
        assert_ok!(<Dancebox as DanceboxParaPallet>::PolkadotXcm::send(
            sudo_origin,
            bx!(bridgehub_destination),
            bx!(xcm),
        ));
    });

    // Send token back from ethereum
    BridgeHubWestend::execute_with(|| {
        type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

        // Check that the transfer token back to Ethereum message was queue in the Ethereum
        // Outbound Queue
        assert_expected_events!(
            BridgeHubWestend,
            vec![RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued{ .. }) => {},]
        );
    });

    // Send token to Ethereum as regular user
	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

        assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Balances::force_set_balance(
			RuntimeOrigin::root(),
			sp_runtime::MultiAddress::Id(AssetHubWestendSender::get()),
			amount_to_transfer_to_ethereum*10,
		));


		let assets = vec![Asset { id: AssetId(Location::parent()), fun: Fungible(amount_to_transfer_to_ethereum) }];
		let versioned_assets = staging_xcm::VersionedAssets::from(Assets::from(assets));

		let destination = VersionedLocation::from(Location::new(
			2,
			[GlobalConsensus(Ethereum { chain_id: CHAIN_ID })],
		));

		let beneficiary = VersionedLocation::from(Location::new(
			0,
			[AccountKey20 { network: None, key: ETHEREUM_DESTINATION_ADDRESS.into() }],
		));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::limited_reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(versioned_assets),
			0,
			Unlimited,
		));

		let events = AssetHubWestend::events();
		// Check that the native asset transferred to some reserved account(sovereign of Ethereum)
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Transfer { amount, to, ..})
					if *amount == amount_to_transfer_to_ethereum && *to == ethereum_sovereign.clone(),
			)),
			"native token reserved to Ethereum sovereign account."
		);
	});
    

    // Send token back from ethereum
	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubWestend,
			vec![RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued{ .. }) => {},]
		);

		// Send relay token back to AH
		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendNativeToken {
				token_id: expected_token_id,
				destination: Destination::AccountId32 { id: AssetHubWestendReceiver::get().into() },
				amount: amount_to_transfer_to_ethereum,
				fee: XCM_FEE*5,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();

        println!("XCM is {:?}", xcm);
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubWestend,
			vec![RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},]
		);
	});

    AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		assert_expected_events!(
			AssetHubWestend,
			vec![RuntimeEvent::Balances(pallet_balances::Event::Burned{ .. }) => {},]
		);

		let events = AssetHubWestend::events();

		// Check that the native token burnt from some reserved account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Burned { who, ..})
					if *who == ethereum_sovereign.clone(),
			)),
			"native token burnt from Ethereum sovereign account."
		);

		// Check that the token was minted to beneficiary
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Minted { who, amount })
					if *amount >= amount_to_transfer_to_ethereum && *who == AssetHubWestendReceiver::get()
			)),
			"Token minted to beneficiary."
		);
	});

}
