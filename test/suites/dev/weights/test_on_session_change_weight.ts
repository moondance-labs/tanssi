import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

import "@polkadot/api-augment";

describeSuite({
  id: "D06",
  title: "On session change weights suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    beforeAll(() => {
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Block weight on session change should be max",
        test: async function () {
            // Let's jump one session
            await jumpSessions(context, 1);

            const blockWeight = (await polkadotJs.query.system.blockWeight()).toJSON();
            expect(blockWeight.normal).to.deep.equal({ refTime: 0, proofSize: 0 });
            expect(blockWeight.operational).to.deep.equal({ refTime: 0, proofSize: 0 });
            expect(blockWeight.mandatory.refTime).to.be.greaterThan(500000000000);
        },
    });

    it({
        id: "E02",
        title: "Block weight not on session change should be small",
        test: async function () {
            await context.createBlock();

            const blockWeight = (await polkadotJs.query.system.blockWeight()).toJSON();
            expect(blockWeight.normal).to.deep.equal({ refTime: 0, proofSize: 0 });
            expect(blockWeight.operational).to.deep.equal({ refTime: 0, proofSize: 0 });
            expect(blockWeight.mandatory.refTime).to.be.lessThan(500000000000);
        },
    });
    },
});
