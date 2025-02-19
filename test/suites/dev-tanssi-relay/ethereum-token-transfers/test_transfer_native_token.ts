import { type DevModeContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { encodeAddress, keccakAsHex, xxhashAsU8a } from "@polkadot/util-crypto";
import "@tanssi/api-augment";
import { readFileSync } from "node:fs";
import { expectEventCount } from "../../../helpers/events";
import type { MultiLocation } from "../../../util/xcm";

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
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", {
                    name: "Alice default",
                });

                const transferAmount = BigInt(10_000);

                // Create BOB account that will receive the tokens
                const bob = encodeAddress(
                    new Uint8Array([
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                    ])
                );

                let signedTx = await polkadotJs.tx.balances
                    .transferKeepAlive(bob, 100_000_000_000_000_000n)
                    .signAsync(alice);

                await context.createBlock([signedTx], { allowFailures: false });

                const sovereign = encodeAddress(
                    "0x34cdd3f84040fb44d70e83b892797846a8c0a556ce08cd470bf6d4cf7b94ff77",
                    0
                );

                signedTx = await polkadotJs.tx.balances
                    .transferKeepAlive(sovereign, 100_000_000_000_000_000n)
                    .signAsync(alice);

                await context.createBlock([signedTx], { allowFailures: false });

                const initialCheckpoint = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
                signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx]);

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

                const slot = 64;

                await getSupportedAssetStorageKey(context, slot, alice);

                const event_log = {
                    address: "0xeda338e4dc46038493b885327842fd3e301cab39",
                    topics: [
                        "0x7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f",
                        "0x0000000000000000000000000000000000000000000000000000000000000004",
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ],
                    data: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        107, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 98, 232, 243, 59, 127, 176, 231, 226, 210, 39, 101, 100, 6,
                        26, 47, 60, 123, 203, 97, 46, 115, 59, 139, 245, 115, 62, 161, 108, 238, 14, 203, 166, 0, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 16,
                        39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                };

                // This contains a struct with key / value where the key = 0 and value = rlp::encode(receipt)
                const receiptShortNode =
                    "0xf9026e80b9026af902670080b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f9015ff9015c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa00000000000000000000000000000000000000000000000000000000000000004a00000000000000000000000000000000000000000000000000000000000000000b8e000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006b0001000000000000000262e8f33b7fb0e7e2d2276564061a2f3c7bcb612e733b8bf5733ea16cee0ecba600050505050505050505050505050505050505050505050505050505050505050510270000000000000000000000000000e8030000000000000000000000000000000000000000000000000000000000000000000000";

                const proof = {
                    block_hash: "0x6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131",
                    tx_index: 0,
                    receipt_proof: [[], [receiptShortNode]],
                    execution_proof: {
                        header: {
                            slot: slot,
                            proposer_index: 4,
                            parent_root: "0x6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef",
                            state_root: "0xb62ac34a8cb82497be9542fe2114410c9f6021855b766015406101a1f3d86434",
                            body_root: "0x8904949e001b4e2946f71cee7b15b9d8aef24a7f4130ee94f527821376cdff4c",
                        },
                        execution_header: {
                            Deneb: {
                                parent_hash: "0x8092290aa21b7751576440f77edd02a94058429ce50e63a92d620951fb25eda2",
                                fee_recipient: "0x0000000000000000000000000000000000000000",
                                state_root: "0x96a83e9ddf745346fafcb0b03d57314623df669ed543c110662b21302a0fae8b",
                                receipts_root: keccakAsHex(receiptShortNode),
                                logs_bloom:
                                    "0x00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000400000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000080000000000000000000000000000040004000000000000002002002000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000080000000000000000000000000000000000100000000000000000200000200000010",
                                prev_randao: "0x62e309d4f5119d1f5c783abc20fc1a549efbab546d8d0b25ff1cfd58be524e67",
                                block_number: 393,
                                gas_limit: 54492273,
                                gas_used: 199644,
                                timestamp: 1710552813,
                                extra_data: "0xd983010d0b846765746888676f312e32312e368664617277696e",
                                base_fee_per_gas: 7,
                                block_hash: "0x6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131",
                                transactions_root: "0x2cfa6ed7327e8807c7973516c5c32a68ef2459e586e8067e113d081c3bd8c07d",
                                withdrawals_root: "0x792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535",
                                blob_gas_used: 0,
                                excess_blob_gas: 0,
                            },
                        },
                        execution_branch: [
                            "0xa6833fa629f3286b6916c6e50b8bf089fc9126bee6f64d0413b4e59c1265834d",
                            "0xb46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb",
                            "0xdb56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71",
                            "0xd3af7c05c516726be7505239e0b9c7cb53d24abce6b91cdb3b3995f0164a75da",
                        ],
                    },
                };

                const message = {
                    event_log,
                    proof,
                };

                // Sovereign balance before
                const {
                    data: { free: sovereignBalanceBefore },
                } = await context.polkadotJs().query.system.account(sovereign);

                // Bob balance before
                const {
                    data: { free: bobBalanceBefore },
                } = await context.polkadotJs().query.system.account(bob);

                // Receive token through ethereum inbound queue
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(message).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check balances were updated correctly.
                const {
                    data: { free: sovereignBalanceAfter },
                } = await context.polkadotJs().query.system.account(sovereign);

                const {
                    data: { free: bobBalanceAfter },
                } = await context.polkadotJs().query.system.account(bob);

                expect(sovereignBalanceAfter.toBigInt()).to.be.eq(sovereignBalanceBefore.toBigInt() - transferAmount);

                expect(bobBalanceAfter.toBigInt()).to.be.eq(bobBalanceBefore.toBigInt() + transferAmount);
            },
        });
    },
});

// TODO: change my name
async function getSupportedAssetStorageKey(context: DevModeContext, slot: number, sudoAccount) {
    const module = xxhashAsU8a(new TextEncoder().encode("EthereumBeaconClient"), 128);
    const method = xxhashAsU8a(new TextEncoder().encode("FinalizedBeaconState"), 128);
    const key = "0x61a5108e3fa264956cec30d42aaaf12d5db5ceeb4c008995703e8e81f32380e9";

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
