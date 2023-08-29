import { describeSuite, expect } from "@moonwall/cli";

describeSuite({
    id: "D01",
    title: "Dev test suite",
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {
        it({
            id: "E01",
            title: "Checking that launched node can create blocks",
            test: async function () {
                const block = (await context.pjsApi.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock();

                const block2 = (await context.pjsApi.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Original block #${block}, new block #${block2}`);
                expect(block2).to.be.greaterThan(block);
            },
        });
    },
});
