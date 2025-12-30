// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { signAndSendAndInclude, sovereignAccountOfChildForAddress32, waitSessions } from "utils";
import { hexToU8a } from "@polkadot/util";

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

                const container2000BlockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(container2000BlockNum).to.be.greaterThan(0);

                const container2001BlockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
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
            title: "Verify LayerZero forwarder pallet exists on relay chain",
            test: async () => {
                const relayMetadata = await relayChainPolkadotJs.rpc.state.getMetadata();
                const layerZeroForwarder = relayMetadata.asLatest.pallets.find(
                    ({ name }) => name.toString() === "LayerZeroForwarder"
                );
                expect(layerZeroForwarder, "LayerZeroForwarder pallet not found on relay").to.not.be.undefined;
                console.log(`LayerZeroForwarder pallet index: ${layerZeroForwarder.index.toNumber()}`);

                // Check if storage for MessageForwardingConfigs exists
                const storageEntries = layerZeroForwarder.storage?.unwrap().items || [];
                const hasConfigStorage = storageEntries.some(
                    (item) => item.name.toString() === "MessageForwardingConfigs"
                );
                expect(hasConfigStorage, "MessageForwardingConfigs storage not found").to.be.true;
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
                const forwardingConfigCall = relayChainPolkadotJs.tx.layerZeroForwarder.updateMessageForwardingConfig({
                    notificationDestination: [palletIndex, callIndex],
                    whitelistedSenders: [[lzSourceEndpoint, Array.from(lzSourceAddress)]],
                });

                const encodedCall = forwardingConfigCall.method.toHex();
                console.log(`Encoded call: ${encodedCall}`);

                // Build XCM message to send from container chain to relay chain
                // The message will:
                // 1. Withdraw assets from sovereign account
                // 2. Buy execution
                // 3. Transact (execute the call)
                // 4. Refund surplus
                // 5. Deposit remaining assets back

                const xcmMessage = {
                    V4: [
                        {
                            WithdrawAsset: [
                                {
                                    id: { parents: 0, interior: "Here" },
                                    fun: { Fungible: 10_000_000_000_000n }, // 10 tokens
                                },
                            ],
                        },
                        {
                            BuyExecution: {
                                fees: {
                                    id: { parents: 0, interior: "Here" },
                                    fun: { Fungible: 10_000_000_000_000n },
                                },
                                weightLimit: "Unlimited",
                            },
                        },
                        {
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
                        },
                        "RefundSurplus",
                        {
                            DepositAsset: {
                                assets: { Wild: "All" },
                                beneficiary: {
                                    parents: 0,
                                    interior: {
                                        X1: [{ Parachain: 2000 }],
                                    },
                                },
                            },
                        },
                    ],
                };

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
                const storedConfig = await relayChainPolkadotJs.query.layerZeroForwarder.messageForwardingConfigs(2000);
                console.log(`Stored config: ${JSON.stringify(storedConfig.toHuman())}`);

                expect(storedConfig.isSome, "Forwarding config should be set").to.be.true;
                const config = storedConfig.unwrap();
                expect(config.notificationDestination[0].toNumber()).to.equal(palletIndex);
                expect(config.notificationDestination[1].toNumber()).to.equal(callIndex);
            },
        });
    },
});
