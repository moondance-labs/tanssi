import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
import { ApiPromise } from "@polkadot/api";
describeSuite({
    id: "ZOF-01",
    title: "Zombie Offchain Tests",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            container2000Api = context.polkadotJs("Container2000");

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);

            expect(header2000.number.toNumber()).to.be.equal(0);
        }, 50000);

        it({
            id: "T01",
            title: "Offchain events are not emitted for simple container chain template",
            test: async function () {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
                await context.waitBlock(6, "Container2000");
                // Create our API with a default connection to the local node
                const api = await ApiPromise.create();
                const events = await api.query.system.events();
                expect(events).to.be.empty;
            },
        });
    },
});
