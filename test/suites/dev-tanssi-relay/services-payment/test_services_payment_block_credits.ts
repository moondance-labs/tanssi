import "@tanssi/api-augment";

import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions, jumpToSession, paraIdTank } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1201",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightSP: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSP =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT.includes(specVersion);
        });
        it({
            id: "E01",
            title: "Genesis container chains have credits and collators",
            test: async () => {
                await context.createBlock();

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(2000, 1000n).signAsync(alice)
                    );
                    return;
                }

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);
                // Since collators are not assigned until session 2, we need to go till session 2 to actually see heads being injected
                await jumpToSession(context, 2);
                const parasRegistered = (await polkadotJs.query.containerRegistrar.registeredParaIds())
                    .toArray()
                    .map((p) => p.toNumber());

                for (const paraId of parasRegistered) {
                    // Should have credits
                    const credits = await polkadotJs.query.servicesPayment.blockProductionCredits(paraId);
                    expect(
                        credits.toJSON(),
                        `Container chain ${paraId} does not have enough credits at genesis`
                    ).toBeGreaterThanOrEqual(2n * blocksPerSession);

                    // Should have assigned collators
                    const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();

                    // We are evaluating blockCredits for now, so lets put a lot of collatorAssignmentCredits
                    const tx = polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId, 1000n);
                    await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                    // Container chain 2001 does not have any collators, this will result in only 1 container chain
                    // producing blocks at a time. So if both container chains have 1000 credits, container 2000
                    // will produce blocks 0-999, and container 2001 will produce blocks 1000-1999.
                    if (paraId === 2000) {
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
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    return;
                }

                // Read num credits of para 2000, then create that many blocks. Check that authorNoting.blockNum does not increase anymore
                // and collatorAssignment does not have collators

                // create at least a couple blocks to at least see one block being consumed
                // we will be doing this for the whole test, i.e., creating two blocks to ensure the parachain advances
                await context.createBlock();
                await context.createBlock();

                const paraId = 2000;

                // Create a block, the block number should increase, and the number of credits should decrease
                const credits1 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum1 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();

                // create at least a couple blocks to at least see one block being consumed
                await context.createBlock();
                await context.createBlock();

                const credits2 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum2 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
                expect(containerBlockNum1, "container chain 2000 did not create a block").toBeLessThan(
                    containerBlockNum2
                );
                expect(credits1, "container chain 2000 created a block without burning any credits").toBeGreaterThan(
                    credits2
                );

                expect(
                    credits1 - credits2,
                    "container chain 2000 created a block without burning any credits"
                ).to.be.eq(containerBlockNum2 - containerBlockNum1);
            },
        });

        it({
            id: "E03",
            title: "Collators are unassigned when a container chain does not have enough credits",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    return;
                }
                // Create blocks until authorNoting.blockNum does not increase anymore.
                // Check that collatorAssignment does not have collators and num credits is less than 2 sessions.

                const paraId = 2000;

                // Create blocks until the block number stops increasing
                let containerBlockNum3 = -1;
                let containerBlockNum4 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();

                while (containerBlockNum3 !== containerBlockNum4) {
                    await context.createBlock();
                    await context.createBlock();
                    containerBlockNum3 = containerBlockNum4;
                    containerBlockNum4 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                        .unwrap()
                        .blockNumber.toNumber();
                }

                // Now the container chain should have less than 2 sessions worth of credits
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                expect(
                    credits,
                    "Container chain 2000 has stopped producing blocks, so it should not have enough credits"
                ).toBeLessThan(2n * blocksPerSession);

                const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId],
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined();
            },
        });

        it({
            id: "E04",
            title: "Root can remove credits",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(2000, 1000n).signAsync(alice)
                    );
                    return;
                }

                // Remove all the credits of container chain 2001, which should have assigned collators now
                // This checks that the node does not panic when we try to subtract credits from 0 (saturating_sub)

                const paraId = 2001n;
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                expect(credits, "Container chain 2001 does not have enough credits").toBeGreaterThanOrEqual(
                    2n * blocksPerSession
                );

                // Should have assigned collators
                const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);

                await context.createBlock();
                await context.createBlock();
                await context.createBlock();

                // Create a block, the block number should increase, and the number of credits should decrease
                const credits1 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum1 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();

                await context.createBlock();
                await context.createBlock();
                const credits2 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                const containerBlockNum2 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
                expect(containerBlockNum1, "container chain 2001 did not create a block").toBeLessThan(
                    containerBlockNum2
                );
                expect(credits1, "container chain 2001 created a block without burning any credits").toBeGreaterThan(
                    credits2
                );

                // Set credits to 0
                const tx = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId, 0n);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const credits3 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON() || 0;
                expect(credits3).toBe(0);
                // Can still create blocks
                const containerBlockNum3 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
                await context.createBlock();
                await context.createBlock();
                const credits4 = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON() || 0;
                const containerBlockNum4 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
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
            test: async () => {
                // As alice, buy credits for para 2000. Check that it is assigned collators again
                const paraId = 2000;

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E05 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1000n).signAsync(alice)
                    );
                    return;
                }

                // Create blocks until no collators are assigned to any container chain
                for (;;) {
                    await context.createBlock();
                    const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                    if (Object.keys(collators.toJSON().containerChains).length === 0) {
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
                const purchasedCredits = 1000n * blocksPerSession;

                const requiredBalance = purchasedCredits * 1_000_000n;
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

                // spend all credits
                let creditsRemaining = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                while (creditsRemaining !== 0) {
                    await context.createBlock();
                    await context.createBlock();
                    creditsRemaining = (await polkadotJs.query.servicesPayment.blockProductionCredits(paraId)).toJSON();
                }

                // create a new block that should trigger para balance to go down
                await context.createBlock();
                await context.createBlock();

                const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);
                expect(balanceTank).toBe(requiredBalance);

                // Create a block, the block number should increase, and the number of credits should decrease
                const containerBlockNum3 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
                await context.createBlock();
                await context.createBlock();
                const containerBlockNum4 = (await polkadotJs.query.authorNoting.latestAuthor(paraId))
                    .unwrap()
                    .blockNumber.toNumber();
                expect(containerBlockNum3, "container chain 2000 did not create a block").toBeLessThan(
                    containerBlockNum4
                );
                const balanceTankAfter = (
                    await polkadotJs.query.system.account(paraIdTank(paraId))
                ).data.free.toBigInt();
                expect(balanceTank, "container chain 2000 created a block without burning any credits").toBeGreaterThan(
                    balanceTankAfter
                );
            },
        });
    },
});
