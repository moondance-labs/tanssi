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
        mock::*, pallet as pallet_services_payment, BlockProductionCredits,
        CollatorAssignmentCredits, ProvideBlockProductionCost, ProvideCollatorAssignmentCost,
        RefundAddress,
    },
    cumulus_primitives_core::ParaId,
    frame_support::{assert_err, assert_noop, assert_ok, traits::fungible::Inspect},
    sp_runtime::DispatchError,
    tp_traits::{AuthorNotingHook, CollatorAssignmentHook},
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
            PaymentServices::burn_block_production_free_credit_for_para(&1u32.into()),
            pallet_services_payment::Error::<Test>::InsufficientCredits,
        );
        assert_err!(
            PaymentServices::burn_collator_assignment_free_credit_for_para(&1u32.into()),
            pallet_services_payment::Error::<Test>::InsufficientCredits,
        );
    });
}

#[test]
fn burn_block_production_credit_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1.into();
            assert_ok!(PaymentServices::set_block_production_credits(
                RuntimeOrigin::root(),
                para_id,
                1u64,
            ),);

            // should succeed and burn one
            assert_eq!(<BlockProductionCredits<Test>>::get(para_id), Some(1u64));
            assert_ok!(PaymentServices::burn_block_production_free_credit_for_para(
                &para_id
            ));
            assert_eq!(<BlockProductionCredits<Test>>::get(para_id), Some(0u64));

            // now should fail
            assert_err!(
                PaymentServices::burn_block_production_free_credit_for_para(&para_id),
                pallet_services_payment::Error::<Test>::InsufficientCredits,
            );
        });
}

#[test]
fn burn_collator_assignment_credit_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1.into();
            assert_ok!(PaymentServices::set_collator_assignment_credits(
                RuntimeOrigin::root(),
                para_id,
                1u32,
            ),);

            // should succeed and burn one
            assert_eq!(<CollatorAssignmentCredits<Test>>::get(para_id), Some(1u32));
            assert_ok!(PaymentServices::burn_collator_assignment_free_credit_for_para(&para_id));
            assert_eq!(<CollatorAssignmentCredits<Test>>::get(para_id), Some(0u32));

            // now should fail
            assert_err!(
                PaymentServices::burn_collator_assignment_free_credit_for_para(&para_id),
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
            assert_ok!(PaymentServices::set_block_production_credits(
                RuntimeOrigin::root(),
                para_id,
                1u64,
            ),);
            assert_ok!(PaymentServices::set_collator_assignment_credits(
                RuntimeOrigin::root(),
                para_id,
                1u32,
            ),);

            // fails for wrong para
            let wrong_para_id = 2.into();
            assert_err!(
                PaymentServices::burn_block_production_free_credit_for_para(&wrong_para_id),
                pallet_services_payment::Error::<Test>::InsufficientCredits,
            );
            assert_err!(
                PaymentServices::burn_collator_assignment_free_credit_for_para(&wrong_para_id),
                pallet_services_payment::Error::<Test>::InsufficientCredits,
            );
        });
}

#[test]
fn set_block_production_credits_bad_origin() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_err!(
                PaymentServices::set_block_production_credits(
                    RuntimeOrigin::signed(ALICE),
                    1.into(),
                    1u64,
                ),
                DispatchError::BadOrigin
            )
        });
}

#[test]
fn set_block_production_credits_above_max_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(PaymentServices::set_block_production_credits(
                RuntimeOrigin::root(),
                1.into(),
                FreeBlockProductionCredits::get() * 2,
            ));

            assert_eq!(
                <BlockProductionCredits<Test>>::get(ParaId::from(1)),
                Some(FreeBlockProductionCredits::get() * 2)
            );
        });
}

#[test]
fn set_block_production_credits_to_zero_kills_storage() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(PaymentServices::set_block_production_credits(
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

            assert_noop!(
                PaymentServices::on_collators_assigned(1.into(), None, false),
                pallet_balances::Error::<Test>::InsufficientBalance
            );

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

#[test]
fn on_deregister_burns_if_no_deposit_address() {
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

            let issuance_before = Balances::total_issuance();
            crate::Pallet::<Test>::para_deregistered(1.into());
            let issuance_after = Balances::total_issuance();
            assert_eq!(issuance_after, issuance_before - 1000u128);

            // Refund address gets cleared
            assert!(<RefundAddress<Test>>::get(ParaId::from(1)).is_none());
        });
}

