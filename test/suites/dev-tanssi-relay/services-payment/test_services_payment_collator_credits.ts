import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions, jumpToSession, paraIdTank } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1202",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const startingCredits = 100n;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightSP: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSP = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT.includes(specVersion);
        });
        it({
            id: "E01",
            title: "Genesis container chains have credits and collators and should have one less credit",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    return;
                }

                await context.createBlock();
                await jumpToSession(context, 1);

                const parasRegistered = await polkadotJs.query.containerRegistrar.registeredParaIds();

                for (const paraId of parasRegistered) {
                    // Should have credits
                    const credits = await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId);

                    // Should have assigned collators
                    const collators = await polkadotJs.query.tanssiCollatorAssignment.pendingCollatorContainerChain();

                    expect(
                        credits.unwrap().toBigInt(),
                        `Container chain ${paraId} should have applied session credits`
                    ).toBe(startingCredits - 1n);
                    expect(
                        collators.toJSON().containerChains[paraId.toString()].length,
                        `Container chain ${paraId} has 0 collators`
                    ).toBeGreaterThan(0);
                }
            },
        });

        it({
            id: "E02",
            title: "Getting assignation should consume credits",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    return;
                }
                await jumpToSession(context, 2);

                // Moving to the next session should have reduced the credit by one to both parachains
                // even if one does not produce blocks

                const paraId = 2000;
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
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    return;
                }
                // Create blocks until authorNoting.blockNum does not increase anymore.
                // Check that collatorAssignment does not have collators and num credits is less than 2 sessions.

                const paraId = 2000;

                // Create blocks until credits reach 0
                let containerCredits = (await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId))
                    .unwrap()
                    .toBigInt();

                while (containerCredits > 0n) {
                    await context.createBlock();
                    containerCredits = (await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId))
                        .unwrap()
                        .toBigInt();
                }

                // Right now we run out of credits in the last assgination, so if we advance one more session, we should see no collators assigned
                // in pending
                await jumpSessions(context, 1);

                // Now the container chain should have less than 2 sessions worth of credits
                const credits = (await polkadotJs.query.servicesPayment.collatorAssignmentCredits(paraId)).toJSON();
                expect(
                    credits,
                    "Container chain 2000 has stopped producing blocks, so it should not have enough credits"
                ).toBeLessThan(2n);

                const collators = await polkadotJs.query.tanssiCollatorAssignment.pendingCollatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId],
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined();
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
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1000n).signAsync(alice));  
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

                const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} has 0 collators`
                ).toBeGreaterThan(0);
                expect(balanceTank).toBe(requiredBalance);
            },
        });
    },
});
