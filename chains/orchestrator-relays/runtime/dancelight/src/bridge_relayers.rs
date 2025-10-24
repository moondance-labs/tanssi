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
    crate::{
        bridge_to_ethereum_config::{BridgeReward, BridgeRewardBeneficiaries},
        AccountId, SnowbridgeFeesAccount,
    },
    bp_relayers::PaymentProcedure,
    frame_support::traits::{fungible::Mutate, tokens::Preservation},
};

pub struct BridgeRewardPayer;
impl PaymentProcedure<AccountId, BridgeReward, u128> for BridgeRewardPayer {
    type Error = sp_runtime::DispatchError;
    type Beneficiary = BridgeRewardBeneficiaries;

    fn pay_reward(
        _relayer: &AccountId,
        reward_kind: BridgeReward,
        reward: u128,
        beneficiary: BridgeRewardBeneficiaries,
    ) -> Result<(), Self::Error> {
        match reward_kind {
            BridgeReward::Snowbridge => match beneficiary {
                BridgeRewardBeneficiaries::LocalAccount(account) => {
                    <pallet_balances::Pallet<crate::Runtime> as Mutate<AccountId>>::transfer(
                        &SnowbridgeFeesAccount::get(),
                        &account,
                        reward,
                        Preservation::Expendable,
                    )
                    .map(drop)
                }
            },
        }
    }
}
