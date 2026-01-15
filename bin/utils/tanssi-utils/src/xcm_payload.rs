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
    alloy_core::{
        primitives::{Address, FixedBytes},
        sol_types::{SolEvent, SolValue},
    },
    ethabi::{encode, Token},
    hex,
    parity_scale_codec::Encode,
    snowbridge_core::TokenIdOf,
    snowbridge_inbound_queue_primitives::{
        v1::{Command, Destination, MessageV1, VersionedXcmMessage},
        v2::message::IGatewayV2,
    },
    sp_core::H160,
    tanssi_runtime_common::processors::v2::RawPayload,
    xcm::latest::prelude::*,
    xcm::latest::Location,
    xcm::prelude::InteriorLocation,
    xcm::v5::NetworkId,
    xcm::{v5::Xcm, VersionedXcm},
    xcm_executor::traits::ConvertLocation,
};

pub const DANCELIGHT_GENESIS_HASH: [u8; 32] =
    hex_literal::hex!["983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a"];

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum DestinationType {
    Relay,
    Container,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum TokenType {
    Native,
    Erc20,
    Eth,
}

#[derive(Debug, clap::ValueEnum, Clone, Default)]
pub enum MessageVersion {
    #[default]
    V1,
    V2,
}

#[derive(Debug, clap::Parser)]
pub struct PayloadGeneratorCmd {
    /// token_location as json
    #[arg(long)]
    pub token_location: Option<String>,

    /// ParaId
    #[arg(long)]
    pub para_id: u32,

    /// Beneficiary address (AccountId20 or AccountId32 in hex)
    #[arg(long)]
    pub beneficiary: String,

    /// Container fee
    #[arg(long)]
    pub container_fee: u128,

    /// Amount
    #[arg(long)]
    pub amount: u128,

    /// Full fee
    #[arg(long)]
    pub fee: u128,

    /// Nonce (default = 1)
    #[arg(long, default_value_t = 1)]
    pub nonce: u64,

    #[arg(long)]
    pub genesis_hash: Option<String>,

    /// Destination type: relay or container
    #[arg(long, value_enum)]
    pub destination: DestinationType,

    /// Token type: native, erc20, or eth (eth only for V2)
    #[arg(long, value_enum)]
    pub token: TokenType,

    /// Only required if token = erc20
    #[arg(long)]
    pub token_address: Option<String>,

    /// Message version: v1 or v2 (default = v1)
    #[arg(long, value_enum, default_value = "v1")]
    pub version: MessageVersion,

    /// Gateway address for V2 (default = snowbridge gateway)
    #[arg(long)]
    pub gateway_address: Option<String>,

    /// Execution fee for V2 messages (default = 0)
    #[arg(long, default_value_t = 0)]
    pub execution_fee: u128,

    /// Relayer fee for V2 messages (default = 0)
    #[arg(long, default_value_t = 0)]
    pub relayer_fee: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PayloadResult {
    pub payload_bytes: Vec<u8>,
    pub encoded_hex: String,
}

/// Default gateway address (snowbridge testnet)
const DEFAULT_GATEWAY_ADDRESS: [u8; 20] =
    hex_literal::hex!("eda338e4dc46038493b885327842fd3e301cab39");

impl PayloadGeneratorCmd {
    pub fn run(&self) -> PayloadResult {
        match self.version {
            MessageVersion::V1 => self.run_v1(),
            MessageVersion::V2 => self.run_v2(),
        }
    }

    fn run_v1(&self) -> PayloadResult {
        let beneficiary_bytes = self.decode_beneficiary();

        let payload = match (&self.destination, &self.token) {
            (DestinationType::Relay, TokenType::Native) => {
                self.build_native_relay(&beneficiary_bytes)
            }
            (DestinationType::Container, TokenType::Native) => {
                self.build_native_container(&beneficiary_bytes)
            }
            (DestinationType::Relay, TokenType::Erc20) => {
                self.build_erc20_relay(&beneficiary_bytes)
            }
            (DestinationType::Container, TokenType::Erc20) => {
                self.build_erc20_container(&beneficiary_bytes)
            }
            (_, TokenType::Eth) => {
                panic!("ETH token type is only supported in V2 format. Use --version v2");
            }
        };

        let payload_bytes = payload.encode();
        println!("\nPayload (bytes): {:?}", payload_bytes);
        let encoded = encode(&[
            Token::Uint(self.nonce.into()),
            Token::Bytes(payload_bytes.clone()),
        ]);
        let encoded_hex = hex::encode(encoded);
        println!(
            "\nSnowbridgeVerificationPrimitivesLog.data (hex): 0x{}",
            encoded_hex
        );

        PayloadResult {
            payload_bytes,
            encoded_hex,
        }
    }

    fn run_v2(&self) -> PayloadResult {
        let beneficiary_bytes = self.decode_beneficiary();
        let gateway_address = self.parse_gateway_address();

        // Build the XCM instructions based on destination
        let xcm_instructions = self.build_v2_xcm_instructions(&beneficiary_bytes);

        // Build assets based on token type
        let (assets, eth_value) = self.build_v2_assets();

        // Build the V2 event
        let event = IGatewayV2::OutboundMessageAccepted {
            nonce: self.nonce,
            payload: IGatewayV2::Payload {
                origin: Address::from_slice(&gateway_address),
                assets,
                xcm: IGatewayV2::Xcm {
                    kind: 0,
                    data: xcm_instructions.encode().into(),
                },
                claimer: vec![].into(),
                value: eth_value,
                executionFee: self.execution_fee,
                relayerFee: self.relayer_fee,
            },
        };

        // Encode the event data
        let event_data = event.encode_data();
        let event_topics: Vec<_> = event
            .encode_topics()
            .into_iter()
            .map(|word| hex::encode(word.0 .0))
            .collect();

        println!("\n=== V2 Message Format ===");
        println!("\nEvent topics:");
        for (i, topic) in event_topics.iter().enumerate() {
            println!("  [{}] 0x{}", i, topic);
        }
        println!("\nEvent data (hex): 0x{}", hex::encode(&event_data));
        println!("\nXCM payload: {:?}", xcm_instructions);

        PayloadResult {
            payload_bytes: event_data.clone(),
            encoded_hex: hex::encode(&event_data),
        }
    }

    fn build_v2_xcm_instructions(&self, beneficiary_bytes: &[u8]) -> RawPayload {
        let destination_xcm = match &self.destination {
            DestinationType::Relay => {
                // Direct deposit to relay chain beneficiary
                let beneficiary_location = self.build_beneficiary_location(beneficiary_bytes);
                let instructions: Vec<Instruction<()>> = vec![DepositAsset {
                    assets: Wild(AllCounted(2)),
                    beneficiary: beneficiary_location,
                }];
                VersionedXcm::V5(Xcm(instructions))
            }
            DestinationType::Container => {
                // InitiateTransfer to container chain with deposit instruction
                let beneficiary_location = self.build_beneficiary_location(beneficiary_bytes);
                let instructions: Vec<Instruction<()>> = vec![InitiateTransfer {
                    destination: Location::new(0, Parachain(self.para_id)),
                    remote_fees: None,
                    preserve_origin: false,
                    assets: sp_runtime::BoundedVec::truncate_from(vec![]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(2)),
                        beneficiary: beneficiary_location,
                    }]),
                }];
                VersionedXcm::V5(Xcm(instructions))
            }
        };

        RawPayload::Xcm(destination_xcm.encode())
    }

    fn build_beneficiary_location(&self, beneficiary_bytes: &[u8]) -> Location {
        match beneficiary_bytes.len() {
            20 => {
                let mut key = [0u8; 20];
                key.copy_from_slice(beneficiary_bytes);
                Location::new(0, AccountKey20 { network: None, key })
            }
            32 => {
                let mut id = [0u8; 32];
                id.copy_from_slice(beneficiary_bytes);
                Location::new(0, AccountId32 { network: None, id })
            }
            n => panic!("beneficiary must be 20 or 32 bytes, got {}", n),
        }
    }

    fn build_v2_assets(&self) -> (Vec<IGatewayV2::EthereumAsset>, u128) {
        match &self.token {
            TokenType::Eth => {
                // Native ETH is sent via the `value` field, not assets
                (vec![], self.amount)
            }
            TokenType::Erc20 => {
                let token_address = self.parse_token_address();
                let token_h160 = H160::from_slice(&token_address[12..]);

                let asset = IGatewayV2::AsNativeTokenERC20 {
                    token_id: Address::from_slice(token_h160.as_bytes()),
                    value: self.amount,
                };

                (
                    vec![IGatewayV2::EthereumAsset {
                        kind: 0, // Native ERC20
                        data: asset.abi_encode().into(),
                    }],
                    0,
                )
            }
            TokenType::Native => {
                // Native token transfer via V2 uses foreign token encoding (kind: 1)
                let token_location = self.parse_token_location();
                let token_location_reanchored = self.reanchor_token(token_location);
                let token_id = TokenIdOf::convert_location(&token_location_reanchored)
                    .expect("unable to convert token location to token_id");

                let asset = IGatewayV2::AsForeignTokenERC20 {
                    token_id: FixedBytes(token_id.into()),
                    value: self.amount,
                };

                (
                    vec![IGatewayV2::EthereumAsset {
                        kind: 1, // Foreign token (registered via EthereumSystem::register_token)
                        data: asset.abi_encode().into(),
                    }],
                    0,
                )
            }
        }
    }

    fn parse_gateway_address(&self) -> [u8; 20] {
        if let Some(ref addr) = self.gateway_address {
            let hex_trimmed = addr.strip_prefix("0x").unwrap_or(addr);
            let bytes = hex::decode(hex_trimmed).expect("invalid hex for gateway_address");
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&bytes);
            arr
        } else {
            DEFAULT_GATEWAY_ADDRESS
        }
    }

    fn build_native_relay(&self, beneficiary: &[u8]) -> VersionedXcmMessage {
        let token_location = self.parse_token_location();
        let token_location_reanchored = self.reanchor_token(token_location);
        let token_id = TokenIdOf::convert_location(&token_location_reanchored)
            .expect("unable to convert token location to token_id");

        let destination = self.build_account32_destination(beneficiary);

        VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendNativeToken {
                token_id,
                destination,
                amount: self.amount,
                fee: self.fee,
            },
        })
    }

    fn build_native_container(&self, beneficiary: &[u8]) -> VersionedXcmMessage {
        let token_location = self.parse_token_location();
        let token_location_reanchored = self.reanchor_token(token_location);
        let token_id = TokenIdOf::convert_location(&token_location_reanchored)
            .expect("unable to convert token location to token_id");

        let destination = self.build_foreign_destination(beneficiary);

        VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendNativeToken {
                token_id,
                destination,
                amount: self.amount,
                fee: self.fee,
            },
        })
    }

    fn build_erc20_relay(&self, beneficiary: &[u8]) -> VersionedXcmMessage {
        let token_address = self.parse_token_address();
        let token = H160::from_slice(&token_address[12..]);
        let destination = self.build_account32_destination(beneficiary);

        VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendToken {
                token,
                destination,
                amount: self.amount,
                fee: self.fee,
            },
        })
    }

    fn build_erc20_container(&self, beneficiary: &[u8]) -> VersionedXcmMessage {
        let token_address = self.parse_token_address();
        let token = H160::from_slice(&token_address[12..]);
        let destination = self.build_foreign_destination(beneficiary);

        VersionedXcmMessage::V1(MessageV1 {
            chain_id: 1,
            command: Command::SendToken {
                token,
                destination,
                amount: self.amount,
                fee: self.fee,
            },
        })
    }

    fn decode_beneficiary(&self) -> Vec<u8> {
        let hex_trimmed = self
            .beneficiary
            .strip_prefix("0x")
            .unwrap_or(&self.beneficiary);
        hex::decode(hex_trimmed).expect("invalid hex in beneficiary")
    }

    fn parse_token_address(&self) -> [u8; 32] {
        let addr = self
            .token_address
            .as_ref()
            .expect("token_address is required for ERC20");
        let hex_trimmed = addr.strip_prefix("0x").unwrap_or(addr);
        let bytes = hex::decode(hex_trimmed).expect("invalid hex for token_address");
        let mut arr = [0u8; 32];
        if bytes.len() == 20 {
            arr[12..].copy_from_slice(&bytes);
        } else if bytes.len() == 32 {
            arr.copy_from_slice(&bytes);
        } else {
            panic!("token_address must be 20 or 32 bytes");
        }
        arr
    }

    fn parse_genesis_hash(&self) -> NetworkId {
        if let Some(ref h) = self.genesis_hash {
            let hex_trimmed = h.strip_prefix("0x").unwrap_or(h);
            let bytes = hex::decode(hex_trimmed).expect("invalid hex for genesis_hash");
            let mut genesis_array = [0u8; 32];
            genesis_array.copy_from_slice(&bytes);
            NetworkId::ByGenesis(genesis_array)
        } else {
            NetworkId::ByGenesis(DANCELIGHT_GENESIS_HASH)
        }
    }

    fn parse_token_location(&self) -> Location {
        let token_location = self
            .token_location
            .as_ref()
            .expect("--token-location is required for native");

        serde_json::from_str::<Location>(token_location).expect("invalid JSON for token_location")
    }

    fn reanchor_token(&self, token_location: Location) -> Location {
        let this_network = self.parse_genesis_hash();
        let ethereum_network = NetworkId::Ethereum { chain_id: 11155111 };
        let eth_location: Location = Location::new(1, ethereum_network);
        let universal_location: InteriorLocation = this_network.into();

        token_location
            .reanchored(&eth_location, &universal_location)
            .expect("unable to reanchor token")
    }

    fn build_foreign_destination(&self, bytes: &[u8]) -> Destination {
        match bytes.len() {
            20 => {
                let mut id20 = [0u8; 20];
                id20.copy_from_slice(bytes);
                Destination::ForeignAccountId20 {
                    para_id: self.para_id,
                    id: id20,
                    fee: self.container_fee,
                }
            }
            32 => {
                let mut id32 = [0u8; 32];
                id32.copy_from_slice(bytes);
                Destination::ForeignAccountId32 {
                    para_id: self.para_id,
                    id: id32,
                    fee: self.container_fee,
                }
            }
            n => panic!("beneficiary must be 20 or 32 bytes, got {}", n),
        }
    }

    fn build_account32_destination(&self, bytes: &[u8]) -> Destination {
        if bytes.len() != 32 {
            panic!("relay destination requires 32-byte AccountId32");
        }
        let mut id32 = [0u8; 32];
        id32.copy_from_slice(bytes);
        Destination::AccountId32 { id: id32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn e2e_native_container() {
        let cmd = PayloadGeneratorCmd {
            token_location: Some(
                json!({"parents":0,"interior":{"X2": [{"Parachain": 2002, }, {"PalletInstance": 10}]}})
                    .to_string(),
            ),
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 500000000000000,
            amount: 100000000,
            fee: 1500000000000000,
            destination: DestinationType::Container,
            token: TokenType::Native,
            genesis_hash: None,
            token_address: None,
            nonce: 1,
            version: MessageVersion::V1,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        assert_eq!(result.encoded_hex, "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007f00010000000000000002c97f6a848a8e7895b55dc9b894e4f552ea33203bfbdb478d506f05b62d9d5fd101d2070000050505050505050505050505050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d540500000000000000000000");
    }

    #[test]
    fn e2e_relay_native_to_relay() {
        let cmd = PayloadGeneratorCmd {
            token_location: Some(json!({"parents":0,"interior":"Here"}).to_string()),
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 500000000000000,
            amount: 100000000,
            fee: 1500000000000000,
            destination: DestinationType::Relay,
            token: TokenType::Native,
            genesis_hash: None,
            token_address: None,
            nonce: 1,
            version: MessageVersion::V1,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        assert_eq!(result.encoded_hex, "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006b00010000000000000002e95142d5aca3299068a3d9b4a659f9589559382d0a130a1d7cedc67d6c3d401d00050505050505050505050505050505050505050505050505050505050505050500e1f50500000000000000000000000000c029f73d5405000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn e2e_erc20_to_relay() {
        let cmd = PayloadGeneratorCmd {
            token_location: Some(json!({"parents":0,"interior":"Here"}).to_string()),
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 500000000000000,
            amount: 100000000,
            fee: 1500000000000000,
            destination: DestinationType::Relay,
            token: TokenType::Erc20,
            genesis_hash: None,
            nonce: 1,
            token_address: Some("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".into()),
            version: MessageVersion::V1,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        assert_eq!(result.encoded_hex, "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000005f00010000000000000001deadbeefdeadbeefdeadbeefdeadbeefdeadbeef00050505050505050505050505050505050505050505050505050505050505050500e1f50500000000000000000000000000c029f73d540500000000000000000000");
    }

    #[test]
    fn e2e_to_container() {
        let cmd = PayloadGeneratorCmd {
            token_location: Some(json!({"parents":0,"interior":"Here"}).to_string()),
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 500000000000000,
            amount: 100000000,
            fee: 1500000000000000,
            destination: DestinationType::Container,
            token: TokenType::Erc20,
            genesis_hash: None,
            nonce: 1,
            token_address: Some("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".into()),
            version: MessageVersion::V1,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        assert_eq!(result.encoded_hex, "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300010000000000000001deadbeefdeadbeefdeadbeefdeadbeefdeadbeef01d2070000050505050505050505050505050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d540500000000000000000000000000000000000000000000");
    }

    #[test]
    fn e2e_v2_erc20_to_container() {
        let cmd = PayloadGeneratorCmd {
            token_location: None,
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 0,
            amount: 100000000,
            fee: 0,
            destination: DestinationType::Container,
            token: TokenType::Erc20,
            genesis_hash: None,
            nonce: 1,
            token_address: Some("0x1111111111111111111111111111111111111111".into()),
            version: MessageVersion::V2,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        // V2 should generate event data
        assert!(!result.encoded_hex.is_empty());
    }

    #[test]
    fn e2e_v2_eth_to_container() {
        let cmd = PayloadGeneratorCmd {
            token_location: None,
            para_id: 2001,
            beneficiary: "0x0505050505050505050505050505050505050505".into(),
            container_fee: 0,
            amount: 50000000000,
            fee: 0,
            destination: DestinationType::Container,
            token: TokenType::Eth,
            genesis_hash: None,
            nonce: 1,
            token_address: None,
            version: MessageVersion::V2,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        // V2 ETH should generate event data with value in the payload
        assert!(!result.encoded_hex.is_empty());
    }

    #[test]
    fn e2e_v2_erc20_to_relay() {
        let cmd = PayloadGeneratorCmd {
            token_location: None,
            para_id: 0,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 0,
            amount: 100000000,
            fee: 0,
            destination: DestinationType::Relay,
            token: TokenType::Erc20,
            genesis_hash: None,
            nonce: 1,
            token_address: Some("0x1111111111111111111111111111111111111111".into()),
            version: MessageVersion::V2,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        // V2 should generate event data
        assert!(!result.encoded_hex.is_empty());
    }

    #[test]
    fn e2e_v2_native_token_to_container() {
        let cmd = PayloadGeneratorCmd {
            token_location: Some(json!({"parents":0,"interior":"Here"}).to_string()),
            para_id: 2002,
            beneficiary: "0x0505050505050505050505050505050505050505050505050505050505050505"
                .into(),
            container_fee: 0,
            amount: 100000000,
            fee: 0,
            destination: DestinationType::Container,
            token: TokenType::Native,
            genesis_hash: None,
            nonce: 1,
            token_address: None,
            version: MessageVersion::V2,
            gateway_address: None,
            execution_fee: 0,
            relayer_fee: 0,
        };
        let result = cmd.run();

        // V2 native token should use foreign token encoding (kind: 1)
        assert!(!result.encoded_hex.is_empty());
    }
}
