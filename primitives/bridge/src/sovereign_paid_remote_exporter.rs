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
    core::marker::PhantomData,
    frame_support::traits::Get,
    sp_std::vec,
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

pub struct SovereignPaidRemoteExporter<Router, UniversalLocation, EthereumNetwork>(
    PhantomData<(Router, UniversalLocation, EthereumNetwork)>,
);
impl<Router: SendXcm, UniversalLocation: Get<InteriorLocation>, EthereumNetwork> SendXcm
    for SovereignPaidRemoteExporter<Router, UniversalLocation, EthereumNetwork>
where
    EthereumNetwork: Get<NetworkId>,
{
    type Ticket = Router::Ticket;

    fn validate(
        dest: &mut Option<Location>,
        msg: &mut Option<Xcm<()>>,
    ) -> SendResult<Router::Ticket> {
        let d = dest.as_ref().ok_or(MissingArgument)?;
        let xcm = msg.take().ok_or(MissingArgument)?;
        // Check if the destination is an Ethereum location
        if !is_ethereum_location(&d.clone(), EthereumNetwork::get()) {
            return Err(Unroutable);
        }

        // `xcm` should already end with `SetTopic` - if it does, then extract and derive into
        // an onward topic ID.
        let maybe_forward_id = match xcm.last() {
            Some(SetTopic(t)) => Some(*t),
            _ => None,
        };

        let fees_mode_instruction = SetFeesMode { jit_withdraw: true };

        let export_instruction = ExportMessage {
            network: EthereumNetwork::get(),
            destination: Here,
            xcm: xcm.clone(),
        };

        // Prepare the message to send
        let mut message = Xcm(vec![fees_mode_instruction, export_instruction]);

        let tanssi_location = Location {
            parents: 1,
            interior: Here,
        };

        if let Some(forward_id) = maybe_forward_id {
            message.0.push(SetTopic(forward_id));
        }

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

impl<Router: SendXcm, UniversalLocation: Get<InteriorLocation>, EthereumNetwork>
    InspectMessageQueues
    for SovereignPaidRemoteExporter<Router, UniversalLocation, EthereumNetwork>
{
    fn clear_messages() {}

    fn get_messages() -> vec::Vec<(VersionedLocation, vec::Vec<VersionedXcm<()>>)> {
        vec::Vec::new()
    }
}
