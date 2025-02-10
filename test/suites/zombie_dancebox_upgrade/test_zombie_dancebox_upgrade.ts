import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";
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
                const rtBefore = paraApi.consts.system.version.specVersion.toNumber();

                if (rtHex === codeString) {
                    log("Runtime already upgraded, skipping test");
                    return;
                } else {
                    log("Runtime not upgraded, proceeding with test");
                    log("Current runtime spec version:", rtBefore);
                    log("Current runtime bytes: " + rtHex.slice(0, 10) + "..." + rtHex.slice(-10));
                    log("New runtime bytes: " + codeString.slice(0, 10) + "..." + codeString.slice(-10));
                }

                await context.upgradeRuntime({ from: alice, logger: log });
                await context.waitBlock(2);
                const rtafter = paraApi.consts.system.version.specVersion.toNumber();
                log("New runtime spec version:", rtafter);
                if (rtBefore === rtafter) {
                    throw new Error("Runtime upgrade failed");
                }
                const blockNumberAfter = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Before: #${blockNumberBefore}, After: #${blockNumberAfter}`);
                expect(blockNumberAfter, "Block number did not increase").to.be.greaterThan(blockNumberBefore);
            },
        });

        it({
            id: "T03",
            title: "Can send balance transfers",
            timeout: 600000,
            test: async function () {
                const randomAccount = generateKeyringPair("sr25519");

                let tries = 0;
                const balanceBefore = (await paraApi.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// Chopsticks does not have the notion of tx pool either, so we need to retry
                /// Therefore we just retry at most MAX_BALANCE_TRANSFER_TRIES
                const MAX_BALANCE_TRANSFER_TRIES = 5;
                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await paraApi.tx.balances
                        .transferAllowDeath(randomAccount.address, 1_000_000_000)
                        .signAndSend(alice);
                    await context.waitBlock(1);

                    const block = await paraApi.rpc.chain.getBlock();
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    tries++;
                }

                const balanceAfter = (await paraApi.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });
    },
});
