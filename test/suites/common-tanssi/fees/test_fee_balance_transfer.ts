import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair, filterAndApply } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { extractWeight } from "@moonwall/util";
import { extractFeeAuthor, fetchIssuance, filterRewardFromOrchestrator } from "util/block";

describeSuite({
    id: "CT0201",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let adjustedExpectedBasePlusWeightFee;

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
            title: "Fee of balances.transfer can be estimated using paymentInfo",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                // Estimate fee of balances.transfer using paymentInfo API, before sending transaction
                const info = await tx.paymentInfo(alice.address);
                const signedTx = await tx.signAsync(alice);
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);
                // Get actual weight
                const info2 = extractInfoForFee(events);

                // The estimated weight does not match the actual weight reported in the block, because it is missing the
                // "base weight"
                const estimatedPlusBaseWeight = {
                    refTime: info.weight.refTime.toBigInt() + baseWeight,
                    proofSize: info.weight.proofSize.toBigInt(),
                };
                expect(estimatedPlusBaseWeight).to.deep.equal({
                    refTime: info2.weight.refTime.toBigInt(),
                    proofSize: info2.weight.proofSize.toBigInt(),
                });

                // queryWeightToFee expects the "base weight" to be included in the input, so info2.weight provides
                // the correct estimation, but tx.paymentInfo().weight does not
                const basePlusWeightFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee(info2.weight)
                ).toBigInt();
                // These values are: 1000000 for base fee plus fee coming from the weight of the extrinsic
                expect(basePlusWeightFee).to.equal(1000000n + 1479873n);
                // Caution: this +1 comes from the fact that even if qeryWeightToFee applies unadjusted
                // but when we pay fees (or compare with queryFeeDetails), we do it adjusted (with multiplier). In our case we are using
                // a constant multiplier, but because of rounding issues with the weight, we migth obtain
                // a +-1 difference
                adjustedExpectedBasePlusWeightFee = basePlusWeightFee + 1n;

                const expectedFee = adjustedExpectedBasePlusWeightFee + BigInt(signedTx.encodedLength);
                expect(fee).to.equal(expectedFee);

                const tip = 0n;
                expect(fee).to.equal(info.partialFee.toBigInt() + tip);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore + reward - fee - 200_000n).to.equal(balanceAfter);
            },
        });

        it({
            id: "E02",
            title: "Fee of balances.transfer can be estimated using transactionPaymentApi.queryFeeDetails",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx,
                    signedTx.encodedLength
                );

                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);

                const expectedFee = adjustedExpectedBasePlusWeightFee + BigInt(signedTx.encodedLength);
                expect(fee).to.equal(expectedFee);

                const inclusionFee = feeDetails.inclusionFee.unwrapOrDefault();
                const tip = 0n;
                expect(fee).to.equal(
                    inclusionFee.lenFee.toBigInt() +
                        inclusionFee.baseFee.toBigInt() +
                        inclusionFee.adjustedWeightFee.toBigInt() +
                        tip
                );

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore + reward - fee - 200_000n).to.equal(balanceAfter);
            },
        });

        it({
            id: "E03",
            title: "Fee of balances.transfer does not increase after 100 full blocks",
            test: async function () {
                const fillAmount = 600_000_000; // equal to 60% Perbill

                for (let i = 0; i < 100; i++) {
                    const tx = polkadotJs.tx.rootTesting.fillBlock(fillAmount);
                    const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);

                    await context.createBlock([signedTx]);

                    // Because the session duration is only 5 blocks, 1 out of every 5 blocks
                    // cannot include any extrinsics. So we check that case, and create an
                    // additional block.
                    const block = await polkadotJs.rpc.chain.getBlock();
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());

                    if (!includedTxHashes.includes(signedTx.hash.toString())) {
                        await context.createBlock([]);
                    }
                }

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx,
                    signedTx.encodedLength
                );
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);
                const expectedFee = adjustedExpectedBasePlusWeightFee + BigInt(signedTx.encodedLength);
                expect(fee).to.equal(expectedFee);

                const inclusionFee = feeDetails.inclusionFee.unwrapOrDefault();
                const tip = 0n;
                expect(fee).to.equal(
                    inclusionFee.baseFee.toBigInt() +
                        inclusionFee.lenFee.toBigInt() +
                        inclusionFee.adjustedWeightFee.toBigInt() +
                        tip
                );

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore + reward - fee - 200_000n).to.equal(balanceAfter);
            },
        });

        it({
            id: "E04",
            title: "80% of Fees are burned",
            test: async function () {
                const totalSupplyBefore = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);

                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const issuance = fetchIssuance(events).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);
                const expectedFee = adjustedExpectedBasePlusWeightFee + BigInt(signedTx.encodedLength);
                expect(fee).to.equal(expectedFee);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore + reward - fee - 200_000n).to.equal(balanceAfter);

                const totalSupplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();

                expect(totalSupplyAfter - totalSupplyBefore).to.equal(issuance - (fee * 4n) / 5n);
            },
        });

        it({
            id: "E05",
            title: "Proof size does not affect fee",
            test: async function () {
                const refTime = 298945000n;
                const proofSize = 3593n;
                const fee1 = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime,
                        proofSize,
                    })
                ).toBigInt();

                const fee2 = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime,
                        proofSize: 0,
                    })
                ).toBigInt();

                expect(fee1).to.equal(fee2);
            },
        });

        it({
            id: "E06",
            title: "Base refTime pays base fee",
            test: async function () {
                const fee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime: baseWeight,
                        proofSize: 0,
                    })
                ).toBigInt();

                expect(fee).to.equal(1000000n);
            },
        });
    },
});

function getDispatchInfo({ event: { data, method } }) {
    return method === "ExtrinsicSuccess" ? (data[0] as any) : (data[1] as any);
}

function extractInfoForFee(events): any {
    return filterAndApply(events, "system", ["ExtrinsicFailed", "ExtrinsicSuccess"], getDispatchInfo).filter((x) => {
        return x.class.toString() === "Normal" && x.paysFee.toString() === "Yes";
    })[0];
}
