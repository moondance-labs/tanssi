import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { filterAndApply, type KeyringPair } from "@moonwall/util";
import { extractWeight } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { extractFeeAuthor, fetchIssuance, filterRewardFromOrchestrator, getTreasuryAddress } from "utils";
import type { EventRecord } from "@polkadot/types/interfaces";

describeSuite({
    id: "COMMO1001",
    title: "Fee  burning test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        // Difference between the refTime estimated using paymentInfo and the actual refTime reported inside a block
        // https://github.com/paritytech/substrate/blob/5e49f6e44820affccaf517fd22af564f4b495d40/frame/support/src/weights/extrinsic_weights.rs#L56
        let baseWeight: bigint;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
            baseWeight = extractWeight(polkadotJs.consts.system.blockWeights.perClass.normal.baseExtrinsic).toBigInt();
        });

        it({
            id: "E01",
            title: "0% of Fees are burned, they are sent to treasury",
            test: async () => {
                const totalSupplyBefore = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);

                const feeMultiplier = (await polkadotJs.query.transactionPayment.nextFeeMultiplier()).toBigInt();
                const feeInfo = await tx.paymentInfo(alice.address);
                const unadjustedWeightFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime: feeInfo.weight.refTime.toBigInt(),
                        proofSize: feeInfo.weight.proofSize.toBigInt(),
                    })
                ).toBigInt();

                const baseWeightToFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime: baseWeight,
                        proofSize: feeInfo.weight.proofSize.toBigInt(),
                    })
                ).toBigInt();

                const lengthToFee = (
                    await polkadotJs.call.transactionPaymentApi.queryLengthToFee(signedTx.encodedLength)
                ).toBigInt();
                const multiplierAdjustedWeightFee = (feeMultiplier * unadjustedWeightFee) / 1_000_000_000_000_000_000n;

                const expectedFee = baseWeightToFee + multiplierAdjustedWeightFee + lengthToFee;

                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const issuance = fetchIssuance(events).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);

                expect(fee).to.equal(expectedFee);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore + reward - fee - 200_000n).to.equal(balanceAfter);

                const totalSupplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();

                expect(totalSupplyAfter - totalSupplyBefore).to.equal(issuance);
            },
        });

        it({
            id: "E02",
            title: "no funds are burned at the end of spend period",
            test: async () => {
                const polkadotJs = context.polkadotJs();
                const treasuryAccount = getTreasuryAddress(polkadotJs);
                const treasuryBalanceBefore = (
                    await polkadotJs.query.system.account(treasuryAccount)
                ).data.free.toBigInt();

                const spendPeriod = Number(polkadotJs.consts.treasury.spendPeriod);
                const start = performance.now();
                for (let i = 0; i < spendPeriod - 1; i++) {
                    await context.createBlock();
                }
                const end = performance.now();
                console.log(`Took ${end - start} ms to create ${spendPeriod} blocks`);

                const treasuryBalanceAfter = (
                    await polkadotJs.query.system.account(treasuryAccount)
                ).data.free.toBigInt();
                const events = await polkadotJs.query.system.events();

                const spendingEvents = filterAndApply(events, "treasury", ["Spending"], ({ event }: EventRecord) =>
                    event.toHuman()
                );

                const burntEvents = filterAndApply(events, "treasury", ["Burnt"], ({ event }: EventRecord) =>
                    event.toHuman()
                );

                expect(spendingEvents.length, "Expect the length of the treasury.Spending equals 1").toEqual(1);
                expect(burntEvents.length, "Expect the length of the treasury.Burnt is 0").toEqual(0);

                expect(treasuryBalanceAfter).to.equal(treasuryBalanceBefore);
            },
        });
    },
});
