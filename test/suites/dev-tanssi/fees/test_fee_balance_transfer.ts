import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair, extractFee, extractInfo, filterAndApply } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DT0401",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Fee of balances.transfer can be estimated using paymentInfo",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
                const info = await tx.paymentInfo(alice.address);
                const signedTx = await tx.signAsync(alice);
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                console.log("events: ", events.toJSON());

                const fee = extractFee(events).amount.toBigInt();
                console.log("fee2: ", fee);
                function getDispatchInfo({ event: { data, method } }) {
                    return method === "ExtrinsicSuccess" ? (data[0] as any) : (data[1] as any);
                }
                const info2 = filterAndApply(events, "system", ["ExtrinsicFailed", "ExtrinsicSuccess"], getDispatchInfo)
                    .filter((x) => {
                        return x.class.toString() === "Normal" && x.paysFee.toString() === "Yes";
                    })
                    .map((x) => x.toJSON())[0];
                //const info2 = extractInfo(events);
                console.log("INFO2: ", info2);

                const ww = info2.weight.refTime;
                // TODO: proofSize is free?

                const expectedFee = 1000000n + BigInt(signedTx.encodedLength) + 1630678n;
                expect(fee).to.equal(expectedFee);

                const tip = 0n;
                expect(fee).to.equal(info.partialFee.toBigInt() + tip);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
            },
        });

        it({
            id: "E02",
            title: "Fee of balances.transfer can be estimated using transactionPaymentApi.queryFeeDetails",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx,
                    signedTx.encodedLength
                );

                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFee(events).amount.toBigInt();
                const expectedFee = 1000000n + BigInt(signedTx.encodedLength) + 1630678n;
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
                expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
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
                const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);
                const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(
                    tx,
                    signedTx.encodedLength
                );
                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFee(events).amount.toBigInt();
                const expectedFee = 1000000n + BigInt(signedTx.encodedLength) + 1630678n;
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
                expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
            },
        });

        it({
            id: "E04",
            title: "Fees are burned",
            test: async function () {
                const totalSupplyBefore = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
                const signedTx = await tx.signAsync(alice);

                await context.createBlock([signedTx]);

                const events = await polkadotJs.query.system.events();
                const fee = extractFee(events).amount.toBigInt();
                const expectedFee = 1000000n + BigInt(signedTx.encodedLength) + 1630678n;
                expect(fee).to.equal(expectedFee);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Balance must be old balance minus fee minus transfered value
                expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);

                const totalSupplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();

                expect(totalSupplyBefore - totalSupplyAfter).to.equal(fee);
            },
        });
    },
});
