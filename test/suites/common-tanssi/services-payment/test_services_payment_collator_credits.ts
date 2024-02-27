import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { generateKeyringPair, KeyringPair } from "@moonwall/util";
import { jumpSessions } from "util/block";
import { paraIdTank } from "util/payment";

describeSuite({
    id: "CT0601",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const startingCredits = 100n;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Genesis container chains have credits and collators and should have one less credit",
            test: async function () {
                await context.createBlock();
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                for (const paraId of parasRegistered) {
                    // Should have credits
                    const credits = await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId);

                    // Should have assigned collators
                    const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();

                    // Container chain 2001 does not have any collators, this will result in only 1 container chain
                    // producing blocks at a time. So if both container chains have 1000 credits, container 2000
                    // will produce blocks 0-999, and container 2001 will produce blocks 1000-1999.
                    if (paraId.toBigInt() === 2000n) {
                        expect(
                            credits.unwrap().toBigInt(),
                            `Container chain ${paraId} should have applied session credits`
                        ).toBe(startingCredits - 1n);
                        expect(
                            collators.toJSON().containerChains[paraId.toString()].length,
                            `Container chain ${paraId} has 0 collators`
                        ).toBeGreaterThan(0);
                    } else {
                        expect(
                            credits.unwrap().toBigInt(),
                            `Container chain ${paraId} should not have substracted credits`
                        ).toBe(startingCredits);
                        expect(
                            collators.toJSON().containerChains[paraId.toString()].length,
                            `Container chain ${paraId} has 0 collators`
                        ).toBe(0);
                    }
                }
            },
        });

        it({
            id: "E02",
            title: "Getting assignation should consume credits",
            test: async function () {
                // Moving to the next session should have reduced the credit by one to both parachains
                // even if one does not produce blocks

                const paraId = 2000n;
                await jumpSessions(context, 1);
                const credits = await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId);
                expect(
                    credits.unwrap().toBigInt(),
                    `Container chain ${paraId} does not have enough credits at genesis`
                ).toBe(startingCredits - 2n);
            },
        });

        it({
            id: "E03",
            title: "Collators are unassigned when a container chain does not have enough credits",
            test: async function () {
                // Create blocks until authorNoting.blockNum does not increase anymore.
                // Check that collatorAssignment does not have collators and num credits is less than 2 sessions.

                const paraId = 2000n;

                // Create blocks until the block number stops increasing
                let containerBlockNum3 = -1;
                let containerBlockNum4 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;

                while (containerBlockNum3 != containerBlockNum4) {
                    await context.createBlock();
                    containerBlockNum3 = containerBlockNum4;
                    containerBlockNum4 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                        .blockNumber;
                }

                // Now the container chain should have less than 2 sessions worth of credits
                const credits = (await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId)).toJSON();
                expect(
                    credits,
                    "Container chain 2000 has stopped producing blocks, so it should not have enough credits"
                ).toBeLessThan(2n);

                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId],
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined();
            },
        });

        it({
            id: "E04",
            title: "Root can remove credits",
            test: async function () {
                // Remove all the credits of container chain 2001, which should have assigned collators now
                // This checks that the node does not panic when we try to subtract credits from 0 (saturating_sub)
                const paraId = 2001n;
                const credits = (await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId)).toJSON();
                expect(credits, "Container chain 2001 does not have enough credits").toBeGreaterThanOrEqual(2n);

                // Should have assigned collators
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);

                // Set credits to 0
                const tx = polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId, 0n);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                // After 2 sessions, the container-chain should not be assigned
                await jumpSessions(context, 2);
                const collatorsAfter = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collatorsAfter.toJSON().containerChains[paraId],
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined();
            },
        });

        it({
            id: "E05",
            title: "Can buy additional credits",
            test: async function () {
                // As alice, buy credits for para 2000. Check that it is assigned collators again
                const paraId = 2000n;

                // Create blocks until no collators are assigned to any container chain
                for (;;) {
                    await context.createBlock();
                    const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                    if (Object.keys(collators.toJSON().containerChains).length == 0) {
                        break;
                    }
                }

                // Use random account instead of alice because alice is getting block rewards
                const randomAccount = generateKeyringPair("sr25519");
                const value = 100_000_000_000n;
                await context.createBlock([
                    await polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, value).signAsync(alice),
                ]);

                // Now, buy some credits for container chain 2000
                const balanceBefore = (
                    await polkadotJs.query.system.account(randomAccount.address)
                ).data.free.toBigInt();
                const purchasedCredits = 100n;

                const requiredBalance = purchasedCredits * 100_000_000n;
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, requiredBalance);
                await context.createBlock([await tx.signAsync(randomAccount)]);

                const balanceAfter = (
                    await polkadotJs.query.system.account(randomAccount.address)
                ).data.free.toBigInt();
                expect(balanceAfter).toBeLessThan(balanceBefore);

                const balanceTank = (await polkadotJs.query.system.account(paraIdTank(paraId))).data.free.toBigInt();
                expect(balanceTank).toBe(requiredBalance);

                // Check that after 2 sessions, container chain 2000 has collators and is producing blocks
                await jumpSessions(context, 2);

                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);
                expect(balanceTank).toBe(requiredBalance);
            },
        });
    },
});
