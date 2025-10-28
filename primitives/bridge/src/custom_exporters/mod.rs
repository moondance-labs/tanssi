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

pub mod container_token_to_ethereum_message_exporter;
pub mod snowbridge_outbound_token_transfer;
pub mod snowbridge_outbound_token_transfer_v2;

pub use container_token_to_ethereum_message_exporter::*;
pub use snowbridge_outbound_token_transfer::*;
pub use snowbridge_outbound_token_transfer_v2::*;

#[derive(PartialEq, Debug)]
pub enum XcmConverterError {
    UnexpectedEndOfXcm,
    EndOfXcmMessageExpected,
    WithdrawAssetExpected,
    DepositAssetExpected,
    NoReserveAssets,
    FilterDoesNotConsumeAllAssets,
    TooManyAssets,
    ZeroAssetTransfer,
    BeneficiaryResolutionFailed,
    AssetResolutionFailed,
    InvalidFeeAsset,
    SetTopicExpected,
    ReserveAssetDepositedExpected,
    InvalidAsset,
    UnexpectedInstruction,
    AliasOriginExpected,
    InvalidOrigin,
    TooManyCommands,
}
