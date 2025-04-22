import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "COMM0204",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const paraId2000 = 2000;
        const paraId2001 = 2001;
        const costPerSession = 100_000_000n;
        let collatorAssignmentAlias: any;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightSP: boolean;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            collatorAssignmentAlias = runtimeName.includes("light")
                ? polkadotJs.query.tanssiCollatorAssignment
                : polkadotJs.query.collatorAssignment;

            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSP = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Collators are unassigned when a container chain does not have enough collator assignment credits",
            test: async () => {
                // Create blocks until authorNoting.blockNum does not increase anymore.
                // Check that collatorAssignment does not have collators and num credits is less than 2 sessions.

                const tx2000free = polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2000, 0n);
                const tx2001free = polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2001, 0n);

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx2000free.signAsync(alice));
                    await checkCallIsFiltered(context, polkadotJs, await tx2001free.signAsync(alice));
                    return;
                }

                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2000free).signAsync(alice)]);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2001free).signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has collators and is producing blocks
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId2000],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E02",
            title: "Collators are not assigned when we buy a session + ED -1 of collator assignment credits",
            test: async () => {
                const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
                // Now, buy some credits for container chain 2000. we only buy ones session -1
                const purchasedCredits = costPerSession + existentialDeposit - 1n;
                // Check that after 2 sessions, container chain 2000 has not collators
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has 0 collators and is not producing blocks
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId2000],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E03",
            title: "Collators are assigned when we buy at least a session + ED of block credits",
            test: async () => {
                // Now, buy the remaining
                const purchasedCredits = 1n;
                // Purchase the remaining 1
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }
                
                await context.createBlock([await tx.signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has collators and is producing blocks
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId2000].length,
                    `Container chain ${paraId2000} has 0 collators`
                ).toBeGreaterThan(0);
            },
        });
    },
});
