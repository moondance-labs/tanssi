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
    ethabi::{encode, Token},
    hex,
    parity_scale_codec::Encode,
    snowbridge_core::TokenIdOf,
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, MessageV1, VersionedXcmMessage,
    },
    sp_core::H160,
    std::sync::Arc,
    xcm::latest::prelude::*,
    xcm::latest::Junctions,
    xcm::latest::Location,
    xcm::prelude::InteriorLocation,
    xcm::v5::NetworkId,
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
}

#[derive(Debug, clap::Parser)]
pub struct PayloadGeneratorCmd {
    /// token_location: "Here" or "Parachain:<id>,PalletInstance:<id>"
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

    /// Token type: native or erc20
    #[arg(long, value_enum)]
    pub token: TokenType,

    /// Only required if token = erc20
    #[arg(long)]
    pub token_address: Option<String>,
}

impl PayloadGeneratorCmd {
    pub fn run(&self) {
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
        };

        let payload_bytes = payload.encode();
        println!("\nPayload (bytes): {:?}", payload_bytes);
        let encoded = encode(&[Token::Uint(self.nonce.into()), Token::Bytes(payload_bytes)]);
        println!(
            "\nSnowbridgeVerificationPrimitivesLog.data (hex): 0x{}",
            hex::encode(encoded)
        );
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

        if token_location.eq_ignore_ascii_case("Here") {
            Location::here()
        } else if token_location.starts_with("Parachain:") {
            let parts: Vec<&str> = token_location.split(',').collect();
            let mut para_id: Option<u32> = None;
            let mut pallet_id: Option<u8> = None;

            for p in parts {
                if let Some(v) = p.strip_prefix("Parachain:") {
                    para_id = Some(v.parse().expect("invalid parachain id"));
                } else if let Some(v) = p.strip_prefix("PalletInstance:") {
                    pallet_id = Some(v.parse().expect("invalid pallet id"));
                }
            }

            let para_id = para_id.expect("Parachain missing");
            let pallet_id = pallet_id.expect("PalletInstance missing");

            Location {
                parents: 0,
                interior: Junctions::X2(Arc::new([Parachain(para_id), PalletInstance(pallet_id)])),
            }
        } else {
            panic!("Unsupported token-location format: {}", token_location);
        }
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
