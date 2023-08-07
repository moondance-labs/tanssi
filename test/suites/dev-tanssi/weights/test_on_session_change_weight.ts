import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

import "@tanssi-network/api-augment";

describeSuite({
  id: "D06",
  title: "On session change weights suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    let maxBlock: number;
    const anotherLogger = setupLogger("anotherLogger");
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
            const blockWeight = await polkadotJs.query.system.blockWeight();
            expect(blockWeight.normal.refTime.toBigInt()).to.equal(0n);
            expect(blockWeight.normal.proofSize.toBigInt()).to.equal(0n);
            expect(blockWeight.operational.refTime.toBigInt()).to.equal(0n);
            expect(blockWeight.operational.proofSize.toBigInt()).to.equal(0n);
            expect(blockWeight.mandatory.refTime.toNumber()).to.be.greaterThan(maxBlock);
        },
    });

    it({
        id: "E02",
        title: "Block weight not on session change should be small",
        test: async function () {
            await context.createBlock();

            // TODO: fix once we have types
            const blockWeight = (await polkadotJs.query.system.blockWeight());
            expect(blockWeight.normal.refTime.toBigInt()).to.equal(0n);
            expect(blockWeight.normal.proofSize.toBigInt()).to.equal(0n);
            expect(blockWeight.operational.refTime.toBigInt()).to.equal(0n);
            expect(blockWeight.operational.proofSize.toBigInt()).to.equal(0n);
            expect(blockWeight.mandatory.refTime.toNumber()).to.be.lessThan(maxBlock);
        },
    });
    },
});
