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

        // Strict check for first-level instructions
        let valid_first_level = match instructions {
            [Instruction::SetFeesMode { jit_withdraw: true }, Instruction::ExportMessage { .. }] => {
                true
            }
            _ => false,
        };

        // Check all ExportMessage instructions
        let mut all_exports_valid = false;
        for instr in instructions {
            if let Instruction::ExportMessage { xcm, .. } = instr {
                all_exports_valid = true;

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

        if !(is_from_parachain && valid_first_level && all_exports_valid) {
            log::trace!("validate params: is_from_parachain: {:?}, all_exports_valid: {:?}, valid_first_level: {:?}", is_from_parachain, all_exports_valid, valid_first_level);
            return Err(ProcessMessageError::Unsupported);
        }

        Ok(())
    }
}

#[test]
fn test_barrier_bypass() {
    type RuntimeCall = ();

    // We need to be a container chain to pass the barrier
    let para_origin = Location {
        parents: 0,
        interior: Junctions::X1([Parachain(2000)].into()),
    };
    // Whatever, the barrier doesn't care about this
    let mut props = Properties {
        weight_credit: Weight::zero(),
        message_id: None,
    };

    // This could be any malicous XCM, as long as it does not contain an ExportMessage instruction
    let mut msg: Xcm<RuntimeCall> = Xcm::new();

    // bypass fee payment by effectively setting jit_withdraw to false
    msg.inner_mut()
        .push(Instruction::SetFeesMode { jit_withdraw: true });
    msg.inner_mut().push(Instruction::SetFeesMode {
        jit_withdraw: false,
    });

    // Let's make sure we not pass the barrier
    frame_support::assert_err!(
        AllowExportMessageFromContainerChainBarrier::should_execute(
            &para_origin,
            msg.inner_mut(),
            Weight::zero(),
            &mut props
        ),
        ProcessMessageError::Unsupported
    );
}
