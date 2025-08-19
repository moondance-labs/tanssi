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
            [SetFeesMode { jit_withdraw: true }, ExportMessage { .. }] => true,
            _ => false,
        };

        // Check all ExportMessage instructions
        let mut all_exports_valid = false;
        for instr in instructions {
            if let ExportMessage { xcm, .. } = instr {
                all_exports_valid = true;

                // Verify the exact expected sequence of instructions
                if xcm.0.len() != 5 {
                    all_exports_valid = false;
                    break;
                }

                // Check each instruction in order
                match &xcm.0[..] {
                    [ReserveAssetDeposited(_), ClearOrigin, BuyExecution { .. }, DepositAsset { beneficiary, .. }, SetTopic(_)] =>
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn barrier_duplicate_fees_mode_yields_error() {
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
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(SetFeesMode {
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

    #[test]
    fn barrier_rejects_if_not_parachain() {
        type RuntimeCall = ();

        let non_para_origin = Location {
            parents: 1,
            interior: Junctions::Here,
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: Xcm::new(),
        });

        frame_support::assert_err!(
            AllowExportMessageFromContainerChainBarrier::should_execute(
                &non_para_origin,
                msg.inner_mut(),
                Weight::zero(),
                &mut props
            ),
            ProcessMessageError::Unsupported
        );
    }

    #[test]
    fn barrier_rejects_if_first_level_instructions_wrong() {
        type RuntimeCall = ();

        let para_origin = Location {
            parents: 0,
            interior: Junctions::X1([Parachain(2000)].into()),
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: Xcm::new(),
        });
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });

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

    #[test]
    fn barrier_rejects_if_export_message_has_wrong_length() {
        type RuntimeCall = ();

        let para_origin = Location {
            parents: 0,
            interior: Junctions::X1([Parachain(2000)].into()),
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        // Only 2 instructions instead of 5
        let export_xcm = Xcm(vec![ReserveAssetDeposited(assets.clone()), ClearOrigin]);

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: export_xcm,
        });

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

    #[test]
    fn barrier_rejects_if_no_ethereum_beneficiary() {
        type RuntimeCall = ();

        let para_origin = Location {
            parents: 0,
            interior: Junctions::X1([Parachain(2000)].into()),
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let export_xcm = Xcm(vec![
            ReserveAssetDeposited(assets.clone()),
            ClearOrigin,
            BuyExecution {
                fees: assets.get(0).unwrap().clone(),
                weight_limit: WeightLimit::Unlimited,
            },
            DepositAsset {
                assets: Wild(WildAsset::AllCounted(1)),
                beneficiary: Here.into(), // no ETH address
            },
            SetTopic([0u8; 32]),
        ]);

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: export_xcm,
        });

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

    #[test]
    fn barrier_rejects_if_instructions_order_is_incorrect() {
        type RuntimeCall = ();

        let para_origin = Location {
            parents: 0,
            interior: Junctions::X1([Parachain(2000)].into()),
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let eth_beneficiary = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(NetworkId::Ethereum { chain_id: 11155111 }),
                    key: [1u8; 20],
                }]
                .into(),
            ),
        };

        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let export_xcm = Xcm(vec![
            SetTopic([0u8; 32]),
            ReserveAssetDeposited(assets.clone()),
            ClearOrigin,
            BuyExecution {
                fees: assets.get(0).unwrap().clone(),
                weight_limit: WeightLimit::Unlimited,
            },
            DepositAsset {
                assets: Wild(WildAsset::AllCounted(1)),
                beneficiary: eth_beneficiary.clone(),
            },
        ]);

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: export_xcm,
        });

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

    #[test]
    fn barrier_allows_valid_export_message() {
        type RuntimeCall = ();

        let para_origin = Location {
            parents: 0,
            interior: Junctions::X1([Parachain(2000)].into()),
        };
        let mut props = Properties {
            weight_credit: Weight::zero(),
            message_id: None,
        };

        let eth_beneficiary = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(NetworkId::Ethereum { chain_id: 11155111 }),
                    key: [1u8; 20],
                }]
                .into(),
            ),
        };

        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let export_xcm = Xcm(vec![
            ReserveAssetDeposited(assets.clone()),
            ClearOrigin,
            BuyExecution {
                fees: assets.get(0).unwrap().clone(),
                weight_limit: WeightLimit::Unlimited,
            },
            DepositAsset {
                assets: Wild(WildAsset::AllCounted(1)),
                beneficiary: eth_beneficiary.clone(),
            },
            SetTopic([0u8; 32]),
        ]);

        let mut msg: Xcm<RuntimeCall> = Xcm::new();
        msg.inner_mut().push(SetFeesMode { jit_withdraw: true });
        msg.inner_mut().push(ExportMessage {
            network: NetworkId::Ethereum { chain_id: 11155111 },
            destination: Here.into(),
            xcm: export_xcm,
        });

        assert_eq!(
            AllowExportMessageFromContainerChainBarrier::should_execute(
                &para_origin,
                msg.inner_mut(),
                Weight::zero(),
                &mut props
            ),
            Ok(())
        );
    }
}
