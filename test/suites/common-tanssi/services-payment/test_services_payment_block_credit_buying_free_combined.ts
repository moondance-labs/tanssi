import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";

describeSuite({
    id: "COMM0203",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        const paraId2000 = 2000;
        const paraId2001 = 2001;
        const costPerBlock = 1_000_000n;
        let collatorAssignmentAlias: any;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            collatorAssignmentAlias = runtimeName.includes("light")
                ? polkadotJs.query.tanssiCollatorAssignment
                : polkadotJs.query.collatorAssignment;
        });

        it({
            id: "E01",
            title: "Collators are unassigned when a container chain does not have enough block credits",
            test: async () => {
                // Create blocks until authorNoting.blockNum does not increase anymore.
                // Check that collatorAssignment does not have collators and num credits is less than 2 sessions.

                const tx2000free = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2000, 0n);
                const tx2001free = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0n);

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
            title: "Collators are not assigned when we buy 1 session + ED -1 of block credits",
            test: async () => {
                // Set half of the needed block production credits as free credits
                const tx2000OneSession = polkadotJs.tx.servicesPayment.setBlockProductionCredits(
                    paraId2000,
                    blocksPerSession / 2n
                );
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2000OneSession).signAsync(alice)]);
                const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
                // Now, buy some credits for container chain 2000. we only the second half of the needed credits - 1
                const purchasedCredits = (blocksPerSession / 2n) * costPerBlock + existentialDeposit - 1n;
                // Check that after 2 sessions, container chain 2000 has not collators
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);
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
            title: "Collators are assigned when we buy at least 2 session + ED of block credits",
            test: async () => {
                // Now, buy the remaining
                const purchasedCredits = 1n;
                // Purchase the remaining 1
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2000, purchasedCredits);
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
