import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { extractWeight } from "@moonwall/util";
import { extractFeeAuthor, fetchIssuance, filterRewardFromOrchestrator } from "util/block";

describeSuite({
    id: "CT0201",
    title: "Fee  burning test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        // Difference between the refTime estimated using paymentInfo and the actual refTime reported inside a block
        // https://github.com/paritytech/substrate/blob/5e49f6e44820affccaf517fd22af564f4b495d40/frame/support/src/weights/extrinsic_weights.rs#L56
        let baseWeight;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
            baseWeight = extractWeight(polkadotJs.consts.system.blockWeights.perClass.normal.baseExtrinsic).toBigInt();
        });

        it({
            id: "E01",
            title: "80% of Fees are burned",
            test: async function () {
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

                expect(totalSupplyAfter - totalSupplyBefore).to.equal(issuance - (fee * 4n) / 5n);
            },
        });
    },
});
