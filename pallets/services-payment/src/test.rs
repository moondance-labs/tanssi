use {
    crate::{pallet as payment_services_pallet, mock::*},
    frame_support::{assert_err, assert_ok},
};

const ALICE: u64 = 1;

#[test]
fn purchase_credits_works() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            // apparently events don't work in genesis block, so start on block 1
            System::set_block_number(1);

            assert_ok!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), MaxCreditsStored::get()),
            );

            assert_eq!(
                events(),
                vec![payment_services_pallet::Event::CreditsPurchased {
                    para_id: 1.into(),
                    payer: ALICE,
                    fee: 500,
                    credits_purchased: MaxCreditsStored::get(),
                    credits_owned: MaxCreditsStored::get(),
                }]
            );
        });
}

#[test]
fn purchase_credits_fails_when_over_max() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), MaxCreditsStored::get()),
            );

            assert_err!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), 1),
                payment_services_pallet::Error::<Test>::TooManyCredits,
            );
            
        });
}

#[test]
fn purchase_credits_fails_with_insufficient_balance() {
    ExtBuilder::default()
        .build()
        .execute_with(|| {
            // really what we're testing is that purchase_credits fails when OnChargeForBlockCredits does
            assert_err!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), 1),
                payment_services_pallet::Error::<Test>::InsufficientFunds,
            );
        });

}
