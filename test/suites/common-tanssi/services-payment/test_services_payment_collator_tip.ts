import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { fetchCollatorAssignmentTip, jumpSessions } from "util/block";

describeSuite({
    id: "CT0608",
    title: "Services payment collator assignment tip test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Tip should prioritize collator assignment",
            test: async function () {
                await context.createBlock();

                const paraId = 2001n;

                const tip = 123;

                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                await context.createBlock([await tx.signAsync(alice)]);

                const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(txMaxTip).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 2 collators`
                ).toBe(2);

                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent.tip.toNumber()).to.be.equal(tip);
            },
        });
    },
});
