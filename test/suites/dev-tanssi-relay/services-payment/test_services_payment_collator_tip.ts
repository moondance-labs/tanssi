import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@moonwall/util";
import { fetchCollatorAssignmentTip, jumpSessions } from "util/block";

describeSuite({
    id: "DTR0901",
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
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;

                const tip = 123;

                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                await context.createBlock([await tx.signAsync(alice)]);

                const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                // In genesis we have 4 collators, hence if we make 4 collators per para, we make sure the one
                // with priority gets them
                const changeCollatorsPerChain = polkadotJs.tx.collatorConfiguration.setCollatorsPerContainer(4);
                await context.createBlock([
                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.utility.batchAll([txMaxTip, changeCollatorsPerChain]))
                        .signAsync(alice),
                ]);
                await jumpSessions(context, 2);

                const collators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 4 collators`
                ).toBe(4);

                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent.tip.toNumber()).to.be.equal(tip);
            },
        });
    },
});
