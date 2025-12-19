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

extern crate alloc;

use crate::processors::v1::GatewayAndChannelValidator;
use alloc::vec;
use core::marker::PhantomData;
use parity_scale_codec::{DecodeAll, Encode, EncodeLike};
use snowbridge_core::Channel;
use snowbridge_inbound_queue_primitives::v1::{
    Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
};
use sp_core::{Get, H160};
use sp_runtime::{DispatchError, DispatchResult};
use xcm::latest::{
    prelude::*, Asset as XcmAsset, AssetId as XcmAssetId, Assets as XcmAssets, ExecuteXcm,
    Fungibility, Junctions::*,
};
use xcm_executor::traits::{TransactAsset, WeightBounds};

/// `EthTokensLocalProcessor` is responsible for receiving and processing the ETH native
/// token and ERC20s coming from Ethereum with Tanssi chain or container-chains as final destinations.
/// TODO: add support for container transfers
pub struct EthTokensLocalProcessor<
    T,
    XcmProcessor,
    XcmWeigher,
    AssetTransactor,
    EthereumLocation,
    EthereumNetwork,
>(
    PhantomData<(
        T,
        XcmProcessor,
        XcmWeigher,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
    )>,
);

impl<T, XcmProcessor, XcmWeigher, AssetTransactor, EthereumLocation, EthereumNetwork>
    MessageProcessor
    for EthTokensLocalProcessor<
        T,
        XcmProcessor,
        XcmWeigher,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + pallet_foreign_asset_creator::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    <T as frame_system::Config>::AccountId: Into<Location>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
    AssetTransactor: TransactAsset,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    cumulus_primitives_core::Location:
        EncodeLike<<T as pallet_foreign_asset_creator::Config>::ForeignAsset>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        // Validate channel and gateway
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!("EthTokensLocalProcessor::can_process_message: invalid gateway or channel");
            return false;
        }

        if let Some(eth_transfer_data) =
            Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
        {
            // Check if the token is registered in the relay
            match eth_transfer_data.destination {
                Destination::AccountId32 { id: _ } => {
                    return pallet_foreign_asset_creator::ForeignAssetToAssetId::<T>::get(
                        eth_transfer_data.token_location,
                    )
                    .is_some();
                }
                _ => return true,
            }
        }

        false
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        let eth_transfer_data = Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
            .ok_or(DispatchError::Other("unexpected message"))?;

        match eth_transfer_data.destination {
            Destination::AccountId32 { id: _ } => {
                Self::process_xcm_local_native_eth_transfer(eth_transfer_data)
            }
            Destination::ForeignAccountId32 { .. } | Destination::ForeignAccountId20 { .. } => {
                Self::process_xcm_container_eth_transfer(eth_transfer_data)
            }
        }
    }
}

pub struct EthTransferData {
    pub token_location: Location,
    pub destination: Destination,
    pub amount: u128,
}

impl<T, XcmProcessor, XcmWeigher, AssetTransactor, EthereumLocation, EthereumNetwork>
    EthTokensLocalProcessor<
        T,
        XcmProcessor,
        XcmWeigher,
        AssetTransactor,
        EthereumLocation,
        EthereumNetwork,
    >
