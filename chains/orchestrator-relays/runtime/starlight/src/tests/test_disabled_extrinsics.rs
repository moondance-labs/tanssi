#![cfg(test)]
use {
    crate::{tests::common::*, RuntimeCall},
    frame_support::assert_noop,
    snowbridge_core::BasicOperatingMode::Halted,
    sp_core::H160,
    sp_runtime::traits::Dispatchable,
};

#[test]
fn test_disabled_some_extrinsics_for_balances() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
                dest: AccountId::from(BOB).into(),
                value: 12345,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::Balances(pallet_balances::Call::force_transfer {
                source: AccountId::from(ALICE).into(),
                dest: AccountId::from(BOB).into(),
                value: 12345,
            })
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}

#[test]
fn test_disabled_some_extrinsics_for_bridges() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(2);

        assert_noop!(
            RuntimeCall::EthereumSystem(snowbridge_pallet_system::Call::create_agent {}).dispatch(
                <Runtime as frame_system::Config>::RuntimeOrigin::signed(AccountId::from(ALICE))
            ),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumOutboundQueue(
                snowbridge_pallet_outbound_queue::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumInboundQueue(
                snowbridge_pallet_inbound_queue::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumTokenTransfers(
                pallet_ethereum_token_transfers::Call::transfer_native_token {
                    amount: 12345,
                    recipient: H160::random(),
                }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );

        assert_noop!(
            RuntimeCall::EthereumBeaconClient(
                snowbridge_pallet_ethereum_client::Call::set_operating_mode { mode: Halted }
            )
            .dispatch(<Runtime as frame_system::Config>::RuntimeOrigin::signed(
                AccountId::from(ALICE)
            )),
            frame_system::Error::<Runtime>::CallFiltered
        );
    });
}
