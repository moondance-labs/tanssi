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
    frame_support::traits::ProcessMessageError,
    sp_std::prelude::*,
    xcm::latest::Junction::Parachain,
    xcm::prelude::*,
    xcm_executor::traits::{Properties, ShouldExecute},
};

pub struct AllowExportMessageFromContainerChainBarrier;
impl ShouldExecute for AllowExportMessageFromContainerChainBarrier {
    fn should_execute<RuntimeCall>(
        origin: &Location,
        instructions: &mut [Instruction<RuntimeCall>],
        _max_weight: Weight,
        _properties: &mut Properties,
    ) -> Result<(), ProcessMessageError> {
        // Check the origin is parachain
        let is_from_parachain = match origin {
            Location {
                parents: 0,
                interior: Junctions::X1(junctions),
            } => {
                matches!(&**junctions, [Parachain(_)])
            }
            _ => false,
        };

        // Check if ExportMessage exists and destination - ETH
        let has_export_with_eth_beneficiary = instructions.iter().any(|instr| {
            if let Instruction::ExportMessage { xcm, .. } = instr {
                xcm.0.iter().any(|inner_instr| {
                    if let Instruction::DepositAsset { beneficiary, .. } = inner_instr {
                        beneficiary.interior().into_iter().any(|junction| {
                            matches!(
                                junction,
                                AccountKey20 {
                                    network: Some(NetworkId::Ethereum { .. }),
                                    ..
                                }
                            )
                        })
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        });

        if !(is_from_parachain && has_export_with_eth_beneficiary) {
            return Err(ProcessMessageError::Unsupported);
        }

        Ok(())
    }
}