where
    T: frame_system::Config + pallet_xcm::Config + pallet_ethereum_token_transfers::Config,
    <T as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<T>>,
    <T as frame_system::Config>::AccountId: Into<Location>,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
    AssetTransactor: TransactAsset,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    pub fn decode_message_for_eth_transfer(mut payload: &[u8]) -> Option<EthTransferData> {
        match VersionedXcmMessage::decode_all(&mut payload) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command:
                    Command::SendToken {
                        token: token_address,
                        destination,
                        amount,
                        fee: _,
                    },
                ..
            })) => {
                let token_location = if token_address == H160::zero() {
                    Location {
                        parents: 1,
                        interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
                    }
                } else {
                    Location {
                        parents: 1,
                        interior: X2([
                            GlobalConsensus(EthereumNetwork::get()),
                            AccountKey20 {
                                network: Some(EthereumNetwork::get()),
                                key: token_address.into(),
                            },
                        ]
                        .into()),
                    }
                };

                Some(EthTransferData {
                    token_location,
                    destination,
                    amount,
                })
            }
            _ => None,
        }
    }

    fn process_xcm_local_native_eth_transfer(eth_transfer_data: EthTransferData) -> DispatchResult {
        let assets_to_holding: XcmAssets = vec![XcmAsset {
            id: XcmAssetId::from(eth_transfer_data.token_location),
            fun: Fungibility::Fungible(eth_transfer_data.amount),
        }]
        .into();

        let destination_account = match eth_transfer_data.destination {
            Destination::AccountId32 { id } => id,
            _ => {
                log::error!("EthTokensLocalProcessor: invalid destination");
                return Ok(());
            }
        };

        let mut xcm = Xcm::<<T as pallet_xcm::Config>::RuntimeCall>(vec![
            ReserveAssetDeposited(assets_to_holding),
            DepositAsset {
                assets: AllCounted(1).into(),
                beneficiary: Location::new(
                    0,
                    [AccountId32 {
                        network: None,
                        id: destination_account,
                    }],
                ),
            },
        ]);

        let ethereum_location = EthereumLocation::get();

        // Using Weight::MAX here because we don't have a limit, same as they do in pallet-xcm
        if let Ok(weight) = XcmWeigher::weight(&mut xcm, Weight::MAX) {
            let mut message_id = xcm.using_encoded(sp_io::hashing::blake2_256);

            let outcome = XcmProcessor::prepare_and_execute(
                ethereum_location,
                xcm,
                &mut message_id,
                weight,
                weight,
            );

            if let Err(error) = outcome.ensure_complete() {
                log::error!(
                    "EthTokensLocalProcessor: XCM execution failed with error {:?}",
                    error
                );
            }
        } else {
            log::error!("EthTokensLocalProcessor: unweighable message");
        }

        Ok(())
    }

    fn process_xcm_container_eth_transfer(eth_transfer_data: EthTransferData) -> DispatchResult {
        // Get the para_id, beneficiary and fee from the destination
        let (para_id, beneficiary, fee) = match eth_transfer_data.destination {
            Destination::ForeignAccountId32 { para_id, id, fee } => (
                para_id,
                Location::new(0, [AccountId32 { network: None, id }]),
                fee,
            ),
            Destination::ForeignAccountId20 { para_id, id, fee } => (
                para_id,
                Location::new(
                    0,
                    [AccountKey20 {
                        network: None,
                        key: id,
                    }],
                ),
                fee,
            ),
            _ => {
                log::error!(
                    "EthTokensLocalProcessor: unsupported destination for container transfer: {:?}",
                    eth_transfer_data.destination
                );
                return Ok(());
            }
        };

        // Container chain location from relay point of view
        let container_location = Location::new(0, [Parachain(para_id)]);

        // Reanchor the token location to the container chain location
        let token_id_dest = match eth_transfer_data.token_location.clone().reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                    "EthTokensLocalProcessor: failed to reanchor token location: {:?}",
                    e
                );
                return Ok(());
            }
        };

        // Asset to pay fees, in this case native relay token
        let asset_fee_relay: Asset = (Location::here(), fee).into();

        // Reanchor the asset fee to the container chain location
        let asset_fee_container = match asset_fee_relay.clone().reanchored(
            &container_location,
            &<T as pallet_xcm::Config>::UniversalLocation::get(),
        ) {
            Ok(loc) => loc,
            Err(e) => {
                log::error!(
                    "EthTokensLocalProcessor: failed to reanchor relay token location: {:?}",
                    e
                );
                return Ok(());
            }
        };

        // Ethereum token location from relay point of view
        let eth_token_location: Asset = (
            eth_transfer_data.token_location.clone(),
            eth_transfer_data.amount,
        )
            .into();

        // Asset to deposit into the container chain
        let asset_to_deposit: Asset = (token_id_dest.clone(), eth_transfer_data.amount).into();

        let dummy_context = XcmContext {
            origin: None,
            message_id: Default::default(),
            topic: None,
        };

        // To early check if the token is registered in the relay
        if let Err(e) =
            AssetTransactor::can_check_in(&container_location, &eth_token_location, &dummy_context)
        {
            log::error!("EthTokensLocalProcessor: can_check_in failed: {:?}", e);
            return Ok(());
        }

        // Transfer fee from FeesAccount to container sovereign account
        if let Err(e) = AssetTransactor::transfer_asset(
            &asset_fee_relay,
            &T::FeesAccount::get().into(),
            &container_location,
            &dummy_context,
        ) {
            log::error!(
                "EthTokensLocalProcessor: failed to transfer fee from FeesAccount to container sovereign account: {:?}",
                e
            );
            return Ok(());
        }

        // Mint the ERC20 token into the container sovereign account
        AssetTransactor::check_in(&container_location, &eth_token_location, &dummy_context);

        if let Err(e) =
            AssetTransactor::deposit_asset(&eth_token_location, &container_location, None)
        {
            log::error!("EthTokensLocalProcessor: deposit_asset failed: {:?}", e);
            return Ok(());
        }

        // Send XCM to deposit the ERC20 token into beneficiary account and pay fees
        let remote_xcm = Xcm::<()>(vec![
            ReserveAssetDeposited(
                vec![asset_fee_container.clone(), asset_to_deposit.clone()].into(),
            ),
            BuyExecution {
                fees: asset_fee_container.clone(),
                weight_limit: Unlimited,
            },
            DepositAsset {
                assets: Definite(vec![asset_to_deposit].into()),
                beneficiary,
            },
        ]);

        match send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
            container_location.clone(),
            remote_xcm.clone(),
        ) {
            Ok((message_id, _price)) => {
                let evt: pallet_xcm::Event<T> = pallet_xcm::Event::Sent {
                    origin: Here.into_location(),
                    destination: container_location,
                    message: remote_xcm,
                    message_id,
                };
                frame_system::Pallet::<T>::deposit_event(
                    <T as frame_system::Config>::RuntimeEvent::from(evt),
                );
            }
            Err(e) => {
                log::error!(
                    "EthTokensLocalProcessor: XCM send failed to para_id {} with error: {:?}",
                    para_id,
                    e
                );
            }
        };

        Ok(())
    }
}
