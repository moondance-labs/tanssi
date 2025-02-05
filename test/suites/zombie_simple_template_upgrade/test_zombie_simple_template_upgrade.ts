import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { alith } from "@moonwall/util";

import fs from "node:fs";

describeSuite({
    id: "ZOMBI01",
    title: "Zombie Container Template Upgrade Test",
    foundationMethods: "zombie",
    testCases: ({ it, context, log }) => {
        let paraApi: ApiPromise;
        let alice_or_alith: KeyringPair;
        beforeAll(async () => {
            paraApi = context.polkadotJs("parachain");
            const container2001Network = paraApi.consts.system.version.specName.toString();
            if (container2001Network.includes("frontier-template")) {
                alice_or_alith = alith;
            } else {
                const keyring = new Keyring({ type: "sr25519" });
                alice_or_alith = keyring.addFromUri("//Alice", { name: "Alice default" });
            }
            const currentBlock = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
            expect(currentBlock, "Parachain not producing blocks").to.be.greaterThan(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async () => {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Chain can be upgraded",
            timeout: 600000,
            test: async () => {
                const blockNumberBefore = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                const currentCode = await paraApi.rpc.state.getStorage(":code");
                const codeString = currentCode.toString();
                const rtBefore = paraApi.consts.system.version.specVersion.toNumber();

                const wasm = fs.readFileSync((await MoonwallContext.getContext()).rtUpgradePath);
                const rtHex = `0x${wasm.toString("hex")}`;

                if (rtHex === codeString) {
                    log("Runtime already upgraded, skipping test");
                    return;
                }
                log("Runtime not upgraded, proceeding with test");
                log(`Current runtime hash: ${rtHex.slice(0, 10)}...${rtHex.slice(-10)}`);
                log(`New runtime hash: ${codeString.slice(0, 10)}...${codeString.slice(-10)}`);

                await context.upgradeRuntime({ from: alice_or_alith, logger: log });
                await context.waitBlock(2);
                const rtafter = paraApi.consts.system.version.specVersion.toNumber();
                if (rtBefore === rtafter) {
                    throw new Error("Runtime upgrade failed");
                }
                const blockNumberAfter = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Before: #${blockNumberBefore}, After: #${blockNumberAfter}`);
                expect(blockNumberAfter, "Block number did not increase").to.be.greaterThan(blockNumberBefore);
            },
        });
    },
});
