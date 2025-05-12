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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    crate::{
        xcm_config::{AssetId, ForeignAssetsInstance, XcmConfig},
        AccountId, Balances, ForeignAssetsCreator, Runtime,
    },
    frame_support::{parameter_types, traits::ConstU64},
    pallet_evm_precompile_balances_erc20::{Erc20BalancesPrecompile, Erc20Metadata},
    pallet_evm_precompile_batch::BatchPrecompile,
    pallet_evm_precompile_call_permit::CallPermitPrecompile,
    pallet_evm_precompile_modexp::Modexp,
    pallet_evm_precompile_proxy::{OnlyIsProxyAndProxy, ProxyPrecompile},
    pallet_evm_precompile_sha3fips::Sha3FIPS256,
    pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256},
    pallet_evm_precompile_xcm::PalletXcmPrecompile,
    pallet_evm_precompile_xcm_utils::{AllExceptXcmExecute, XcmUtilsPrecompile},
    pallet_evm_precompileset_assets_erc20::Erc20AssetsPrecompileSet,
    precompile_utils::precompile_set::{
        AcceptDelegateCall, AddressU64, CallableByContract, CallableByPrecompile, OnlyFrom,
        PrecompileAt, PrecompileSetBuilder, PrecompileSetStartingWith, PrecompilesInRangeInclusive,
        SubcallWithMaxNesting,
    },
    xcm_primitives::location_matcher::{ForeignAssetMatcher, SingleAddressMatcher},
};

/// ERC20 metadata for the native token.
pub struct NativeErc20Metadata;

impl Erc20Metadata for NativeErc20Metadata {
    /// Returns the name of the token.
    fn name() -> &'static str {
        "UNIT token"
    }

    /// Returns the symbol of the token.
    fn symbol() -> &'static str {
        "UNIT"
    }

    /// Returns the decimals places of the token.
    fn decimals() -> u8 {
        18
    }

    /// Must return `true` only if it represents the main native currency of
    /// the network. It must be the currency used in `pallet_evm`.
    fn is_native_currency() -> bool {
        true
    }
}

/// The asset precompile address prefix. Addresses that match against this prefix will be routed
/// to Erc20AssetsPrecompileSet being marked as foreign
pub const FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[255u8; 18];

/// Const to identify ERC20_BALANCES_PRECOMPILE address
pub const ERC20_BALANCES_PRECOMPILE: u64 = 2048;

/// System account size in bytes = Pallet_Name_Hash (16) + Storage_name_hash (16) +
/// Blake2_128Concat (16) + AccountId (20) + AccountInfo (4 + 12 + AccountData (4* 16)) = 148
pub const SYSTEM_ACCOUNT_SIZE: u64 = 148;

parameter_types! {
    pub ForeignAssetPrefix: &'static [u8] = FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX;
}

type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);

// Pallet-xcm precompile types.
// The pallet-balances address is identified by ERC20_BALANCES_PRECOMPILE const
type BalancesPrecompileMatch = SingleAddressMatcher<AccountId, ERC20_BALANCES_PRECOMPILE, Balances>;

// Type that matches an AccountId with a foreign asset address (if any)
type ForeignAssetMatch = ForeignAssetMatcher<AccountId, AssetId, Runtime, ForeignAssetsCreator>;

#[precompile_utils::precompile_name_from_address]
type TemplatePrecompilesAt<R> = (
    // Ethereum precompiles:
    // Allow DELEGATECALL to stay compliant with Ethereum behavior.
    PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
    PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
    PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
    PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
    PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
    // Non-template specific nor Ethereum precompiles :
    PrecompileAt<AddressU64<1024>, Sha3FIPS256, (CallableByContract, CallableByPrecompile)>,
    PrecompileAt<AddressU64<1025>, ECRecoverPublicKey, (CallableByContract, CallableByPrecompile)>,
    // Template specific precompiles:
    PrecompileAt<
        AddressU64<ERC20_BALANCES_PRECOMPILE>,
        Erc20BalancesPrecompile<R, NativeErc20Metadata, ConstU64<SYSTEM_ACCOUNT_SIZE>>,
        (CallableByContract, CallableByPrecompile),
    >,
    PrecompileAt<AddressU64<2049>, BatchPrecompile<R>, SubcallWithMaxNesting<2>>,
    PrecompileAt<
        AddressU64<2050>,
        CallPermitPrecompile<R>,
        (SubcallWithMaxNesting<0>, CallableByContract),
    >,
    PrecompileAt<
        AddressU64<2051>,
        XcmUtilsPrecompile<R, XcmConfig, ConstU64<SYSTEM_ACCOUNT_SIZE>>,
        CallableByContract<AllExceptXcmExecute<R, XcmConfig, ConstU64<SYSTEM_ACCOUNT_SIZE>>>,
    >,
    PrecompileAt<
        AddressU64<2052>,
        PalletXcmPrecompile<R, (BalancesPrecompileMatch, ForeignAssetMatch)>,
        (CallableByContract, CallableByPrecompile),
    >,
    PrecompileAt<
        AddressU64<2053>,
        ProxyPrecompile<R>,
        (
            CallableByContract<OnlyIsProxyAndProxy<R>>,
            SubcallWithMaxNesting<0>,
            // Batch is the only precompile allowed to call Proxy.
            CallableByPrecompile<OnlyFrom<AddressU64<2049>>>,
        ),
    >,
);

pub type TemplatePrecompiles<R> = PrecompileSetBuilder<
    R,
    (
        PrecompilesInRangeInclusive<(AddressU64<1>, AddressU64<4095>), TemplatePrecompilesAt<R>>,
        // Prefixed precompile sets (XC20)
        PrecompileSetStartingWith<
            ForeignAssetPrefix,
            Erc20AssetsPrecompileSet<R, ForeignAssetsInstance>,
            (CallableByContract, CallableByPrecompile),
        >,
    ),
>;
