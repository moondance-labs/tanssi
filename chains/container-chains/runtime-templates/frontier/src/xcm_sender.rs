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
    crate::alloc::vec,
    core::marker::PhantomData,
    frame_support::traits::{tokens::ConversionFromAssetBalance, Get},
    xcm::{latest::Location, prelude::*},
    xcm_builder::InspectMessageQueues,
    SendError::*,
};

fn is_ethereum_location(loc: &Location) -> bool {
    matches!(
        loc,
        Location {
            parents: 2,
            interior: Junctions::X1(juncs)
        } if juncs[0] == GlobalConsensus(crate::EthereumNetwork::get())
    )
}

fn calculate_container_fee_from_tanssi_amount(
    tanssi_fee: u128,
    tanssi_asset_id: crate::xcm_config::AssetId,
) -> Result<Asset, SendError> {
    let native_fee = crate::AssetRate::from_asset_balance(tanssi_fee, tanssi_asset_id)
        .map_err(|_| Transport("Rate for Tanssi token not found or invalid"))?;

    Ok(Asset {
        id: AssetId(Location::here()),
        fun: Fungible(native_fee),
    })
}

pub struct SovereignPaidRemoteExporter<Router, UniversalLocation>(
    PhantomData<(Router, UniversalLocation)>,
);
impl<Router: SendXcm, UniversalLocation: Get<InteriorLocation>> SendXcm
    for SovereignPaidRemoteExporter<Router, UniversalLocation>
{
    type Ticket = Router::Ticket;

    fn validate(
        dest: &mut Option<Location>,
        msg: &mut Option<Xcm<()>>,
    ) -> SendResult<Router::Ticket> {
        let d = dest.as_ref().ok_or(MissingArgument)?;
        // Check if the destination is an Ethereum location
        if !is_ethereum_location(&d.clone()) {
            return Err(Unroutable);
        }

        let xcm = msg.take().ok_or(MissingArgument)?;

        let export_instruction = ExportMessage {
            network: crate::EthereumNetwork::get(),
            destination: Here,
            xcm: xcm.clone(),
        };

        // Get the asset to transfer to Ethereum
        let withdrawn_assets = xcm
            .0
            .iter()
            .find_map(|instr| {
                if let WithdrawAsset(assets) = instr {
                    Some(assets)
                } else {
                    None
                }
            })
            .ok_or(Transport("No WithdrawAsset in XCM"))?;

        let assets_vec = withdrawn_assets.clone().into_inner();

        let asset = assets_vec
            .into_iter()
            .find_map(|asset| {
                if let Fungible(_) = asset.fun {
                    Some(asset)
                } else {
                    None
                }
            })
            .ok_or(Transport("No fungible asset found"))?;

        let tanssi_location: Location = Location {
            parents: 1,
            interior: Here,
        };

        // Check if Tanssi registered, to be able to pay fee
        let tanssi_asset_id =
            pallet_foreign_asset_creator::ForeignAssetToAssetId::<crate::Runtime>::get(
                tanssi_location,
            )
            .ok_or(Transport("Tanssi token not registered"))?;

        // Hardcoding Tanssi fee for now
        let tanssi_fee = 100_000_000_000u128;
        // Calculate fee in native tokens with conversion rate
        let container_fee =
            calculate_container_fee_from_tanssi_amount(tanssi_fee, tanssi_asset_id)?;

        // Prepare the message to send
        let message = Xcm(vec![
            WithdrawAsset(asset.clone().into()),
            BuyExecution {
                fees: container_fee.clone(),
                weight_limit: Unlimited,
            },
            DepositAsset {
                assets: asset.clone().into(),
                beneficiary: Location {
                    parents: 0,
                    interior: Junctions::X1(alloc::sync::Arc::new([Junction::AccountKey20 {
                        network: None,
                        key: crate::EthereumSovereignAccount::get().into(),
                    }])),
                },
            },
            export_instruction,
        ]);

        let tanssi_location = Location {
            parents: 1,
            interior: Here,
        };

        let (v, cost) =
            validate_send::<Router>(tanssi_location.into(), message).inspect_err(|err| {
                if let NotApplicable = err {
                    *msg = Some(xcm);
                }
            })?;

        Ok((v, cost))
    }

    fn deliver(ticket: Router::Ticket) -> Result<XcmHash, SendError> {
        Router::deliver(ticket)
    }
}

impl<Router: SendXcm, UniversalLocation: Get<InteriorLocation>> InspectMessageQueues
    for SovereignPaidRemoteExporter<Router, UniversalLocation>
{
    fn clear_messages() {
        todo!()
    }

    fn get_messages() -> vec::Vec<(VersionedLocation, vec::Vec<VersionedXcm<()>>)> {
        todo!()
    }
}
