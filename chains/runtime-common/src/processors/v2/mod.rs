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

mod fallback_message_processor;
mod raw_message_processor;
mod symbiotic_message_processor;

pub use raw_message_processor::RawMessageProcessor;
pub use symbiotic_message_processor::SymbioticMessageProcessor;

use alloc::vec;
use alloc::{boxed::Box, string::String, vec::Vec};

use thiserror::Error;

use frame_support::weights::Weight;
use parity_scale_codec::{Decode, Encode};
use snowbridge_inbound_queue_primitives::v2::{
    AssetTransfer, EthereumAsset, Message, MessageProcessorError,
};
use sp_core::{H160, H256};
use sp_io::hashing::blake2_256;
use sp_runtime::traits::MaybeEquivalence;
use sp_runtime::DispatchError;
use xcm::latest::prelude::*;
use xcm_executor::traits::WeightBounds;

/// Topic prefix used for generating unique identifiers for messages
pub const RAW_MESSAGE_PROCESSOR_TOPIC_PREFIX: &str = "TanssiRawMessageProcessor";

/// Wrapping parity_scale_codec::Error so that it implements Error
#[derive(Debug)]
pub struct CodecError(parity_scale_codec::Error);

impl core::fmt::Display for CodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl core::error::Error for CodecError {}

impl From<parity_scale_codec::Error> for CodecError {
    fn from(e: parity_scale_codec::Error) -> Self {
        CodecError(e)
    }
}

#[derive(Error, Debug)]
pub enum MessageExtractionError {
    #[error("Unsupported Message: {context} due to {source:?}")]
    UnsupportedMessage {
        context: String,
        source: Option<Box<dyn core::error::Error + Send + Sync>>,
    },
    #[error("Invalid Message: {context} due to {source:?}")]
    InvalidMessage {
        context: String,
        source: Option<Box<dyn core::error::Error + Send + Sync>>,
    },
    #[error("Other error: {context} due to {source:?}")]
    Other {
        context: String,
        source: Option<Box<dyn core::error::Error + Send + Sync>>,
    },
}

#[derive(Error, Debug)]
pub enum LocationConversionError {
    #[error("Unable to reanchor {location:?}")]
    UnableToReanchor { location: Location },
    #[error("Unable to convert {token_id} in location")]
    UnableToConvertTokenId { token_id: H256 },
}

impl Into<MessageProcessorError> for MessageExtractionError {
    fn into(self) -> MessageProcessorError {
        match self {
            MessageExtractionError::UnsupportedMessage { .. } => {
                MessageProcessorError::ProcessMessage(DispatchError::Other(
                    "Unsupported v2 message",
                ))
            }
            MessageExtractionError::InvalidMessage { .. } => {
                MessageProcessorError::ProcessMessage(DispatchError::Other("Invalid v2 message"))
            }
            MessageExtractionError::Other { .. } => MessageProcessorError::ProcessMessage(
                DispatchError::Other("Other error while processing v2 message"),
            ),
        }
    }
}

