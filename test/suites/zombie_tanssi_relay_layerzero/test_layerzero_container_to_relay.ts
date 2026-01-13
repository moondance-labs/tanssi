import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { hexToU8a } from "@polkadot/util";
import { findEventInRecentBlocks } from "helpers";
import { XcmFragment, signAndSendAndInclude, sovereignAccountOfChildForAddress32, waitSessions } from "utils";

describeSuite({
    id: "ZOMBIETANSSILZ02",
    title: "LayerZero Message Sending from Container Chain to Ethereum via Relay",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let container2000Api: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;
        let aliceContainer: KeyringPair;

        beforeAll(async () => {
            container2000Api = context.polkadotJs("Container2000");
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");

            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
            // Simple template uses sr25519
            aliceContainer = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

            // Verify connections
            const relayNetwork = relayChainPolkadotJs.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on relay and container chains",
            test: async () => {
                const relayBlockNum = (await relayChainPolkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(relayBlockNum).to.be.greaterThan(0);

                const container2000BlockNum = (
                    await container2000Api.rpc.chain.getBlock()
                ).block.header.number.toNumber();
                expect(container2000BlockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Verify LzRouter pallet exists on relay chain",
            test: async () => {
                const relayMetadata = await relayChainPolkadotJs.rpc.state.getMetadata();
                const lzRouter = relayMetadata.asLatest.pallets.find(({ name }) => name.toString() === "LzRouter");
                expect(lzRouter, "LzRouter pallet not found on relay").to.not.be.undefined;
                console.log(`LzRouter pallet index: ${lzRouter.index.toNumber()}`);
            },
        });

        it({
            id: "T03",
            title: "Verify EthereumOutboundQueueV2 pallet exists on relay chain",
            test: async () => {
                const relayMetadata = await relayChainPolkadotJs.rpc.state.getMetadata();
                const outboundQueue = relayMetadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "EthereumOutboundQueueV2"
                );
                expect(outboundQueue, "EthereumOutboundQueueV2 pallet not found on relay").to.not.be.undefined;
                console.log(`EthereumOutboundQueueV2 pallet index: ${outboundQueue.index.toNumber()}`);
            },
        });

        it({
            id: "T04",
            title: "Fund container chain sovereign account on relay",
            timeout: 300000,
            test: async () => {
                const sovereignAccount = sovereignAccountOfChildForAddress32(context, 2000);
                const fundAmount = 100_000_000_000_000_000n; // 100 tokens

                console.log(`Funding sovereign account: ${sovereignAccount}`);
                await signAndSendAndInclude(
                    relayChainPolkadotJs.tx.balances.transferAllowDeath(sovereignAccount, fundAmount),
                    aliceRelay
                );

                // Wait for the transfer to be finalized
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                // Check sovereign account balance
                const balance = await relayChainPolkadotJs.query.system.account(sovereignAccount);
                console.log(`Sovereign account balance: ${balance.data.free.toString()}`);
                expect(balance.data.free.toBigInt()).to.be.greaterThan(0n);
            },
        });

        it({
            id: "T05",
            title: "Send LayerZero message from container chain to Ethereum via XCM",
            timeout: 300000,
            test: async () => {
                // LayerZero destination parameters
                const lzDestinationAddress = hexToU8a("0xabcdef1234567890abcdef1234567890abcdef12");
                const lzDestinationEndpoint = 30110; // Arbitrum endpoint
                const payload = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"
                const reward = 1_000_000_000_000n; // 1 token
                const gas = 500_000n;

                // Build the call to send message to Ethereum via LzRouter
                const sendMessageCall = relayChainPolkadotJs.tx.lzRouter.sendMessageToEthereum(
                    Array.from(lzDestinationAddress),
                    lzDestinationEndpoint,
                    Array.from(payload),
                    reward,
                    gas
                );

                const encodedCall = sendMessageCall.method.toHex();
                console.log(`Encoded sendMessageToEthereum call: ${encodedCall}`);

                // Get outbound nonce before
                const outboundNonceBefore = await relayChainPolkadotJs.query.ethereumOutboundQueueV2.nonce();
                console.log(`Outbound nonce before: ${outboundNonceBefore.toNumber()}`);

                // Build XCM message to send from container chain to relay chain
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: { parents: 0, interior: { Here: null } },
                            fungible: 10_000_000_000_000n, // 10 tokens
                        },
                    ],
                })
                    .withdraw_asset()
                    .buy_execution()
                    .push_any({
                        Transact: {
                            originKind: "Xcm",
                            requireWeightAtMost: {
                                refTime: 1_000_000_000n,
                                proofSize: 100_000n,
                            },
                            call: {
                                encoded: encodedCall,
                            },
                        },
                    })
                    .refund_surplus()
                    .deposit_asset(1n, null, {
                        parents: 0,
                        interior: {
                            X1: [{ Parachain: 2000 }],
                        },
                    })
                    .as_v4();

                // Destination is the relay chain (parent)
                const dest = {
                    V4: {
                        parents: 1,
                        interior: "Here",
                    },
                };

                // Send the XCM message from container chain to relay
                console.log("Sending XCM message from container chain to relay...");
                const xcmTx = container2000Api.tx.polkadotXcm.send(dest, xcmMessage);
                const sudoTx = container2000Api.tx.sudo.sudo(xcmTx);
                await signAndSendAndInclude(sudoTx, aliceContainer);

                // Wait for XCM to be processed on relay
                await waitSessions(context, relayChainPolkadotJs, 2, null, "Tanssi-relay");

                // Check outbound nonce after
                const outboundNonceAfter = await relayChainPolkadotJs.query.ethereumOutboundQueueV2.nonce();
                console.log(`Outbound nonce after: ${outboundNonceAfter.toNumber()}`);
                expect(outboundNonceAfter.toNumber()).to.be.greaterThan(outboundNonceBefore.toNumber());

                // Check for OutboundMessageSent event on relay chain
                const outboundEventResult = await findEventInRecentBlocks(
                    relayChainPolkadotJs,
                    (record) => record.event.section === "lzRouter" && record.event.method === "OutboundMessageSent",
                    20
                );
                expect(outboundEventResult, "OutboundMessageSent event should exist on relay").to.not.be.null;
                console.log(`Found OutboundMessageSent event at block ${outboundEventResult.blockNum}`);
                console.log(`Event data: ${JSON.stringify(outboundEventResult.event.event.toHuman())}`);

                // Verify the message has correct source chain (2000)
                const eventData: any = outboundEventResult.event.event.toJSON();
                expect(eventData.data[1].sourceChain).to.equal(2000);
                expect(eventData.data[1].lzDestinationEndpoint).to.equal(lzDestinationEndpoint);

                console.log("LayerZero message successfully sent from container chain to Ethereum outbound queue!");
            },
        });

        it({
            id: "T06",
            title: "Verify MessageQueued event on EthereumOutboundQueueV2",
            timeout: 300000,
            test: async () => {
                // Check for MessageQueued event
                const messageQueuedResult = await findEventInRecentBlocks(
                    relayChainPolkadotJs,
                    (record) =>
                        record.event.section === "ethereumOutboundQueueV2" && record.event.method === "MessageQueued",
                    20
                );
                expect(messageQueuedResult, "MessageQueued event should exist on relay").to.not.be.null;
                console.log(`Found MessageQueued event at block ${messageQueuedResult.blockNum}`);

                const eventData: any = messageQueuedResult.event.event.toJSON();
                const commands = eventData.data[0]?.commands;
                expect(commands, "Commands should exist in MessageQueued event").to.not.be.undefined;
                expect(commands.length).to.be.greaterThan(0);
                expect(commands[0].callContract, "First command should be CallContract").to.not.be.undefined;

                console.log(`Message ID: ${eventData.data[0].id}`);
                console.log("Message successfully queued for Ethereum!");
            },
        });
    },
});
