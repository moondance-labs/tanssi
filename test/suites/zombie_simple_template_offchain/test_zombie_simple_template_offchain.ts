import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { signAndSendAndInclude, isEventEmittedInTheNextBlocks } from "../../util/block.ts";
describeSuite({
    id: "ZOF01",
    title: "Zombie Offchain Tests",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        const baseBlockWaitingInterval: number = 10;

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
            test: async () => {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
                const isOffchainEventEmitted = await isEventEmittedInTheNextBlocks(
                    context,
                    container2000Api,
                    baseBlockWaitingInterval,
                    "Container2000",
                    "SimpleOffchainEvent"
                );
                expect(isOffchainEventEmitted).to.be.false;
            },
        });

        it({
            id: "T02",
            title: "Offchain events are not emitted for simple container chain when offchain pallet testing is enabled",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
                const ocwOnTx = container2000Api.tx.offchainWorker.setOffchainWorker(true);

                // Enable off-chain worker test event emission
                await signAndSendAndInclude(container2000Api.tx.sudo.sudo(ocwOnTx), alice);
                const isOffchainEventEmitted1 = await isEventEmittedInTheNextBlocks(
                    context,
                    container2000Api,
                    baseBlockWaitingInterval,
                    "Container2000",
                    "SimpleOffchainEvent"
                );
                expect(isOffchainEventEmitted1).to.be.false;

                // Disable off-chain worker test event emission
                const ocwOffTx = container2000Api.tx.offchainWorker.setOffchainWorker(false);
                await signAndSendAndInclude(container2000Api.tx.sudo.sudo(ocwOffTx), alice);
                const isOffchainEventEmitted2 = await isEventEmittedInTheNextBlocks(
                    context,
                    container2000Api,
                    baseBlockWaitingInterval,
                    "Container2000",
                    "SimpleOffchainEvent"
                );
                expect(isOffchainEventEmitted2).to.be.false;
            },
        });
    },
});
