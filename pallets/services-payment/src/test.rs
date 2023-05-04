use {
    crate::{pallet as payment_services_pallet, mock::*},
    frame_support::{assert_err, assert_ok},
};

const ALICE: u64 = 1;

#[test]
fn purchase_credits_fails_when_over_max() {
    ExtBuilder::default()
        .with_balances([(ALICE, 1_000)].into())
        .build()
        .execute_with(|| {
            assert_ok!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), MaxCreditsStored::get()),
            );
            
        });
}

#[test]
fn purchase_credits_fails_with_insufficient_balance() {
    ExtBuilder::default()
        .build()
        .execute_with(|| {
            assert_err!(
                PaymentServices::purchase_credits(RuntimeOrigin::signed(ALICE), 1.into(), 1),
                payment_services_pallet::Error::<Test>::InsufficientFunds,
            );
        });

}
