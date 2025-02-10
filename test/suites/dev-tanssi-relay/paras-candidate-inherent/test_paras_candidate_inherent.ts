import "@tanssi/api-augment";
import { describeSuite, customDevRpcRequest, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "util/block";
import { getHeaderFromRelay } from "util/relayInterface.ts";

describeSuite({
    id: "DEVT1101",
    title: "Paras inherent tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Paras heads should be updated every block",
            test: async () => {
                const parasHeadGenesis = await polkadotJs.query.paras.heads(2000);
                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);
                // Since collators are not assigned until session 2, we need to go till session 2 to actually see heads being injected
                await jumpToSession(context, 3);
                await context.createBlock();
                const parasHeadAfterOneBlock = await polkadotJs.query.paras.heads(2000);
                expect(parasHeadAfterOneBlock).to.not.be.eq(parasHeadGenesis);
                await context.createBlock();
                // we create one more block to test we are persisting candidates every block
                const parasHeadAfterTwoBlocks = await polkadotJs.query.paras.heads(2000);
                expect(parasHeadAfterOneBlock).to.not.be.eq(parasHeadAfterTwoBlocks);
                const header2000 = await getHeaderFromRelay(context.polkadotJs(), 2000);
                expect(header2000.number.toBigInt()).to.be.equal(31n);
            },
        });
    },
});
