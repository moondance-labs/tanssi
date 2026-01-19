import "@tanssi/api-augment";

import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import fs from "node:fs";
import { testPalletVersions } from "../../utils";

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
            test: async ({ skip }) => {
                const blockNumberBefore = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                const currentCode = await paraApi.rpc.state.getStorage(":code");
                const codeString = currentCode.toString();

                const wasm = fs.readFileSync((await MoonwallContext.getContext()).rtUpgradePath);
                const rtHex = `0x${wasm.toString("hex")}`;
                const rtBefore = paraApi.consts.system.version.specVersion.toNumber();

                if (rtHex === codeString) {
                    log("Runtime already upgraded, skipping test");
                    skip();
                }
                log(`Current runtime spec version: ${rtBefore}`);
                log("Runtime not upgraded, proceeding with test");
                log(`Current runtime bytes: ${rtHex.slice(0, 10)}...${rtHex.slice(-10)}`);
                log(`New runtime bytes: ${codeString.slice(0, 10)}...${codeString.slice(-10)}`);

                await context.upgradeRuntime({ from: alice_or_alith });
                await context.waitBlock(2);
                const rtafter = paraApi.consts.system.version.specVersion.toNumber();
                log(`New runtime spec version: ${rtafter}`);
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
            test: async () => {
                const randomAccount = generateKeyringPair("sr25519");

                let tries = 0;
                const balanceBefore = (await paraApi.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// But the txpool is smart and it will keep the transaction until it can be included.
                /// So wait for MAX_BALANCE_TRANSFER_WAIT_BLOCKS
                const txHash = await paraApi.tx.balances
                    .transferAllowDeath(randomAccount.address, 1_000_000_000)
                    .signAndSend(alice_or_alith);
                await context.waitBlock(1);
                const MAX_BALANCE_TRANSFER_WAIT_BLOCKS = 5;
                while (tries < MAX_BALANCE_TRANSFER_WAIT_BLOCKS) {
                    const block = await paraApi.rpc.chain.getBlock();
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    await context.waitBlock(1);
                    tries++;
                }

                const balanceAfter = (await paraApi.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });

        it({
            id: "T04",
            title: "Test pallet versions for missed migrations",
            test: async () => {
                let command: string;
                const container2001Network = paraApi.consts.system.version.specName.toString();
                if (container2001Network.includes("frontier-template")) {
                    command = "../target/release/container-chain-frontier-node";
                } else {
                    command = "../target/release/container-chain-simple-node";
                }
                const args = ["export-chain-spec", "--raw"];

                await testPalletVersions(paraApi, command, args);
            },
        });
    },
});
