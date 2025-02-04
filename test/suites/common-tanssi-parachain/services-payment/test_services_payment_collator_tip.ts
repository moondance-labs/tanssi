import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@moonwall/util";
import { fetchCollatorAssignmentTip, jumpSessions } from "util/block";
import { paraIdTank } from "../../../util/payment.ts";

describeSuite({
    id: "CPT0608",
    title: "Services payment collator assignment tip test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let collatorAssignmentAlias;
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
            title: "Tip should prioritize collator assignment",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;

                const tip = 123;

                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                await context.createBlock([await tx.signAsync(alice)]);

                const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(txMaxTip).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 2 collators`
                ).toBe(2);

                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent.tip.toNumber()).to.be.equal(tip);
            },
        });
        it({
            id: "E02",
            title: "Tip is not charged when there are enough collators for all chains",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;
                const otherParaId = 2000n;

                // Deregister the other chain, 2000, so that 2001 is the only chain
                const txDeregister = polkadotJs.tx.registrar.deregister(otherParaId);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(txDeregister).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 2 collators`
                ).toBe(2);

                // No tip event
                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent).to.be.undefined;
            },
        });
        it({
            id: "E03",
            title: "If parachain tank account does not have enough balance, collators are not assigned",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;

                // Set tank account to 0 balance, shouldn't matter because the chain doesn't need tip, and also the
                // chain has credits.
                // But actually this will make some checks fail, and result in the chain being not assigned any collators.
                const tx = polkadotJs.tx.balances.forceSetBalance(paraIdTank(paraId), 0);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId]?.length,
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined;

                // No tip event
                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent).to.be.undefined;
            },
        });
    },
});
