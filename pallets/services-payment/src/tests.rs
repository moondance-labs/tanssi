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

//! # Author Noting Pallet
//!
//! This pallet notes the author of the different containerChains that have registered:
//!
//! The set of container chains is retrieved thanks to the GetContainerChains trait
//! For each containerChain, we inspect the Header stored in the relayChain as
//! a generic header. This is the first requirement for containerChains.
//!
//! The second requirement is that an Aura digest with the slot number for the containerChains
//! needs to exist
//!  
//! Using those two requirements we can select who the author was based on the collators assigned
//! to that containerChain, by simply assigning the slot position.

use {
    crate::{mock::*, pallet as pallet_services_payment, BlockProductionCredits},
    cumulus_primitives_core::ParaId,
    frame_support::{assert_err, assert_ok, traits::fungible::Inspect},
    sp_runtime::DispatchError,
    tp_traits::AuthorNotingHook,
};

const ALICE: u64 = 1;

#[test]
fn purchase_credits_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            System::set_block_number(1);

            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                100u128,
            ),);

            assert_eq!(
                events(),
                vec![pallet_services_payment::Event::CreditsPurchased {
                    para_id: 1.into(),
                    payer: ALICE,
                    credit: 100u128
                }]
            );
            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                100u128
            );
        });
}
#[test]
fn purchase_credits_fails_with_insufficient_balance() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            // cannot purchase if death
            assert_err!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), 1000u128),
                sp_runtime::TokenError::NotExpendable,
            );
        });
}

#[test]
fn burn_credit_fails_with_no_credits() {
    ExtBuilder::default().build().execute_with(|| {
        assert_err!(
            PaymentServices::burn_free_credit_for_para(&1u32.into()),
            pallet_services_payment::Error::<Test>::InsufficientCredits,
        );
    });
}

#[test]
fn burn_credit_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1.into();
            assert_ok!(PaymentServices::set_credits(
                RuntimeOrigin::root(),
                para_id,
                1u64,
            ),);

            // should succeed and burn one
            assert_eq!(<BlockProductionCredits<Test>>::get(para_id), Some(1u64));
            assert_ok!(PaymentServices::burn_free_credit_for_para(&para_id));
            assert_eq!(<BlockProductionCredits<Test>>::get(para_id), Some(0u64));

            // now should fail
            assert_err!(
                PaymentServices::burn_free_credit_for_para(&para_id),
                pallet_services_payment::Error::<Test>::InsufficientCredits,
            );
        });
}

#[test]
fn burn_credit_fails_for_wrong_para() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1.into();
            assert_ok!(PaymentServices::set_credits(
                RuntimeOrigin::root(),
                para_id,
                1u64,
            ),);

            // fails for wrong para
            let wrong_para_id = 2.into();
            assert_err!(
                PaymentServices::burn_free_credit_for_para(&wrong_para_id),
                pallet_services_payment::Error::<Test>::InsufficientCredits,
            );
        });
}

#[test]
fn set_credits_bad_origin() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_err!(
                PaymentServices::set_credits(RuntimeOrigin::signed(ALICE), 1.into(), 1u64,),
                DispatchError::BadOrigin
            )
        });
}

#[test]
fn set_credits_above_max_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(PaymentServices::set_credits(
                RuntimeOrigin::root(),
                1.into(),
                MaxCreditsStored::get() * 2,
            ));

            assert_eq!(
                <BlockProductionCredits<Test>>::get(ParaId::from(1)),
                Some(MaxCreditsStored::get() * 2)
            );
        });
}

#[test]
fn set_credits_to_zero_kills_storage() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(PaymentServices::set_credits(
                RuntimeOrigin::root(),
                1.into(),
                0u64,
            ));

            assert_eq!(<BlockProductionCredits<Test>>::get(ParaId::from(1)), None,);
        });
}

#[test]
fn credits_should_be_substracted_from_tank_if_no_free_credits() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                1000u128,
            ));

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                1000u128
            );

            PaymentServices::on_container_author_noted(&1, 1, 1.into());

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                900u128
            );
        });
}

#[test]
fn credits_should_not_be_substracted_from_tank_if_it_involves_death() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                100u128,
            ));

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                100u128
            );

            PaymentServices::on_container_author_noted(&1, 1, 1.into());

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                100u128
            );
        });
}

#[test]
fn not_having_enough_tokens_in_tank_should_not_error() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                1u128,
            ));

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                1u128
            );

            PaymentServices::on_container_author_noted(&1, 1, 1.into());

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(1.into())),
                1u128
            );
        });
}
