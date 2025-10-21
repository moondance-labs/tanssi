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
    bp_relayers::{PaymentProcedure, StakeAndSlash},
    core::{fmt::Debug, marker::PhantomData},
    frame_support::traits::{fungible::Mutate, tokens::Preservation, NamedReservableCurrency},
    sp_runtime::{
        codec::Codec,
        traits::{Get, Zero},
        DispatchError, DispatchResult,
    },
};

pub struct DoNothingStakeAndSlashNamed<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>(
    PhantomData<(AccountId, BlockNumber, Currency, ReserveId, Stake, Lease)>,
);

impl<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>
    StakeAndSlash<AccountId, BlockNumber, Currency::Balance>
    for DoNothingStakeAndSlashNamed<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>
where
    AccountId: Codec + Debug,
    Currency: NamedReservableCurrency<AccountId>,
    ReserveId: Get<Currency::ReserveIdentifier>,
    Stake: Get<Currency::Balance>,
    Lease: Get<BlockNumber>,
{
    type RequiredStake = Stake;
    type RequiredRegistrationLease = Lease;

    fn reserve(_relayer: &AccountId, _amount: Currency::Balance) -> DispatchResult {
        // Currency::reserve_named(&ReserveId::get(), relayer, amount)
        Ok(())
    }

    fn unreserve(_relayer: &AccountId, _amount: Currency::Balance) -> Currency::Balance {
        Zero::zero()
        // Currency::unreserve_named(&ReserveId::get(), relayer, amount)
    }

    fn repatriate_reserved(
        _relayer: &AccountId,
        _beneficiary: &AccountId,
        _amount: Currency::Balance,
    ) -> Result<Currency::Balance, DispatchError> {
        Ok(Zero::zero())
        // Currency::repatriate_reserved_named(
        //     &ReserveId::get(),
        //     relayer,
        //     &beneficiary,
        //     amount,
        //     BalanceStatus::Free,
        // )
    }
}

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
