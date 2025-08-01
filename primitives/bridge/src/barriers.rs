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

        // Check if fees mode is set to JIT withdraw
        let has_set_fees_mode_jit_true = instructions
            .iter()
            .any(|instr| matches!(instr, Instruction::SetFeesMode { jit_withdraw: true }));

        // Check all ExportMessage instructions
        let mut all_exports_valid = true;
        for instr in instructions {
            if let Instruction::ExportMessage { xcm, .. } = instr {
                // Verify the exact expected sequence of instructions
                if xcm.0.len() != 5 {
                    all_exports_valid = false;
                    break;
                }

                // Check each instruction in order
                match &xcm.0[..] {
                    [Instruction::ReserveAssetDeposited(_), Instruction::ClearOrigin, Instruction::BuyExecution { .. }, Instruction::DepositAsset { beneficiary, .. }, Instruction::SetTopic(_)] =>
                    {
                        // Additional check for Ethereum beneficiary
                        let has_eth_beneficiary =
                            beneficiary.interior().into_iter().any(|junction| {
                                matches!(
                                    junction,
                                    AccountKey20 {
                                        network: Some(NetworkId::Ethereum { .. }),
                                        ..
                                    }
                                )
                            });
                        if !has_eth_beneficiary {
                            all_exports_valid = false;
                            break;
                        }
                    }
                    _ => {
                        all_exports_valid = false;
                        break;
                    }
                }
            }
        }

        if !(is_from_parachain && all_exports_valid && has_set_fees_mode_jit_true) {
            log::trace!("validate params: is_from_parachain: {:?}, all_exports_valid: {:?}, has_set_fees_mode_jit_true: {:?}", is_from_parachain, all_exports_valid, has_set_fees_mode_jit_true);
            return Err(ProcessMessageError::Unsupported);
        }

        Ok(())
    }
}
