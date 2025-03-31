import "@tanssi/api-augment";

import { type DevModeContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { encodeAddress, keccakAsHex, xxhashAsU8a } from "@polkadot/util-crypto";
import { readFileSync } from "node:fs";
import { generateEventLog, generateUpdate, SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, type MultiLocation } from "utils";
import { expectEventCount } from "../../../helpers/events";
import type { SnowbridgeCoreInboundLog } from "@polkadot/types/lookup";

describeSuite({
    id: "DTR1702",
    title: "EthereumTokenTransfers tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "transferNativeToken should send message to Ethereum",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                // Set channel info on EthereumTokenTransfers pallet.
                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                const tokenLocation: MultiLocation = {
                    parents: 0,
                    interior: "Here",
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "dance",
                    symbol: "dance",
                    decimals: 12,
                };

                // Register token on EthereumSystem.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                const recipient = "0x0000000000000000000000000000000000000007";
                const amount = 1000;

                // Finally call transferNativeToken extrinsic.
                const tx3 = await polkadotJs.tx.ethereumTokenTransfers
                    .transferNativeToken(amount, recipient)
                    .signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check events and digest were emitted correctly.
                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x.other);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0].other;

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                    NativeTokenTransferred: 1,
                });

                // Also a MessagesCommitted event with the same hash as the digest log
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "MessagesCommitted";
                });
                expect(ev1.length).to.be.equal(1);
                const ev1Data = ev1[0].event.data[0].toJSON();

                // logHex == 0x00 + ev1Data
                // Example:
                // logHex: 0x0064cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                // ev1Data:  0x64cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                const prefixedEv1Data = `0x00${ev1Data.slice(2)}`;
                expect(prefixedEv1Data).to.be.equal(logHex);
            },
        });

        it({
            id: "E02",
            title: "receive native token from Ethereum",
            test: async () => {
                const transferAmount = BigInt(10_000);

                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", {
                    name: "Alice default",
                });

                // Create token receiver account and send some balance to it
                const tokenReceiver = encodeAddress(
                    "0x0505050505050505050505050505050505050505050505050505050505050505"
                );

                let signedTx = await polkadotJs.tx.balances
                    .transferKeepAlive(tokenReceiver, 100_000_000_000_000_000n)
                    .signAsync(alice);

                await context.createBlock([signedTx], { allowFailures: false });

                // Ethereum sovereign account: send some balance to it
                signedTx = await polkadotJs.tx.balances
                    .transferKeepAlive(SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, 100_000_000_000_000_000n)
                    .signAsync(alice);

                await context.createBlock([signedTx], { allowFailures: false });

                // Constant log
                const log = polkadotJs.createType<SnowbridgeCoreInboundLog>("SnowbridgeCoreInboundLog", {
                    address: "0xeda338e4dc46038493b885327842fd3e301cab39",
                    topics: [
                        "0x7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f",
                        "0x0000000000000000000000000000000000000000000000000000000000000004",
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ],
                    data: "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006b00010000000000000002bcd4282ca0c30cbd9c578b5c790e88c803d80cd9cc91f28686f24ac25a61e06e00050505050505050505050505050505050505050505050505050505050505050510270000000000000000000000000000e8030000000000000000000000000000000000000000000000000000000000000000000000",
                });
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                // Create EthereumTokenTransfers channel to validate when receiving the tokens
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                // Create token on EthereumSystem to be validated when receiving the tokens
                const tokenLocation: MultiLocation = {
                    parents: 0,
                    interior: "Here",
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "dance",
                    symbol: "dance",
                    decimals: 12,
                };

                // Register token on EthereumSystem.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                // Sovereign balance before
                const {
                    data: { free: sovereignBalanceBefore },
                } = await context.polkadotJs().query.system.account(SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS);

                // Bob balance before
                const {
                    data: { free: bobBalanceBefore },
                } = await context.polkadotJs().query.system.account(tokenReceiver);

                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check balances were updated correctly.
                const {
                    data: { free: sovereignBalanceAfter },
                } = await context.polkadotJs().query.system.account(SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS);

                const {
                    data: { free: bobBalanceAfter },
                } = await context.polkadotJs().query.system.account(tokenReceiver);

                expect(sovereignBalanceAfter.toBigInt()).to.be.eq(sovereignBalanceBefore.toBigInt() - transferAmount);

                expect(bobBalanceAfter.toBigInt()).to.be.eq(bobBalanceBefore.toBigInt() + transferAmount);
            },
        });
    },
});

async function setFinalizedBeaconState(context: DevModeContext, slot: number, sudoAccount) {
    const module = xxhashAsU8a(new TextEncoder().encode("EthereumBeaconClient"), 128);
    const method = xxhashAsU8a(new TextEncoder().encode("FinalizedBeaconState"), 128);

    // This key is computed using the merkle tree of the execution_proof header
    const key = "0x72a1f6e510b05adf535bda49c8e7b05870c2c9dade912453797c40a9d20591a7";

    const compactBeaconState = {
        slot: slot,
        blockRootsRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
    };

    const value = context
        .polkadotJs()
        .createType("SnowbridgeBeaconPrimitivesCompactBeaconState", compactBeaconState)
        .toHex();

    const keyBytes = hexToU8a(key);

    const concatenatedKey = u8aToHex(new Uint8Array([...module, ...method, ...keyBytes]));

    const api = context.polkadotJs();

    await context.createBlock(
        api.tx.sudo.sudo(api.tx.system.setStorage([[concatenatedKey, value]])).signAsync(sudoAccount),
        { allowFailures: false }
    );
}
