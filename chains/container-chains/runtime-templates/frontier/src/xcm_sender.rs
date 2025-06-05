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
            parents: 1,
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

        // We need 32 bytes address, so convert 20 bytes to 32 by adding empty bytes
        let eth_20: fp_account::AccountId20 = crate::EthereumSovereignAccount::get();
        let mut eth_32 = [0u8; 32];
        eth_32[12..32].copy_from_slice(&eth_20.as_ref());

        // Prepare the message to send
        let message = Xcm(vec![
            WithdrawAsset(asset.clone().into()),
            DepositAsset {
                assets: asset.clone().into(),
                beneficiary: Location {
                    parents: 0,
                    interior: Junctions::X1(alloc::sync::Arc::new([Junction::AccountId32 {
                        network: None,
                        id: eth_32,
                    }])),
                },
            },
            WithdrawAsset(Assets::from(vec![container_fee.clone()])),
            BuyExecution {
                fees: container_fee.clone(),
                weight_limit: Unlimited,
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