#[test]
fn on_deregister_cleans_refund_address_even_when_purchases_have_not_being_made() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            let refund_address = 10u64;

            assert_ok!(PaymentServices::set_refund_address(
                RuntimeOrigin::root(),
                1.into(),
                Some(refund_address),
            ));

            crate::Pallet::<Test>::para_deregistered(1.into());

            // Refund address gets cleared
            assert!(<RefundAddress<Test>>::get(ParaId::from(1)).is_none());
        });
}

#[test]
fn on_deregister_deposits_if_refund_address() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            let refund_address = 10u64;
            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                1000u128,
            ));

            // this should set refund address
            assert_ok!(PaymentServices::set_refund_address(
                RuntimeOrigin::root(),
                1.into(),
                Some(refund_address),
            ));

            let issuance_before = Balances::total_issuance();
            crate::Pallet::<Test>::para_deregistered(1.into());
            let issuance_after = Balances::total_issuance();
            assert_eq!(issuance_after, issuance_before);

            let balance_refund_address = Balances::balance(&refund_address);
            assert_eq!(balance_refund_address, 1000u128);

            assert!(<RefundAddress<Test>>::get(ParaId::from(1)).is_none());
        });
}

#[test]
fn set_refund_address_with_none_removes_storage() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000)].into())
        .build()
        .execute_with(|| {
            let refund_address = 10u64;
            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                1.into(),
                1000u128,
            ));

            // this should set refund address
            assert_ok!(PaymentServices::set_refund_address(
                RuntimeOrigin::root(),
                1.into(),
                Some(refund_address),
            ));

            assert!(<RefundAddress<Test>>::get(ParaId::from(1)).is_some());

            assert_ok!(PaymentServices::set_refund_address(
                RuntimeOrigin::root(),
                1.into(),
                None,
            ));

            assert!(<RefundAddress<Test>>::get(ParaId::from(1)).is_none());
        });
}

#[test]
fn tip_should_be_charged_on_collators_assignment() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1;
            let tip = 10u128;
            let balance = 5000u128;

            // this should give 10 block credit
            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                para_id.into(),
                balance,
            ));

            assert_ok!(PaymentServices::set_max_tip(
                RuntimeOrigin::root(),
                para_id.into(),
                Some(tip),
            ));

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(para_id.into())),
                balance,
            );

            assert_ok!(PaymentServices::on_collators_assigned(
                para_id.into(),
                Some(&tip),
                false
            ));

            PaymentServices::on_container_author_noted(&1, 1, para_id.into());

            let (assignment_cost, _weight) =
                <Test as crate::Config>::ProvideCollatorAssignmentCost::collator_assignment_cost(
                    &para_id.into(),
                );
            let (block_cost, _weight) =
                <Test as crate::Config>::ProvideBlockProductionCost::block_cost(&para_id.into());

            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(para_id.into())),
                balance - assignment_cost - block_cost - tip,
            );
        });
}

#[test]
fn insufficient_balance_for_tip_reimburses_fee_imbalance() {
    ExtBuilder::default()
        .with_balances([(ALICE, 2_000_000)].into())
        .build()
        .execute_with(|| {
            let para_id = 1;
            let tip = 10u128;
            // Just enough for one assignment but not for tip;
            let balance = 205u128;

            assert_ok!(PaymentServices::purchase_credits(
                RuntimeOrigin::signed(ALICE),
                para_id.into(),
                balance,
            ));

            assert_ok!(PaymentServices::set_max_tip(
                RuntimeOrigin::root(),
                para_id.into(),
                Some(tip),
            ));

            // it should fail when trying to withdraw the tip
            assert!(
                PaymentServices::on_collators_assigned(para_id.into(), Some(&tip), false).is_err()
            );

            // Tank balance shouldn't have changed
            assert_eq!(
                Balances::balance(&crate::Pallet::<Test>::parachain_tank(para_id.into())),
                balance,
            );
        });
}
