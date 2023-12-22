import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import fs from "node:fs";

describeSuite({
    id: "R01",
    title: "Zombie Dancebox Upgrade Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context, log }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            paraApi = context.polkadotJs("parachain");
            relayApi = context.polkadotJs("relaychain");

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

            const paraNetwork = paraApi.consts.system.version.specName.toString();
            expect(paraNetwork, "Para API incorrect").to.contain("dancebox");

            const currentBlock = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
            expect(currentBlock, "Parachain not producing blocks").to.be.greaterThan(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async function () {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Chain can be upgraded",
            timeout: 600000,
            test: async function () {
                const blockNumberBefore = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                const currentCode = await paraApi.rpc.state.getStorage(":code");
                const codeString = currentCode.toString();

                const wasm = fs.readFileSync((await MoonwallContext.getContext()).rtUpgradePath);
                const rtHex = `0x${wasm.toString("hex")}`;

                if (rtHex === codeString) {
                    log("Runtime already upgraded, skipping test");
                    return;
                } else {
                    log("Runtime not upgraded, proceeding with test");
                    log("Current runtime hash: " + rtHex.slice(0, 10) + "..." + rtHex.slice(-10));
                    log("New runtime hash: " + codeString.slice(0, 10) + "..." + codeString.slice(-10));
                }

                await context.upgradeRuntime({ from: alice, logger: log });
                await context.waitBlock(2);
                const blockNumberAfter = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Before: #${blockNumberBefore}, After: #${blockNumberAfter}`);
                expect(blockNumberAfter, "Block number did not increase").to.be.greaterThan(blockNumberBefore);
            },
        });
    },
});
