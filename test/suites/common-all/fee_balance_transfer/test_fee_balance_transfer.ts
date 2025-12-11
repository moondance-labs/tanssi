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
        let isPara: boolean;

        // Difference between the refTime estimated using paymentInfo and the actual refTime reported inside a block
        // https://github.com/paritytech/substrate/blob/5e49f6e44820affccaf517fd22af564f4b495d40/frame/support/src/weights/extrinsic_weights.rs#L56
        let baseWeight: bigint;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
            baseWeight = extractWeight(polkadotJs.consts.system.blockWeights.perClass.normal.baseExtrinsic).toBigInt();
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isPara =
                runtimeName === "frontier-template" ||
                runtimeName === "container-chain-template" ||
                runtimeName.includes("box");
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
                const estimatedPaymentInfo = await tx.paymentInfo(alice.address);
                const signedTx = await tx.signAsync(alice);
                const feeMultiplier = (await polkadotJs.query.transactionPayment.nextFeeMultiplier()).toBigInt();
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, alice.address).amount.toBigInt();
                const reward = filterRewardFromOrchestrator(events, alice.address);
                // Get actual weight
                const info2 = extractInfoForFee(events);

                // The estimated weight does not match the actual weight reported in the block, because it is missing the
                // "base weight"
                const estimatedPlusBaseWeight = {
                    refTime: estimatedPaymentInfo.weight.refTime.toBigInt() + baseWeight,
                    proofSize: estimatedPaymentInfo.weight.proofSize.toBigInt(), // TODO: fix me
                };

                if (isPara) {
                    const maxBlockProofSize = polkadotJs.consts.system.blockWeights.maxBlock.proofSize.toBigInt();

                    expect(estimatedPlusBaseWeight.refTime).to.equal(info2.weight.refTime.toBigInt());

                    // Due to Storage weight reclaim, the on chain proof size is not the same as the estimated proof size
                    // using the paymentInfo runtime-api.
                    // The estimated should be greater than the on chain one.
                    expect(estimatedPlusBaseWeight.proofSize).toBeGreaterThan(info2.weight.proofSize.toBigInt());

                    // We check that the actual proof size is inside a ~5000 bytes range
                    expect(info2.weight.proofSize.toBigInt()).toBeGreaterThan(0n);
                    expect(info2.weight.proofSize.toBigInt()).toBeLessThanOrEqual(maxBlockProofSize / 1000n);
                } else {
                    expect(estimatedPlusBaseWeight).to.deep.equal({
                        refTime: info2.weight.refTime.toBigInt(),
                        proofSize: info2.weight.proofSize.toBigInt(),
                    });
                }

                const baseWeightToFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime: baseWeight,
                        // Base weight represents the fixed computational cost, independent of proof size.
                        // Setting proofSize to 0 ensures we only calculate the ref_time component of the base fee.
                        proofSize: 0n,
                    })
                ).toBigInt();

                // info contains just the extrinsic weight
                const onlyExtrinsicWeightFee = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee(estimatedPaymentInfo.weight)
                ).toBigInt();

                // These values are: 1000000 for base fee plus fee coming from the weight of the extrinsic
                // We allow variance of 10%
                const expectedBaseFee = context.isEthereumChain ? 1000000000000n : isRelay ? 3333333n : 1000000n;

                const expectedBasePlusWeightFee = expectedBaseFee + onlyExtrinsicWeightFee;

                /*
			stable2412:
			fee:  6055350n
			basePlusWeightFee:  6055206n
			expectedBaseFee:  1000000n
			expectedBasePlusWeightFee:  2600000n

		        master:
		        fee:  2724942n
			basePlusWeightFee:  2724798n
			expectedBaseFee:  1000000n
			expectedBasePlusWeightFee:  2600000n
		*/

                console.log("fee: ", fee);
                console.log("basePlusWeightFee: ", baseWeightToFee);
                console.log("expectedBaseFee: ", expectedBaseFee);
                console.log("expectedBasePlusWeightFee: ", expectedBasePlusWeightFee);

                const lengthToFee = (
                    await polkadotJs.call.transactionPaymentApi.queryLengthToFee(signedTx.encodedLength)
                ).toBigInt();

                const multiplierAdjustedWeightFee =
                    (feeMultiplier * onlyExtrinsicWeightFee) / 1_000_000_000_000_000_000n;

                const expectedFee = baseWeightToFee + multiplierAdjustedWeightFee + lengthToFee;

                expect(fee).to.equal(expectedFee);

                const tip = 0n;
                expect(fee).to.equal(estimatedPaymentInfo.partialFee.toBigInt() + tip);

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
                        proofSize: 0,
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
                        proofSize: 0,
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

                const refTimeFeesOnly = (
                    await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                        refTime,
                        proofSize: 0,
                    })
                ).toBigInt();

                const feesMap = new Map<bigint, number>();

                for (const proofSize of [0n, 10n, 100n, 10000n, 100000n]) {
                    const fullFee = (
                        await polkadotJs.call.transactionPaymentApi.queryWeightToFee({
                            refTime,
                            proofSize,
                        })
                    ).toBigInt();

                    feesMap.set(fullFee, (feesMap.get(fullFee) || 0) + 1);
                }

                if (isPara) {
                    // We expect that with 5 measurements, we should have at least 2 identical fees, based
                    // on the formula max(refTimeFee, proofSizeFee)
                    expect(feesMap.get(refTimeFeesOnly)).greaterThan(2);
                    console.log("feesMap", feesMap);
                } else {
                    // for *light chains, the proof size does not affect the fee
                    expect(feesMap.size).to.equal(1);
                }
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
    const event = filterAndApply(events, "system", ["ExtrinsicFailed", "ExtrinsicSuccess"], getDispatchInfo).filter(
        (x) => {
            return x.class.toString() === "Normal" && x.paysFee.toString() === "Yes";
        }
    );
    expect(event.length === 1);
    return event[0];
}
