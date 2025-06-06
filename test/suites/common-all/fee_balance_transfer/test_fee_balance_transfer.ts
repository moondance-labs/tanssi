import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, extractWeight, filterAndApply } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES, checkCallIsFiltered } from "helpers";
import { extractFeeAuthor, filterRewardFromOrchestrator } from "utils";

describeSuite({
    id: "C0101",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let isRelay: boolean;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipBalances: boolean;

        // Difference between the refTime estimated using paymentInfo and the actual refTime reported inside a block
        // https://github.com/paritytech/substrate/blob/5e49f6e44820affccaf517fd22af564f4b495d40/frame/support/src/weights/extrinsic_weights.rs#L56
        let baseWeight: bigint;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
            baseWeight = extractWeight(polkadotJs.consts.system.blockWeights.perClass.normal.baseExtrinsic).toBigInt();
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isRelay = runtimeName.includes("light");
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipBalances = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Fee of balances.transfer can be estimated using paymentInfo",
            test: async () => {
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);

                if (shouldSkipBalances) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
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
                    proofSize: info.weight.proofSize.toBigInt(), // TODO: fix me
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

                // info contains just the extrinsic weight
                const onlyExtrinsicWeightFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee(info.weight)
                ).toBigInt();

                // These values are: 1000000 for base fee plus fee coming from the weight of the extrinsic
                // We allow variance of 10%
                const expectedBaseFee = context.isEthereumChain ? 1000000000000n : isRelay ? 3333333n : 1000000n;

                const expectedbasePlusWeightFee = expectedBaseFee + onlyExtrinsicWeightFee;

                /*
			stable2412:
			fee:  6055350n
			basePlusWeightFee:  6055206n
			expectedBaseFee:  1000000n
			expectedbasePlusWeightFee:  2600000n

		        master:
		        fee:  2724942n
			basePlusWeightFee:  2724798n
			expectedBaseFee:  1000000n
			expectedbasePlusWeightFee:  2600000n
		*/

                console.log("fee: ", fee);
                console.log("basePlusWeightFee: ", basePlusWeightFee);
                console.log("expectedBaseFee: ", expectedBaseFee);
                console.log("expectedbasePlusWeightFee: ", expectedbasePlusWeightFee);

                expect(
                    basePlusWeightFee >= (expectedbasePlusWeightFee * 90n) / 100n &&
                        basePlusWeightFee <= (expectedbasePlusWeightFee * 110n) / 100n
                ).to.be.true;

                const expectedFee = basePlusWeightFee + BigInt(signedTx.encodedLength);

                // Caution: this +1 comes from the fact that even if qeryWeightToFee applies unadjusted
                // but when we pay fees (or compare with queryFeeDetails), we do it adjusted (with multiplier). In our case we are using
                // a constant multiplier, but because of rounding issues with the weight, we migth obtain
                // a +-1 difference
                expect(fee >= expectedFee - 1n && basePlusWeightFee <= expectedFee + 1n).to.be.true;

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
            test: async () => {
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);

                if (shouldSkipBalances) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx.toU8a(),
                    signedTx.encodedLength
                );

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
                const reward = filterRewardFromOrchestrator(events, alice.address);

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
            title: "Fee of balances.transfer does increase after 100 full blocks due to slow adjusting multiplier",
            test: async () => {
                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);

                if (shouldSkipBalances) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }
                const fillAmount = 600_000_000; // equal to 60% Perbill

                const previousfeeMultiplier = (
                    await polkadotJs.query.transactionPayment.nextFeeMultiplier()
                ).toBigInt();
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
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx.toU8a(),
                    signedTx.encodedLength
                );
                const currentfeeMultiplier = (await polkadotJs.query.transactionPayment.nextFeeMultiplier()).toBigInt();
                expect(currentfeeMultiplier).toBeGreaterThan(previousfeeMultiplier);
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
                const multiplierAdjustedWeightFee =
                    (currentfeeMultiplier * unadjustedWeightFee) / 1_000_000_000_000_000_000n;

                const expectedFee = baseWeightToFee + multiplierAdjustedWeightFee + lengthToFee;
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);
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
            title: "Proof size does not affect fee",
            test: async () => {
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
            id: "E05",
            title: "Base refTime pays base fee",
            test: async () => {
                const fee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime: baseWeight,
                        proofSize: 0,
                    })
                ).toBigInt();

                const expectedFee = context.isEthereumChain ? 1000000000000n : isRelay ? 3333333n : 1000000n;
                expect(fee).to.equal(expectedFee);
            },
        });
    },
});

function getDispatchInfo({ event: { data, method } }) {
    return method === "ExtrinsicSuccess" ? (data[0] as any) : (data[1] as any);
}

function extractInfoForFee(events): any {
    const event = filterAndApply(events, "system", ["ExtrinsicFailed", "ExtrinsicSuccess"], getDispatchInfo).filter((x) => {
        return x.class.toString() === "Normal" && x.paysFee.toString() === "Yes";
    });
    expect(event.length === 1)
    return event[0];
}
