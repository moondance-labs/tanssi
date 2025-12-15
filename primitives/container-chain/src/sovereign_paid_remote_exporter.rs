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
    alloc::vec,
    core::marker::PhantomData,
    cumulus_primitives_core::ParaId,
    frame_support::traits::Get,
    xcm::{
        latest::{Location, NetworkId},
        prelude::*,
    },
    xcm_builder::InspectMessageQueues,
    SendError::*,
};

// We check if the destination is ETH
fn is_ethereum_location(loc: &Location, ethereum_network: NetworkId) -> bool {
    matches!(
        loc,
        Location {
            parents: 2,
            interior: Junctions::X1(juncs)
        } if juncs[0] == GlobalConsensus(ethereum_network)
    )
}

pub struct SovereignPaidRemoteExporter<
    Router,
    UniversalLocation,
    EthereumNetwork,
    ExecutionFee,
    SelfParaId,
>(
    PhantomData<(
        Router,
        UniversalLocation,
        EthereumNetwork,
        ExecutionFee,
        SelfParaId,
    )>,
);
impl<
        Router: SendXcm,
        UniversalLocation: Get<InteriorLocation>,
        EthereumNetwork,
        ExecutionFee,
        SelfParaId,
    > SendXcm
    for SovereignPaidRemoteExporter<
        Router,
        UniversalLocation,
        EthereumNetwork,
        ExecutionFee,
        SelfParaId,
    >
where
    EthereumNetwork: Get<NetworkId>,
    ExecutionFee: Get<u128>,
    SelfParaId: Get<ParaId>,
{
    type Ticket = Router::Ticket;

    fn validate(
        dest: &mut Option<Location>,
        msg: &mut Option<Xcm<()>>,
    ) -> SendResult<Router::Ticket> {
        log::trace!(target: "xcm::sovereign_paid_remote_exporter", "validate params: dest={dest:?}, msg={msg:?}");

        let d = dest.as_ref().ok_or(MissingArgument)?;
        let xcm = msg.take().ok_or(MissingArgument)?;
        // Check if the destination is an Ethereum location
        if !is_ethereum_location(&d.clone(), EthereumNetwork::get()) {
            return Err(NotApplicable);
        }

        // `xcm` should already end with `SetTopic` - if it does, then extract and derive into
        // an onward topic ID.
        let maybe_forward_id = match xcm.last() {
            Some(SetTopic(t)) => Some(*t),
            _ => None,
        };

        let export_instruction = ExportMessage {
            network: EthereumNetwork::get(),
            destination: Here,
            xcm: xcm.clone(),
        };

        // Check if xcm contains AliasOrigin instruction.
        // We use the presence of an AliasOrigin instruction to distinguish
        // between Snowbridge V2 and Snowbridge V1 messages.
        let has_alias_origin = xcm
            .0
            .iter()
            .any(|instruction| matches!(instruction, AliasOrigin(_)));

        // For now hardcoding fees, but later it should be converted from fixed Tanssi relay amount
        let fees = Asset {
            id: AssetId(Location::here()),
            fun: Fungible(ExecutionFee::get()),
        };
        let container_location = Location::new(0, Parachain(SelfParaId::get().into()));

        // Prepare the message to send
        let mut message_instructions = vec![
            WithdrawAsset(fees.clone().into()),
            BuyExecution {
                fees,
                weight_limit: Unlimited,
            },
        ];

        // Add SetFeesMode if AliasOrigin is present in the exported xcm
        if has_alias_origin {
            message_instructions.push(SetFeesMode { jit_withdraw: true });
        }

        message_instructions.push(SetAppendix(Xcm(vec![DepositAsset {
            assets: AllCounted(1).into(),
            beneficiary: container_location,
        }])));
        message_instructions.push(export_instruction);

        let mut message = Xcm(message_instructions);

        let tanssi_location = Location {
            parents: 1,
            interior: Here,
        };

        if let Some(forward_id) = maybe_forward_id {
            message.0.push(SetTopic(forward_id));
        }

        let (v, cost) = validate_send::<Router>(tanssi_location, message).inspect_err(|err| {
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

impl<
        Router: SendXcm,
        UniversalLocation: Get<InteriorLocation>,
        EthereumNetwork,
        ExecutionFee,
        SelfParaId,
    > InspectMessageQueues
    for SovereignPaidRemoteExporter<
        Router,
        UniversalLocation,
        EthereumNetwork,
        ExecutionFee,
        SelfParaId,
    >
{
    fn clear_messages() {}

    fn get_messages() -> vec::Vec<(VersionedLocation, vec::Vec<VersionedXcm<()>>)> {
        vec::Vec::new()
    }
}
