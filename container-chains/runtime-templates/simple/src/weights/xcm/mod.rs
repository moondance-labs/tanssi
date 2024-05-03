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

pub mod pallet_xcm_benchmarks_generic;

use {
    crate::Runtime,
    frame_support::weights::Weight,
    pallet_xcm_benchmarks_generic::WeightInfo as XcmGeneric,
    sp_std::prelude::*,
    staging_xcm::{
        latest::{prelude::*, Weight as XCMWeight},
        DoubleEncoded,
    },
};

trait WeighMultiAssets {
    fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight;
}

impl WeighMultiAssets for MultiAssets {
    fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight {
        weight.saturating_mul(self.inner().iter().count() as u64)
    }
}

// Values copied from statemint benchmarks
const ASSET_BURN_MAX_PROOF_SIZE: u64 = 7242;
const ASSET_MINT_MAX_PROOF_SIZE: u64 = 7242;
const ASSET_TRANSFER_MAX_PROOF_SIZE: u64 = 13412;

// For now we are returning benchmarked weights only for generic XCM instructions.
// Fungible XCM instructions will return a fixed weight value of
// 200_000_000 ref_time and its proper PoV weight taken from statemint benchmarks.
//
// TODO: add the fungible benchmarked values once these are calculated.
pub struct XcmWeight<RuntimeCall>(core::marker::PhantomData<RuntimeCall>);
impl<RuntimeCall> XcmWeightInfo<RuntimeCall> for XcmWeight<RuntimeCall>
where
    Runtime: frame_system::Config,
{
    fn withdraw_asset(assets: &MultiAssets) -> XCMWeight {
        assets.weigh_multi_assets(XCMWeight::from_parts(
            200_000_000u64,
            ASSET_BURN_MAX_PROOF_SIZE,
        ))
    }
    fn reserve_asset_deposited(assets: &MultiAssets) -> XCMWeight {
        assets.weigh_multi_assets(XCMWeight::from_parts(200_000_000u64, 0))
    }
    fn receive_teleported_asset(_assets: &MultiAssets) -> XCMWeight {
        XCMWeight::MAX
    }
    fn query_response(
        _query_id: &u64,
        _response: &Response,
        _max_weight: &Weight,
        _querier: &Option<MultiLocation>,
    ) -> XCMWeight {
        XcmGeneric::<Runtime>::query_response()
    }
    fn transfer_asset(assets: &MultiAssets, _dest: &MultiLocation) -> XCMWeight {
        assets.weigh_multi_assets(XCMWeight::from_parts(
            200_000_000u64,
            ASSET_TRANSFER_MAX_PROOF_SIZE,
        ))
    }
    fn transfer_reserve_asset(
        assets: &MultiAssets,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> XCMWeight {
        assets.weigh_multi_assets(XCMWeight::from_parts(
            200_000_000u64,
            ASSET_TRANSFER_MAX_PROOF_SIZE,
        ))
    }
    fn transact(
        _origin_type: &OriginKind,
        _require_weight_at_most: &Weight,
        _call: &DoubleEncoded<RuntimeCall>,
    ) -> XCMWeight {
        XcmGeneric::<Runtime>::transact()
    }
    fn hrmp_new_channel_open_request(
        _sender: &u32,
        _max_message_size: &u32,
        _max_capacity: &u32,
    ) -> XCMWeight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn hrmp_channel_accepted(_recipient: &u32) -> XCMWeight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn hrmp_channel_closing(_initiator: &u32, _sender: &u32, _recipient: &u32) -> XCMWeight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn clear_origin() -> XCMWeight {
        XcmGeneric::<Runtime>::clear_origin()
    }
    fn descend_origin(_who: &InteriorMultiLocation) -> XCMWeight {
        XcmGeneric::<Runtime>::descend_origin()
    }
    fn report_error(_query_response_info: &QueryResponseInfo) -> XCMWeight {
        XcmGeneric::<Runtime>::report_error()
    }
    fn deposit_asset(_assets: &MultiAssetFilter, _dest: &MultiLocation) -> XCMWeight {
        Weight::from_parts(200_000_000u64, ASSET_MINT_MAX_PROOF_SIZE)
    }
    fn deposit_reserve_asset(
        _assets: &MultiAssetFilter,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> XCMWeight {
        Weight::from_parts(200_000_000u64, ASSET_MINT_MAX_PROOF_SIZE)
    }
    fn exchange_asset(
        _give: &MultiAssetFilter,
        _receive: &MultiAssets,
        _maximal: &bool,
    ) -> XCMWeight {
        Weight::MAX
    }
    fn initiate_reserve_withdraw(
        _assets: &MultiAssetFilter,
        _reserve: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> XCMWeight {
        XCMWeight::from_parts(200_000_000u64, ASSET_TRANSFER_MAX_PROOF_SIZE)
    }
    fn initiate_teleport(
        _assets: &MultiAssetFilter,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> XCMWeight {
        XCMWeight::MAX
    }
    fn report_holding(_response_info: &QueryResponseInfo, _assets: &MultiAssetFilter) -> Weight {
        XcmGeneric::<Runtime>::report_holding()
    }
    fn buy_execution(_fees: &MultiAsset, _weight_limit: &WeightLimit) -> XCMWeight {
        XcmGeneric::<Runtime>::buy_execution()
    }
    fn refund_surplus() -> XCMWeight {
        XcmGeneric::<Runtime>::refund_surplus()
    }
    fn set_error_handler(_xcm: &Xcm<RuntimeCall>) -> XCMWeight {
        XcmGeneric::<Runtime>::set_error_handler()
    }
    fn set_appendix(_xcm: &Xcm<RuntimeCall>) -> XCMWeight {
        XcmGeneric::<Runtime>::set_appendix()
    }
    fn clear_error() -> XCMWeight {
        XcmGeneric::<Runtime>::clear_error()
    }
    fn claim_asset(assets: &MultiAssets, _ticket: &MultiLocation) -> XCMWeight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::claim_asset())
    }
    fn trap(_code: &u64) -> XCMWeight {
        XcmGeneric::<Runtime>::trap()
    }
    fn subscribe_version(_query_id: &QueryId, _max_response_weight: &Weight) -> XCMWeight {
        XcmGeneric::<Runtime>::subscribe_version()
    }
    fn unsubscribe_version() -> XCMWeight {
        XcmGeneric::<Runtime>::unsubscribe_version()
    }
    fn burn_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::burn_asset())
    }
    fn expect_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::expect_asset())
    }
    fn expect_origin(_origin: &Option<MultiLocation>) -> Weight {
        XcmGeneric::<Runtime>::expect_origin()
    }
    fn expect_error(_error: &Option<(u32, XcmError)>) -> Weight {
        XcmGeneric::<Runtime>::expect_error()
    }
    fn expect_transact_status(_transact_status: &MaybeErrorCode) -> Weight {
        XcmGeneric::<Runtime>::expect_transact_status()
    }
    fn query_pallet(_module_name: &Vec<u8>, _response_info: &QueryResponseInfo) -> Weight {
        XcmGeneric::<Runtime>::query_pallet()
    }
    fn expect_pallet(
        _index: &u32,
        _name: &Vec<u8>,
        _module_name: &Vec<u8>,
        _crate_major: &u32,
        _min_crate_minor: &u32,
    ) -> Weight {
        XcmGeneric::<Runtime>::expect_pallet()
    }
    fn report_transact_status(_response_info: &QueryResponseInfo) -> Weight {
        XcmGeneric::<Runtime>::report_transact_status()
    }
    fn clear_transact_status() -> Weight {
        XcmGeneric::<Runtime>::clear_transact_status()
    }
    fn universal_origin(_: &Junction) -> Weight {
        Weight::MAX
    }
    fn export_message(_: &NetworkId, _: &Junctions, _: &Xcm<()>) -> Weight {
        Weight::MAX
    }
    fn lock_asset(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn unlock_asset(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn note_unlockable(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn request_unlock(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn set_fees_mode(_: &bool) -> Weight {
        XcmGeneric::<Runtime>::set_fees_mode()
    }
    fn set_topic(_topic: &[u8; 32]) -> Weight {
        XcmGeneric::<Runtime>::set_topic()
    }
    fn clear_topic() -> Weight {
        XcmGeneric::<Runtime>::clear_topic()
    }
    fn alias_origin(_: &MultiLocation) -> Weight {
        // XCM Executor does not currently support alias origin operations
        Weight::MAX
    }
    fn unpaid_execution(_: &WeightLimit, _: &Option<MultiLocation>) -> Weight {
        XcmGeneric::<Runtime>::unpaid_execution()
    }
}
