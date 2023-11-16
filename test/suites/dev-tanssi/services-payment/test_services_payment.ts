import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { generateKeyringPair, KeyringPair } from "@moonwall/util";
import { jumpSessions } from "util/block";

describeSuite({
    id: "SP0801",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 5n;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Genesis container chains have credits and collators",
            test: async function () {
                await context.createBlock();
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                for (const paraId of parasRegistered.toJSON()) {
                    // Should have credits
                    const credits = await polkadotJs.query.servicesPayment.blockProductionCredits(paraId);
                    expect(
                        credits.toJSON(),
                        `Container chain ${paraId} does not have enough credits at genesis`
                    ).toBeGreaterThanOrEqual(2n * blocksPerSession);

                    // Should have assigned collators
                    const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                    // Container chain 2001 does not have any collators, this will result in only 1 container chain
                    // producing blocks at a time. So if both container chains have 1000 credits, container 2000
                    // will produce blocks 0-999, and container 2001 will produce blocks 1000-1999.
                    if (paraId == 2000) {
                        expect(
                            collators.toJSON().containerChains[paraId].length,
                            `Container chain ${paraId} has 0 collators`
                        ).toBeGreaterThan(0);
                    }
                }
            },
        });

        it({
            id: "E02",
            title: "Creating a container chain block costs credits",
            test: async function () {
                // Read num credits of para 2000, then create that many blocks. Check that authorNoting.blockNum does not increase anymore
                // and collatorAssignment does not have collators

                const paraId = 2000n;

                // Create a block, the block number should increase, and the number of credits should decrease
                const credits1 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum1 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                await context.createBlock();
                const credits2 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum2 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                expect(containerBlockNum1, "container chain 2000 did not create a block").toBeLessThan(
                    containerBlockNum2
                );
                expect(credits1, "container chain 2000 created a block without burning any credits").toBeGreaterThan(
                    credits2
                );
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
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                expect(
                    credits,
                    "Container chain 2000 has stopped producing blocks, so it should not have enough credits"
                ).toBeLessThan(2n * blocksPerSession);

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
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                expect(credits, "Container chain 2001 does not have enough credits").toBeGreaterThanOrEqual(
                    2n * blocksPerSession
                );

                // Should have assigned collators
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);

                // Create a block, the block number should increase, and the number of credits should decrease
                const credits1 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum1 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                await context.createBlock();
                const credits2 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum2 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                expect(containerBlockNum1, "container chain 2001 did not create a block").toBeLessThan(
                    containerBlockNum2
                );
                expect(credits1, "container chain 2001 created a block without burning any credits").toBeGreaterThan(
                    credits2
                );

                // Set credits to 0
                const tx = polkadotJs.tx.servicesPayment.setCredits(paraId, 0n);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const credits3 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON() || 0;
                expect(credits3).toBe(0);
                // Can still create blocks
                const containerBlockNum3 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                await context.createBlock();
                const credits4 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON() || 0;
                const containerBlockNum4 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                expect(
                    containerBlockNum3,
                    "container chain 2001 did not create a block after root set credits to 0"
                ).toBeLessThan(containerBlockNum4);
                // But credits cannot be lower than 0
                expect(credits4, "container chain 2001 has negative credits").toBe(0);
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
                    await polkadotJs.tx.balances.transfer(randomAccount.address, value).signAsync(alice),
                ]);

                // Now, buy some credits for container chain 2000
                const balanceBefore = (
                    await polkadotJs.query.system.account(randomAccount.address)
                ).data.free.toBigInt();
                const credits1 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const purchasedCredits = 100n * blocksPerSession;

                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, purchasedCredits, null);
                await context.createBlock([await tx.signAsync(randomAccount)]);

                const balanceAfter = (
                    await polkadotJs.query.system.account(randomAccount.address)
                ).data.free.toBigInt();
                expect(balanceAfter).toBeLessThan(balanceBefore);
                const credits2 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                expect(BigInt(credits2)).toBe(BigInt(credits1) + purchasedCredits);

                // Check that after 2 sessions, container chain 2000 has collators and is producing blocks
                await jumpSessions(context, 2);

                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);

                // Create a block, the block number should increase, and the number of credits should decrease
                const credits3 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum3 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                await context.createBlock();
                const credits4 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum4 = await (await polkadotJs.query.authorNoting.latestAuthor(paraId)).toJSON()
                    .blockNumber;
                expect(containerBlockNum3, "container chain 2000 did not create a block").toBeLessThan(
                    containerBlockNum4
                );
                expect(credits3, "container chain 2000 created a block without burning any credits").toBeGreaterThan(
                    credits4
                );
            },
        });
    },
});
