import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import {
    signAndSendAndInclude,
    sovereignAccountOfChildForAddress32,
    waitSessions,
    XcmFragment,
    generateLayerZeroOutboundLog,
    generateUpdate,
} from "utils";
import { hexToU8a } from "@polkadot/util";
import { retrieveDispatchErrors, findEventInRecentBlocks } from "helpers";

describeSuite({
    id: "ZOMBIETANSSILZ01",
    title: "LayerZero Message Forwarding from Relay to Container Chain",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;
        let aliceContainer: KeyringPair;

        beforeAll(async () => {
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
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

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
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

                const container2001BlockNum = (
                    await container2001Api.rpc.chain.getBlock()
                ).block.header.number.toNumber();
                expect(container2001BlockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Verify container chains have LzReceiverExample pallet",
            test: async () => {
                // Check simple template (2000)
                const container2000Metadata = await container2000Api.rpc.state.getMetadata();
                const lzReceiver2000 = container2000Metadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "LzReceiverExample"
                );
                expect(lzReceiver2000, "LzReceiverExample not found in Container2000").to.not.be.undefined;
                console.log(`Container2000 LzReceiverExample pallet index: ${lzReceiver2000.index.toNumber()}`);

                // Check frontier template (2001)
                const container2001Metadata = await container2001Api.rpc.state.getMetadata();
                const lzReceiver2001 = container2001Metadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "LzReceiverExample"
                );
                expect(lzReceiver2001, "LzReceiverExample not found in Container2001").to.not.be.undefined;
                console.log(`Container2001 LzReceiverExample pallet index: ${lzReceiver2001.index.toNumber()}`);
            },
        });

        it({
            id: "T03",
            title: "Verify LzRouter pallet exists on relay chain",
            test: async () => {
                const relayMetadata = await relayChainPolkadotJs.rpc.state.getMetadata();
                const lzRouter = relayMetadata.asLatest.pallets.find(({ name }) => name.toString() === "LzRouter");
                expect(lzRouter, "LzRouter pallet not found on relay").to.not.be.undefined;
                console.log(`LzRouter pallet index: ${lzRouter.index.toNumber()}`);

                // Check if storage for RoutingConfigs exists
                const storageEntries = lzRouter.storage?.unwrap().items || [];
                const hasConfigStorage = storageEntries.some((item) => item.name.toString() === "RoutingConfigs");
                expect(hasConfigStorage, "RoutingConfigs storage not found").to.be.true;
            },
        });

        it({
            id: "T04",
            title: "Verify EthereumInboundQueueV2 pallet exists on relay chain",
            test: async () => {
                const relayMetadata = await relayChainPolkadotJs.rpc.state.getMetadata();
                const inboundQueue = relayMetadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "EthereumInboundQueueV2"
                );
                expect(inboundQueue, "EthereumInboundQueueV2 pallet not found on relay").to.not.be.undefined;
                console.log(`EthereumInboundQueueV2 pallet index: ${inboundQueue.index.toNumber()}`);
            },
        });

        it({
            id: "T05",
            title: "Set up LayerZero forwarding config for container 2000 via XCM",
            timeout: 300000,
            test: async () => {
                // Get the pallet index for LzReceiverExample on the container chain
                const containerMetadata = await container2000Api.rpc.state.getMetadata();
                const lzReceiverPallet = containerMetadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "LzReceiverExample"
                );
                expect(lzReceiverPallet, "LzReceiverExample pallet not found").to.not.be.undefined;
                const palletIndex = lzReceiverPallet.index.toNumber();
                const callIndex = 0; // receive_message is call_index 0

                console.log(`LzReceiverExample pallet index: ${palletIndex}, call index: ${callIndex}`);

                // LayerZero sender to whitelist
                const lzSourceAddress = hexToU8a("0x0000000000000000000000001234567890abcdef1234567890abcdef12345678");
                const lzSourceEndpoint = 30101; // Ethereum mainnet LayerZero endpoint ID

                // Build the call to execute on the relay chain
                const routingConfigCall = relayChainPolkadotJs.tx.lzRouter.updateRoutingConfig({
                    notificationDestination: [palletIndex, callIndex],
                    whitelistedSenders: [[lzSourceEndpoint, Array.from(lzSourceAddress)]],
                });

                const encodedCall = routingConfigCall.method.toHex();
                console.log(`Encoded call: ${encodedCall}`);

                // Build XCM message to send from container chain to relay chain
                // The message will:
                // 1. Withdraw assets from sovereign account
                // 2. Buy execution
                // 3. Transact (execute the call)
                // 4. Refund surplus
                // 5. Deposit remaining assets back

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

                // First, fund the container chain's sovereign account on the relay
                const sovereignAccount = sovereignAccountOfChildForAddress32(context, 2000); // para + 2000
                const fundAmount = 100_000_000_000_000_000n; // 100 tokens

                console.log(`Funding sovereign account: ${sovereignAccount}`);
                await signAndSendAndInclude(
                    relayChainPolkadotJs.tx.balances.transferAllowDeath(sovereignAccount, fundAmount),
                    aliceRelay
                );

                // Wait a bit for the transfer to be finalized
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                // Check sovereign account balance
                const balance = await relayChainPolkadotJs.query.system.account(sovereignAccount);
                console.log(`Sovereign account balance: ${balance.data.free.toString()}`);
                expect(balance.data.free.toBigInt()).to.be.greaterThan(0n);

                // Send the XCM message from container chain to relay
                console.log("Sending XCM message from container chain to relay...");
                const xcmTx = container2000Api.tx.polkadotXcm.send(dest, xcmMessage);
                const sudoTx = container2000Api.tx.sudo.sudo(xcmTx);
                await signAndSendAndInclude(sudoTx, aliceContainer);

                // Wait for XCM to be processed on relay
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                // Verify the forwarding config was set
                const storedConfig = await relayChainPolkadotJs.query.lzRouter.routingConfigs(2000);
                console.log(`Stored config: ${JSON.stringify(storedConfig.toHuman())}`);

                expect(storedConfig.isSome, "Forwarding config should be set").to.be.true;
                const config = storedConfig.unwrap();
                expect(config.notificationDestination[0].toNumber()).to.equal(palletIndex);
                expect(config.notificationDestination[1].toNumber()).to.equal(callIndex);
            },
        });

        it({
            id: "T06",
            title: "Submit LayerZero message from Ethereum and verify processing on relay",
            timeout: 300000,
            test: async () => {
                // LayerZero sender (same as whitelisted in T05)
                const lzSourceAddress = hexToU8a("0x0000000000000000000000001234567890abcdef1234567890abcdef12345678");
                const lzSourceEndpoint = 30101;
                const destinationChain = 2000;

                // The message payload to send
                const message = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0xde, 0xad, 0xbe, 0xef]);

                // Use a unique nonce based on timestamp to avoid InvalidNonce errors
                // The nonce is tracked in a sparse bitmap and each can only be used once
                const nonce = Date.now();
                console.log(`Using nonce: ${nonce}`);

                console.log("Generating LayerZero outbound log...");
                const log = await generateLayerZeroOutboundLog(relayChainPolkadotJs, nonce, {
                    lzSourceAddress,
                    lzSourceEndpoint,
                    destinationChain,
                    message,
                });

                console.log("Generating beacon update...");
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(relayChainPolkadotJs, [log]);

                // Submit the checkpoint via sudo
                console.log("Submitting force checkpoint...");
                const checkpointTx = relayChainPolkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const sudoCheckpointTx = relayChainPolkadotJs.tx.sudo.sudo(checkpointTx);
                await signAndSendAndInclude(sudoCheckpointTx, aliceRelay);

                // Wait for checkpoint to be processed
                await context.waitBlock(1, "Tanssi-relay");

                // Submit the LayerZero message
                console.log("Submitting LayerZero message to inbound queue...");
                const submitTx = relayChainPolkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]);
                const { blockHash } = await signAndSendAndInclude(submitTx, aliceRelay);

                const relayChainApiForInboundQueue = await relayChainPolkadotJs.at(blockHash);

                const dispatchErrors = await retrieveDispatchErrors(relayChainApiForInboundQueue);
                console.log(`Dispatch errors: ${dispatchErrors}`);
                // expect the dispatch errors to be empty
                expect(dispatchErrors).to.be.empty;

                // Check for MessageReceived event on relay chain
                const relayEvents = await relayChainApiForInboundQueue.query.system.events();

                const messageReceivedEvent = relayEvents.find((a) => {
                    return a.event.section === "ethereumInboundQueueV2" && a.event.method === "MessageReceived";
                });
                expect(!!messageReceivedEvent, "MessageReceived event should exist on relay").to.equal(true);
                console.log("LayerZero message received on relay chain!");

                // Check for InboundMessageRouted event
                const routedEvent = relayEvents.find((a) => {
                    return a.event.section === "lzRouter" && a.event.method === "InboundMessageRouted";
                });
                expect(!!routedEvent, "InboundMessageRouted event should exist on relay").to.equal(true);
                console.log("LayerZero inbound message routed on relay chain!");
            },
        });

        it({
            id: "T07",
            title: "Verify container chain receives the LayerZero notification",
            timeout: 300000,
            test: async () => {
                // Wait for DMP to deliver the message to container chain
                console.log("Waiting for DMP message to be delivered to container chain...");
                await waitSessions(context, relayChainPolkadotJs, 2, null, "Tanssi-relay");

                // Check container chain for the MessageReceived event from LzReceiverExample
                const lzEventResult = await findEventInRecentBlocks(
                    container2000Api,
                    (record) =>
                        record.event.section === "lzReceiverExample" && record.event.method === "MessageReceived",
                    20
                );

                expect(lzEventResult, "LzReceiverExample.MessageReceived event should exist on container chain").to.not
                    .be.null;
                console.log(`Found MessageReceived event at block ${lzEventResult.blockNum}`);
                console.log(`Event data: ${JSON.stringify(lzEventResult.event.event.toHuman())}`);
                console.log("LayerZero message successfully forwarded to container chain!");
            },
        });
    },
});