#[derive(Encode, Decode, Clone, Debug)]
pub enum RawPayload {
    Xcm(Vec<u8>),
    Symbiotic(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct ExtractedXcmConstructionInfo<Call> {
    pub origin: H160,
    pub maybe_claimer: Option<Vec<u8>>,
    pub eth_value: u128,
    pub assets: Vec<EthereumAsset>,
    pub execution_fee_in_eth: u128,
    pub nonce: u64,
    pub user_xcm: Xcm<Call>,
}

fn reanchor_location_to_tanssi(
    eth_chain_universal_location: &InteriorLocation,
    tanssi_chain_universal_location: &InteriorLocation,
    location_anchored_to_eth: Location,
) -> Result<Location, LocationConversionError> {
    let tanssi_reanchored_to_eth = tanssi_chain_universal_location
        .clone()
        .into_location()
        .reanchored(&eth_chain_universal_location.clone().into(), &().into())
        .map_err(
            |original_location| LocationConversionError::UnableToReanchor {
                location: original_location,
            },
        )?;
    location_anchored_to_eth
        .reanchored(&tanssi_reanchored_to_eth, eth_chain_universal_location)
        .map_err(
            |original_location| LocationConversionError::UnableToReanchor {
                location: original_location,
            },
        )
}

pub fn derive_asset_transfer_eth_asset<T>(
    eth_network_id: NetworkId,
    eth_chain_universal_location: &InteriorLocation,
    asset: &EthereumAsset,
    tanssi_chain_universal_location: &InteriorLocation,
) -> Result<AssetTransfer, LocationConversionError>
where
    T: snowbridge_pallet_system::Config,
{
    match asset {
        // Native to eth
        EthereumAsset::NativeTokenERC20 { token_id, value } => {
            let asset_location = reanchor_location_to_tanssi(
                eth_chain_universal_location,
                tanssi_chain_universal_location,
                (AccountKey20 {
                    network: Some(eth_network_id),
                    key: token_id.0,
                })
                .into(),
            )?;
            let asset: Asset = (asset_location, *value).into();
            Ok(AssetTransfer::ReserveDeposit(asset))
        }
        // Foreign to eth
        EthereumAsset::ForeignTokenERC20 { token_id, value } => {
            let token_location_reanchored_to_eth = snowbridge_pallet_system::Pallet::<T>::convert(
                &token_id,
            )
            .ok_or(LocationConversionError::UnableToConvertTokenId {
                token_id: *token_id,
            })?;
            let asset_location = reanchor_location_to_tanssi(
                eth_chain_universal_location,
                tanssi_chain_universal_location,
                token_location_reanchored_to_eth,
            )?;
            let asset: Asset = (asset_location, *value).into();
            Ok(AssetTransfer::ReserveWithdraw(asset))
        }
    }
}

pub fn derive_asset_for_native_eth(
    eth_chain_universal_location: &InteriorLocation,
    tanssi_chain_universal_location: &InteriorLocation,
    value: u128,
) -> Result<Asset, LocationConversionError> {
    let native_eth_reanchored_to_tanssi = reanchor_location_to_tanssi(
        eth_chain_universal_location,
        tanssi_chain_universal_location,
        ().into(),
    )?;
    Ok((native_eth_reanchored_to_tanssi, value).into())
}

pub fn derive_asset_transfers<T>(
    eth_network_id: NetworkId,
    eth_chain_universal_location: &InteriorLocation,
    tanssi_chain_universal_location: &InteriorLocation,
    assets: Vec<EthereumAsset>,
    eth_asset: u128,
) -> Result<Vec<AssetTransfer>, LocationConversionError>
where
    T: snowbridge_pallet_system::Config,
{
    let mut asset_transfers = vec![];
    for asset in assets {
        let asset_transfer = derive_asset_transfer_eth_asset::<T>(
            eth_network_id,
            eth_chain_universal_location,
            &asset,
            tanssi_chain_universal_location,
        )?;
        asset_transfers.push(asset_transfer);
    }

    if eth_asset > 0 {
        let native_eth_asset = derive_asset_for_native_eth(
            eth_chain_universal_location,
            tanssi_chain_universal_location,
            eth_asset,
        )?;
        asset_transfers.push(AssetTransfer::ReserveDeposit(native_eth_asset));
    }

    Ok(asset_transfers)
}

pub fn prepare_raw_message_xcm_instructions<T>(
    eth_network_id: NetworkId,
    eth_chain_universal_location: &InteriorLocation,
    tanssi_chain_universal_location: &InteriorLocation,
    gateway_proxy_address: H160,
    default_claimer: T::AccountId,
    topic_prefix: &str,
    extracted_xcm_construction_info: ExtractedXcmConstructionInfo<
        <T as pallet_xcm::Config>::RuntimeCall,
    >,
) -> Result<Vec<Instruction<<T as pallet_xcm::Config>::RuntimeCall>>, LocationConversionError>
where
    T: snowbridge_pallet_system::Config + pallet_xcm::Config,
    [u8; 32]: From<<T as frame_system::Config>::AccountId>,
{
    let ExtractedXcmConstructionInfo {
        origin,
        maybe_claimer,
        eth_value,
        assets,
        execution_fee_in_eth,
        nonce,
        user_xcm,
    } = extracted_xcm_construction_info;

    let claimer = maybe_claimer
        // Get the claimer from the message,
        .and_then(|claimer_bytes| Location::decode(&mut claimer_bytes.as_ref()).ok())
        // or use default claimer passed
        .unwrap_or_else(|| {
            Location::new(
                0,
                [AccountId32 {
                    network: None,
                    id: default_claimer.clone().into(),
                }],
            )
        });

    // derive asset transfers
    let asset_transfers = derive_asset_transfers::<T>(
        eth_network_id,
        eth_chain_universal_location,
        tanssi_chain_universal_location,
        assets,
        eth_value,
    )?;

    let mut instructions = vec![SetHints {
        hints: vec![AssetClaimer { location: claimer }]
            .try_into()
            .expect("checked statically, qed"),
    }];

    if execution_fee_in_eth > 0 {
        let execution_fee_asset = derive_asset_for_native_eth(
            eth_chain_universal_location,
            tanssi_chain_universal_location,
            execution_fee_in_eth,
        )?;
        instructions.push(ReserveAssetDeposited(execution_fee_asset.clone().into()));
    }

    let mut reserve_deposit_assets = vec![];
    let mut reserve_withdraw_assets = vec![];

    for asset in asset_transfers {
        match asset {
            AssetTransfer::ReserveDeposit(asset) => reserve_deposit_assets.push(asset),
            AssetTransfer::ReserveWithdraw(asset) => reserve_withdraw_assets.push(asset),
        };
    }

    if !reserve_deposit_assets.is_empty() {
        instructions.push(ReserveAssetDeposited(reserve_deposit_assets.into()));
    }
    if !reserve_withdraw_assets.is_empty() {
        instructions.push(WithdrawAsset(reserve_withdraw_assets.into()));
    }

    // Append DescendOrigin
    if origin != gateway_proxy_address {
        instructions.push(DescendOrigin(
            AccountKey20 {
                key: origin.into(),
                network: None,
            }
            .into(),
        ));
    }

    // Append raw xcm
    instructions.extend(user_xcm.0);

    // Add SetTopic instruction if not already present as the last instruction
    if !matches!(instructions.last(), Some(SetTopic(_))) {
        let topic = blake2_256(&(topic_prefix, nonce).encode());
        instructions.push(SetTopic(topic));
    }

    Ok(instructions)
}

pub fn execute_xcm<T, XcmProcessor, XcmWeigher>(
    origin: impl Into<Location>,
    mut xcm: Xcm<<T as pallet_xcm::Config>::RuntimeCall>,
    max_weight: Weight,
) -> Result<(), InstructionError>
where
    T: pallet_xcm::Config,
    XcmProcessor: ExecuteXcm<<T as pallet_xcm::Config>::RuntimeCall>,
    XcmWeigher: WeightBounds<<T as pallet_xcm::Config>::RuntimeCall>,
{
    // Calculate weight with the provided limit
    let weight = XcmWeigher::weight(&mut xcm, max_weight)?;

    // Ensure calculated weight doesn't exceed max_weight
    if weight.any_gt(max_weight) {
        log::error!(
            "XCM execution weight {:?} exceeds max allowed {:?}",
            weight,
            max_weight
        );
        return Err(InstructionError {
            index: 0,
            error: xcm::latest::Error::WeightLimitReached(weight),
        });
    }

    let mut message_id = xcm.using_encoded(blake2_256);

    XcmProcessor::prepare_and_execute(origin, xcm, &mut message_id, weight, weight)
        .ensure_complete()
}

fn calculate_message_hash(message: &Message) -> [u8; 32] {
    blake2_256(message.encode().as_slice())
}

pub trait FallbackMessageProcessor<AccountId> {
    fn handle_message(who: AccountId, message: Message) -> Result<(), MessageProcessorError>;
}

pub trait MessageProcessorWithFallback<AccountId> {
    type Fallback: FallbackMessageProcessor<AccountId>;
    type ExtractedMessage;

    fn try_extract_message(
        sender: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError>;

    fn process_extracted_message(
        sender: AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<(), MessageProcessorError>;

    fn calculate_message_id(message: &Message) -> [u8; 32] {
        calculate_message_hash(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::processors::v2::reanchor_location_to_tanssi;
    use xcm::latest::prelude::*;

    #[test]
    fn reanchor_works_for_tanssi_interior() {
        tanssi_interior_reanchor_test(true);
        tanssi_interior_reanchor_test(false);
    }

    fn tanssi_interior_reanchor_test(should_reanchor_tanssi_location: bool) {
        let context: InteriorLocation = GlobalConsensus(Ethereum { chain_id: 4 }).into();
        let mut tanssi_location: Location = GlobalConsensus(ByGenesis([2; 32])).into();
        let mut tanssi_interior_anchored_to_eth: Location = (
            Parent,
            GlobalConsensus(ByGenesis([2; 32])),
            AccountId32 {
                network: None,
                id: [1; 32],
            },
        )
            .into();
        let expected = AccountId32 {
            network: None,
            id: [1; 32],
        }
        .into();
        let generated_by_func = reanchor_location_to_tanssi(
            &context,
            &tanssi_location.interior,
            tanssi_interior_anchored_to_eth.clone(),
        )
        .unwrap();
        assert_eq!(generated_by_func, expected);

        if should_reanchor_tanssi_location {
            tanssi_location
                .reanchor(&context.clone().into(), &().into())
                .unwrap();
        }

        let target = tanssi_location;
        tanssi_interior_anchored_to_eth
            .reanchor(&target, &context)
            .unwrap();
        if should_reanchor_tanssi_location {
            assert_eq!(tanssi_interior_anchored_to_eth, expected);
        } else {
            assert_ne!(tanssi_interior_anchored_to_eth, expected);
        }
    }

    #[test]
    fn reanchor_works_for_eth_interior() {
        eth_interior_reanchor_test(true);
        eth_interior_reanchor_test(false);
    }

    fn eth_interior_reanchor_test(should_reanchor_tanssi_location: bool) {
        let context: InteriorLocation = GlobalConsensus(Ethereum { chain_id: 4 }).into();
        let mut tanssi_location: Location = GlobalConsensus(ByGenesis([2; 32])).into();
        let mut eth_interior: Location = (AccountKey20 {
            network: Some(Ethereum { chain_id: 4 }),
            key: [5; 20],
        })
        .into();
        let expected = (
            Parent,
            GlobalConsensus(Ethereum { chain_id: 4 }),
            AccountKey20 {
                network: Some(Ethereum { chain_id: 4 }),
                key: [5; 20],
            },
        )
            .into();

        let generated_by_func =
            reanchor_location_to_tanssi(&context, &tanssi_location.interior, eth_interior.clone())
                .unwrap();
        assert_eq!(generated_by_func, expected);

        if should_reanchor_tanssi_location {
            tanssi_location
                .reanchor(&context.clone().into(), &().into())
                .unwrap();
        }

        let target = tanssi_location;
        eth_interior.reanchor(&target, &context).unwrap();

        if should_reanchor_tanssi_location {
            assert_eq!(eth_interior, expected);
        } else {
            assert_ne!(eth_interior, expected);
        }
    }
}
