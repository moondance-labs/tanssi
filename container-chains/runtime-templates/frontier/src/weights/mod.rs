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

//! A list of the different weight modules for our runtime.

pub mod cumulus_pallet_dmp_queue;
pub mod cumulus_pallet_parachain_system;
pub mod cumulus_pallet_xcmp_queue;
pub mod frame_system;
pub mod pallet_asset_rate;
pub mod pallet_assets;
pub mod pallet_author_inherent;
pub mod pallet_balances;
pub mod pallet_cc_authorities_noting;
pub mod pallet_foreign_asset_creator;
pub mod pallet_message_queue;
pub mod pallet_multisig;
pub mod pallet_proxy;

pub mod pallet_sudo;
pub mod pallet_timestamp;
pub mod pallet_tx_pause;
pub mod pallet_utility;
pub mod pallet_xcm;
pub mod pallet_xcm_executor_utils;
pub mod xcm;
