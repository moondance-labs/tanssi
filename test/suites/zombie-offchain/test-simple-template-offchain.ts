import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
import { ApiPromise, Keyring } from "@polkadot/api";
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
            title: "Offchain events are not emitted for simple container chain when offchain pallet testing is not enabled",
            test: async function () {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
                await context.waitBlock(10, "Container2000");

                let offchainWorkerStatus = await container2000Api.query.offchainWorker.offchainWorkerTestingEnabled();
                expect(offchainWorkerStatus).to.be.equal(false);

                const events = await container2000Api.query.system.events();
                const offchainWorkerEvents = events.filter((a) => {
                    return a.event.method == "SimpleOffchainEvent";
                });
                expect(offchainWorkerEvents.length).to.be.equal(0);
            },
        });

        it({
            id: "T02",
            title: "Offchain events are not emitted for simple container chain when offchain pallet testing is enabled",
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);

                // Enable off-chain worker
                const switchTx = container2000Api.tx.offchainWorker.switchOffchainWorker();
                await container2000Api.tx.sudo.sudo(switchTx).signAndSend(alice);

                await context.waitBlock(10, "Container2000");

                const events = await container2000Api.query.system.events();
                const offchainWorkerEvents = events.filter((a) => {
                    return a.event.method == "SimpleOffchainEvent";
                });
                expect(offchainWorkerEvents.length).to.be.equal(0);
            },
        });
    },
});
