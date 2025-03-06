import "@tanssi/api-augment";

import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import fs from "node:fs";

describeSuite({
    id: "ZO01",
    title: "Zombie Dancelight Upgrade Test",
    foundationMethods: "zombie",
    testCases: ({ it, context, log }) => {
        let relayApi: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            relayApi = context.polkadotJs("parachain");

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            const currentBlock = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
            expect(currentBlock, "Parachain not producing blocks").to.be.greaterThan(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async () => {
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Chain can be upgraded",
            timeout: 600000,
            test: async ({ skip }) => {
                const blockNumberBefore = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                const currentCode = await relayApi.rpc.state.getStorage(":code");
                const codeString = currentCode.toString();

                const wasm = fs.readFileSync((await MoonwallContext.getContext()).rtUpgradePath);
                const rtHex = `0x${wasm.toString("hex")}`;
                const rtBefore = relayApi.consts.system.version.specVersion.toNumber();

                if (rtHex === codeString) {
                    log("Runtime already upgraded, skipping test");
                    skip();
                }
                log("Runtime not upgraded, proceeding with test");
                log("Current runtime spec version:", rtBefore);
                log(`Current runtime hash: ${rtHex.slice(0, 10)}...${rtHex.slice(-10)}`);
                log(`New runtime hash: ${codeString.slice(0, 10)}...${codeString.slice(-10)}`);

                await context.upgradeRuntime({ from: alice, logger: log });
                await context.waitBlock(2);
                const rtafter = relayApi.consts.system.version.specVersion.toNumber();
                log("New runtime spec version:", rtafter);
                if (rtBefore === rtafter) {
                    throw new Error("Runtime upgrade failed");
                }
                const blockNumberAfter = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Before: #${blockNumberBefore}, After: #${blockNumberAfter}`);
                expect(blockNumberAfter, "Block number did not increase").to.be.greaterThan(blockNumberBefore);
            },
        });

        it({
            id: "T03",
            title: "Can send balance transfers",
            timeout: 600000,
            test: async () => {
                const randomAccount = generateKeyringPair("sr25519");

                let tries = 0;
                const balanceBefore = (await relayApi.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// Chopsticks does not have the notion of tx pool either, so we need to retry
                /// Therefore we just retry at most MAX_BALANCE_TRANSFER_TRIES
                const MAX_BALANCE_TRANSFER_TRIES = 5;
                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await relayApi.tx.balances
                        .transferAllowDeath(randomAccount.address, 1_000_000_000)
                        .signAndSend(alice);
                    await context.waitBlock(1);

                    const block = await relayApi.rpc.chain.getBlock();
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    tries++;
                }

                const balanceAfter = (await relayApi.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });
    },
});
