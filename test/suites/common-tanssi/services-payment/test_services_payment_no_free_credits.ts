import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "COMM0205",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const paraId2000 = 2000;
        const paraId2001 = 2001;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;
        const blocksPerSession = 10n;
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
            shouldSkipStarlightSP =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT.includes(specVersion);
        });
        it({
            id: "E01",
            title: "Removing credits should make chains not get collators",
            test: async () => {
                const removeFreeCredits = polkadotJs.tx.utility.batch([
                    polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2000, 0n),
                    polkadotJs.tx.servicesPayment.setCollatorAssignmentCredits(paraId2001, 0n),
                    polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2000, 0n),
                    polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0n),
                ]);

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);

                    // We check that the call (without sudo) is filtered.
                    await checkCallIsFiltered(context, polkadotJs, await removeFreeCredits.signAsync(alice));
                    return;
                }

                const sudoSignedTx = await polkadotJs.tx.sudo.sudo(removeFreeCredits).signAsync(alice);
                await context.createBlock([sudoSignedTx]);
                // Check that after 2 sessions, chain is deregistered
                await jumpSessions(context, 2);

                await context.createBlock();
                // Should not have assigned collators
                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();

                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2001.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E02",
            title: "Buying credits only for collator-assignment is not enough",
            test: async () => {
                const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
                // Now, buy some credits for container chain 2000. we only buy ones session -1
                const purchasedCredits = costPerSession + existentialDeposit;
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
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
        it({
            id: "E03",
            title: "Additionally buying credits only for block-credits makes it assigned",
            test: async () => {
                // Now, buy some credits for container chain 2000. we only buy ones session -1
                const purchasedCredits = blocksPerSession * costPerBlock * 2n;
                // Check that after 2 sessions, container chain 2000 has not collators
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);

                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);

                // Check that after 2 sessions, container chain 2000 has collators
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())].length,
                    `Container chain ${paraId2000} has 0 collators`
                ).toBeGreaterThan(0);
            },
        });
        it({
            id: "E04",
            title: "Just one session later they should be unassinged",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    return;
                }
                // Check that after 1 sessions
                await jumpSessions(context, 1);

                const collators = await collatorAssignmentAlias.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[Number.parseInt(paraId2000.toString())],
                    `Container chain ${paraId2000} should have 0 collators`
                ).toBeUndefined();
            },
        });
    },
});
