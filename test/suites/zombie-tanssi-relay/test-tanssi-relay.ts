import { describeSuite, expect } from "@moonwall/cli";

describeSuite({
    id: "ZR-01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        it({
            id: "T01",
            title: "Blocks are being produced on tanssi-relay",
            test: async function () {
                console.log("executing");
                const relayApi = context.polkadotJs("Tanssi-relay");
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("starlight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });
    },
});
