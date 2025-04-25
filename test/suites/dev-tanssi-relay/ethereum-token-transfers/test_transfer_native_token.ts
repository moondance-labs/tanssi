import "@tanssi/api-augment";

import { type DevModeContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { encodeAddress, xxhashAsU8a } from "@polkadot/util-crypto";
import { generateEventLog, generateUpdate, SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, type MultiLocation } from "utils";
import { expectEventCount } from "../../../helpers/events";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS, checkCallIsFiltered } from "helpers";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1702",
    title: "EthereumTokenTransfers tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS.includes(specVersion);
        });

        it({
            id: "E01",
            title: "transferNativeToken should send message to Ethereum",
            test: async () => {
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                // Set channel info on EthereumTokenTransfers pallet.
                const tx1 = polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                    newChannelId,
                    newAgentId,
                    newParaId
                );

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

                if (shouldSkipStarlightETT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx1.signAsync(alice));

                    // EthereumSystem call should be also filtered
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata).signAsync(alice)
                    );

                    // Token transfer call should be filtered as well
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.ethereumTokenTransfers.transferNativeToken(1000, "0x").signAsync(alice)
                    );
                    return;
                }

                const sudoSignedTx1 = await polkadotJs.tx.sudo.sudo(tx1).signAsync(alice);
                await context.createBlock([sudoSignedTx1], { allowFailures: false });

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
                if (shouldSkipStarlightETT) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);

                    // Check that inboundQueue.submit is filtered
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.ethereumInboundQueue.submit("0x").signAsync(alice)
                    );
                    return;
                }
                const transferAmount = BigInt(10_000);

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

                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    new Uint8Array([
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 188, 212, 40, 44, 160, 195, 12, 189, 156, 87, 139, 92, 121, 14,
                        136, 200, 3, 216, 12, 217, 204, 145, 242, 134, 134, 242, 74, 194, 90, 97, 224, 110, 0, 5, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 16, 39,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ])
                );
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
