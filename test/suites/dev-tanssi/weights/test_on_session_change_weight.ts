import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "DT0401",
    title: "On session change weights suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let maxBlock: number;

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
            maxBlock = polkadotJs.consts.system.blockWeights.maxBlock.refTime.toNumber();
        });

        it({
            id: "E01",
            title: "Block weight on session change should be max",
            test: async function () {
                // Let's jump one session
                await jumpSessions(context, 1);

                // TODO: fix once we have types
                const blockWeight = (await polkadotJs.query.system.blockWeight()).toJSON();
                expect(blockWeight.normal).to.deep.equal({ refTime: 0, proofSize: 0 });
                expect(blockWeight.operational).to.deep.equal({
                    refTime: 0,
                    proofSize: 0,
                });
                expect(blockWeight.mandatory.refTime).to.be.greaterThan(maxBlock);
            },
        });

        it({
            id: "E02",
            title: "Block weight not on session change should be small",
            test: async function () {
                await context.createBlock();

                // TODO: fix once we have types
                const blockWeight = (await polkadotJs.query.system.blockWeight()).toJSON();
                expect(blockWeight.normal).to.deep.equal({ refTime: 0, proofSize: 0 });
                expect(blockWeight.operational).to.deep.equal({
                    refTime: 0,
                    proofSize: 0,
                });
                expect(blockWeight.mandatory.refTime).to.be.lessThan(maxBlock);
            },
        });
    },
});
