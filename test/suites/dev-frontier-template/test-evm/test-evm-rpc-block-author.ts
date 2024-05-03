import { expect, describeSuite } from "@moonwall/cli";
import { customWeb3Request } from "@moonwall/util";

describeSuite({
    id: "DF0802",
    title: "Pallet EVM - RPC block author",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "should return correct author",
            test: async function () {
                await context.createBlock();

                const author = await context.polkadotJs().query.authorInherent.author();

                const latestBlock = (await customWeb3Request(context.web3(), "eth_getBlockByNumber", ["latest", false]))
                    .result;

                expect(latestBlock.author).eq(author.toString().substring(0, 42));
            },
        });
    },
});
