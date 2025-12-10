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

#![cfg(test)]

use {
    crate::{
        tests::common::*, ConvictionVoting, OpenTechCommitteeCollective, Referenda, RuntimeCall,
    },
    alloc::vec,
    frame_support::{
        assert_ok,
        traits::{schedule::DispatchTime, Bounded},
        weights::Weight,
    },
    pallet_referenda::TracksInfo,
    parity_scale_codec::Encode,
    sp_core::H256,
    sp_runtime::traits::{BlakeTwo256, Hash},
};

#[test]
fn test_root_track_executes_with_root_origin() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let random_code_hash = H256::random();
            let proposal_call =
                RuntimeCall::System(frame_system::Call::<Runtime>::authorize_upgrade {
                    code_hash: random_code_hash,
                })
                .encode();
            assert_ok!(Referenda::submit(
                origin_of(ALICE.into()),
                Box::new(frame_support::dispatch::RawOrigin::Root.into()),
                Bounded::Inline(proposal_call.clone().try_into().unwrap()),
                DispatchTime::At(10),
            ));
            let index = pallet_referenda::ReferendumCount::<Runtime>::get() - 1;
            assert_ok!(Referenda::place_decision_deposit(
                origin_of(ALICE.into()),
                index
            ));
            let vote = pallet_conviction_voting::AccountVote::<Balance>::Standard {
                vote: pallet_conviction_voting::Vote {
                    aye: true,
                    conviction: pallet_conviction_voting::Conviction::None,
                },
                balance: 90_000 * UNIT,
            };
            assert_ok!(ConvictionVoting::vote(origin_of(ALICE.into()), index, vote));
            assert_ok!(ConvictionVoting::vote(origin_of(BOB.into()), index, vote));

            wait_for_democracy_to_pass(index);

            // Now we need to wait for the enactment period
            let current_block = System::block_number();
            // Root track is number 0
            let enactment_period = crate::governance::TracksInfo::info(0)
                .unwrap()
                .min_enactment_period;
            run_to_block(current_block + enactment_period);

            // We assert the call went through
            assert!(System::authorized_upgrade().is_some());
        });
}

#[test]
fn test_whitelist_track_executes_with_whitelist_origin_but_fails_if_not_whitelisted() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let random_code_hash = H256::random();
            let proposal_call =
                RuntimeCall::System(frame_system::Call::<Runtime>::authorize_upgrade {
                    code_hash: random_code_hash,
                });

            let proposal_call_hash = BlakeTwo256::hash(&proposal_call.encode());

            // We propose the whitelist call dispatch in the referenda but we will never whitelist it
            let whitelist_call = RuntimeCall::Whitelist(
                pallet_whitelist::Call::<Runtime>::dispatch_whitelisted_call {
                    call_hash: proposal_call_hash,
                    call_encoded_len: proposal_call.encode().len() as u32,
                    call_weight_witness: Weight::from_parts(1_000_000_000, 1_000_000),
                },
            )
            .encode();

            assert_ok!(Referenda::submit(
                origin_of(ALICE.into()),
                Box::new(crate::pallet_custom_origins::Origin::WhitelistedCaller.into()),
                Bounded::Inline(whitelist_call.clone().try_into().unwrap()),
                DispatchTime::At(10),
            ));
            let index = pallet_referenda::ReferendumCount::<Runtime>::get() - 1;
            assert_ok!(Referenda::place_decision_deposit(
                origin_of(ALICE.into()),
                index
            ));
            let vote = pallet_conviction_voting::AccountVote::<Balance>::Standard {
                vote: pallet_conviction_voting::Vote {
                    aye: true,
                    conviction: pallet_conviction_voting::Conviction::None,
                },
                balance: 90_000 * UNIT,
            };
            assert_ok!(ConvictionVoting::vote(origin_of(ALICE.into()), index, vote));
            assert_ok!(ConvictionVoting::vote(origin_of(BOB.into()), index, vote));

            wait_for_democracy_to_pass(index);

            // Now we need to wait for the enactment period
            let current_block = System::block_number();
            // whitelist caller track is number 1
            let enactment_period = crate::governance::TracksInfo::info(1)
                .unwrap()
                .min_enactment_period;
            run_to_block(current_block + enactment_period);

            // not whitelisted ergo it will fail
            assert!(System::authorized_upgrade().is_none());
        });
}

#[test]
fn test_whitelist_track_executes_with_whitelist_origin_works_if_whitelisted() {
    ExtBuilder::default()
        .with_balances(vec![
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            // Let's first set alice as the committee member
            assert_ok!(OpenTechCommitteeCollective::set_members(
                root_origin(),
                vec![AccountId::from(ALICE)],
                None,
                0
            ));

            let random_code_hash = H256::random();
            let proposal_call =
                RuntimeCall::System(frame_system::Call::<Runtime>::authorize_upgrade {
                    code_hash: random_code_hash,
                });

            let authorize_hash = note_preimage(AccountId::from(ALICE), &proposal_call.encode());

            let whitelist_call_dispatch = RuntimeCall::Whitelist(
                pallet_whitelist::Call::<Runtime>::dispatch_whitelisted_call {
                    call_hash: authorize_hash,
                    call_encoded_len: proposal_call.encode().len() as u32,
                    call_weight_witness: Weight::from_parts(1_000_000_000, 1_000_000),
                },
            )
            .encode();

            assert_ok!(Referenda::submit(
                origin_of(ALICE.into()),
                Box::new(crate::pallet_custom_origins::Origin::WhitelistedCaller.into()),
                Bounded::Inline(whitelist_call_dispatch.clone().try_into().unwrap()),
                DispatchTime::At(10),
            ));
            let index = pallet_referenda::ReferendumCount::<Runtime>::get() - 1;
            assert_ok!(Referenda::place_decision_deposit(
                origin_of(ALICE.into()),
                index
            ));
            let vote = pallet_conviction_voting::AccountVote::<Balance>::Standard {
                vote: pallet_conviction_voting::Vote {
                    aye: true,
                    conviction: pallet_conviction_voting::Conviction::None,
                },
                balance: 90_000 * UNIT,
            };
            assert_ok!(ConvictionVoting::vote(origin_of(ALICE.into()), index, vote));
            assert_ok!(ConvictionVoting::vote(origin_of(BOB.into()), index, vote));

            let whitelist_call =
                RuntimeCall::Whitelist(pallet_whitelist::Call::<Runtime>::whitelist_call {
                    call_hash: authorize_hash,
                });

            // let's waitlist!
            // Alice should be enough as threshold is one
            assert_ok!(OpenTechCommitteeCollective::propose(
                origin_of(ALICE.into()),
                //threshold
                1,
                Box::new(whitelist_call.clone()),
                whitelist_call.encode().len() as u32
            ));

            assert!(pallet_whitelist::WhitelistedCall::<Runtime>::get(authorize_hash).is_some());

            wait_for_democracy_to_pass(index);

            // Now we need to wait for the enactment period
            let current_block = System::block_number();
            // whitelist caller track is number 1
            let enactment_period = crate::governance::TracksInfo::info(1)
                .unwrap()
                .min_enactment_period;
            run_to_block(current_block + enactment_period);

            // whitelisted ergo it should succeed
            assert!(System::authorized_upgrade().is_some());
        });
}
