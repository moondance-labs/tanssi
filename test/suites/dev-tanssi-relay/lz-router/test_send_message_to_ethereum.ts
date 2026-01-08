import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { sendCallAsChildPara, sovereignAccountOfChildForAddress32, SNOWBRIDGE_FEES_ACCOUNT } from "utils";
import type { KeyringPair } from "@moonwall/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2, findEventInBlockRange } from "../../../helpers";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "LZROUTER01",
    title: "LzRouter - Send message to Ethereum tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let shouldSkipStarlightSnV2: boolean;
        let specVersion: number;

        const paraId = 2000; // Container chain ID
        const fundAmount = 100_000_000_000_000_000n; // 100 tokens

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSnV2 =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);

            if (shouldSkipStarlightSnV2) {
                console.log("Skipping tests for Starlight runtime - Snowbridge V2 not available");
                return;
            }

            // Fund the sovereign account of para 2000 on the relay chain
            const paraSovereignAccount = sovereignAccountOfChildForAddress32(context, paraId);
            const fundTx = polkadotJs.tx.balances.transferAllowDeath(paraSovereignAccount, fundAmount);
            await context.createBlock(await fundTx.signAsync(alice), { allowFailures: false });

            console.log(`Funded para ${paraId} sovereign account: ${paraSovereignAccount}`);
        });

        it({
            id: "E01",
            title: "send_message_to_ethereum should queue message to Ethereum",
            test: async () => {
                if (shouldSkipStarlightSnV2) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const paraSovereignAccount = sovereignAccountOfChildForAddress32(context, paraId);

                // Test parameters
                const lzDestinationAddress = hexToU8a("0xabcdef1234567890abcdef1234567890abcdef12");
                const lzDestinationEndpoint = 30110; // Arbitrum endpoint
                const payload = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"
                const reward = 1_000_000_000_000n; // 1 token
                const gas = 500_000n;

                // Get balances before
                const outboundNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const feesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();
                const sovereignAccountBalanceBefore = (
                    await polkadotJs.query.system.account(paraSovereignAccount)
                ).data.free.toBigInt();

                // Build the call to send message to Ethereum
                const sendMessageCall = polkadotJs.tx.lzRouter.sendMessageToEthereum(
                    Array.from(lzDestinationAddress),
                    lzDestinationEndpoint,
                    Array.from(payload),
                    reward,
                    gas
                );

                const blockNumber1 = (await polkadotJs.query.system.number()).toNumber();

                // Send XCM from para 2000 to call send_message_to_ethereum
                await sendCallAsChildPara(sendMessageCall, paraId, context, fundAmount / 100n);

                // Wait for message to be processed
                await context.createBlock();

                const blockNumber2 = (await polkadotJs.query.system.number()).toNumber();

                // Verify nonce increased (message was queued)
                const outboundNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                expect(outboundNonceAfter.toNumber()).to.be.equal(outboundNonceBefore.toNumber() + 1);

                // Verify fee was transferred from sovereign account to fees account
                const feesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();
                const sovereignAccountBalanceAfter = (
                    await polkadotJs.query.system.account(paraSovereignAccount)
                ).data.free.toBigInt();

                expect(feesAccountBalanceAfter).to.be.equal(feesAccountBalanceBefore + reward);
                expect(sovereignAccountBalanceAfter).to.be.lessThan(sovereignAccountBalanceBefore - reward);

                // 1. Check messageQueue.Processed event (UMP from container chain with our paraId)
                const processedResult = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => {
                        if (record.event.section === "messageQueue" && record.event.method === "Processed") {
                            const data: any = record.event.data.toJSON();
                            // Check it's UMP from our parachain and succeeded
                            return data[1]?.ump?.para === paraId && data[3] === true;
                        }
                        return false;
                    }
                );
                expect(processedResult, "messageQueue.Processed event for our para should exist").to.not.be.null;
                console.log(`messageQueue.Processed for para ${paraId} at block ${processedResult.blockNum}`);

                // 2. Check lzRouter.OutboundMessageSent event (with correct source chain and endpoint)
                const { event: outboundEvent } = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => {
                        if (record.event.section === "lzRouter" && record.event.method === "OutboundMessageSent") {
                            const data: any = record.event.data.toJSON();
                            return (
                                data[1].sourceChain === paraId &&
                                data[1].lzDestinationEndpoint === lzDestinationEndpoint
                            );
                        }
                        return false;
                    }
                );
                expect(outboundEvent, "OutboundMessageSent event should exist").to.not.be.undefined;

                const outboundData: any = outboundEvent.event.data.toJSON();
                expect(outboundData[2].toString()).to.equal(reward.toString());
                expect(outboundData[3].toString()).to.equal(gas.toString());

                // 3. Check ethereumOutboundQueueV2.MessageQueued event (with CallContract and matching gas)
                const { event: messageQueuedEvent } = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => {
                        if (
                            record.event.section === "ethereumOutboundQueueV2" &&
                            record.event.method === "MessageQueued"
                        ) {
                            const data: any = record.event.data.toJSON();
                            const commands = data[0]?.commands;
                            return (
                                commands &&
                                commands.length > 0 &&
                                commands[0].callContract &&
                                commands[0].callContract.gas.toString() === gas.toString()
                            );
                        }
                        return false;
                    }
                );
                expect(messageQueuedEvent, "MessageQueued event should exist").to.not.be.undefined;

                const messageQueuedData: any = messageQueuedEvent.event.data.toJSON();
                const messageId = messageQueuedData[0].id;
                console.log(`Message ID: ${messageId}`);

                // 4. Check ethereumOutboundQueueV2.MessageAccepted event (matching our message ID)
                const { event: messageAcceptedEvent } = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => {
                        if (
                            record.event.section === "ethereumOutboundQueueV2" &&
                            record.event.method === "MessageAccepted"
                        ) {
                            const data: any = record.event.data.toJSON();
                            return data[0] === messageId;
                        }
                        return false;
                    }
                );
                expect(messageAcceptedEvent, "MessageAccepted event should exist").to.not.be.undefined;

                console.log("LayerZero message successfully queued to Ethereum!");
            },
        });

        it({
            id: "E02",
            title: "send_message_to_ethereum fails with insufficient balance",
            test: async () => {
                if (shouldSkipStarlightSnV2) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                // In dev mode, UMP messages always come from the scheduled para (2000)
                // Drain para 2000's sovereign account to test insufficient balance
                const testParaId = paraId; // para 2000
                const paraSovereignAccount = sovereignAccountOfChildForAddress32(context, testParaId);

                // Get current balance
                const currentBalance = (await polkadotJs.query.system.account(paraSovereignAccount)).data.free.toBigInt();
                console.log(`Para ${testParaId} sovereign account balance before drain: ${currentBalance}`);

                // Drain most of the balance, leaving only existential deposit + small amount
                const existentialDeposit = polkadotJs.consts.balances.existentialDeposit.toBigInt();
                const amountToKeep = existentialDeposit + 100n; // Keep just above ED
                const amountToDrain = currentBalance - amountToKeep;

                if (amountToDrain > 0n) {
                    const drainTx = polkadotJs.tx.balances.transferKeepAlive(alice.address, amountToDrain);
                    await context.createBlock(await drainTx.signAsync(alice), { allowFailures: false });
                    console.log(`Drained ${amountToDrain} from sovereign account`);
                }

                // Verify balance is now minimal
                const balanceAfterDrain = (await polkadotJs.query.system.account(paraSovereignAccount)).data.free.toBigInt();
                console.log(`Para ${testParaId} sovereign account balance after drain: ${balanceAfterDrain}`);

                const lzDestinationAddress = hexToU8a("0xabcdef1234567890abcdef1234567890abcdef12");
                const lzDestinationEndpoint = 30110;
                const payload = new Uint8Array([0x01, 0x02, 0x03]);
                // Set reward higher than remaining balance to ensure failure
                const reward = balanceAfterDrain + 1_000_000n;
                const gas = 500_000n;

                console.log(`Attempting to send with reward ${reward}, but balance is only ${balanceAfterDrain}`);
                expect(reward).to.be.greaterThan(balanceAfterDrain);

                const sendMessageCall = polkadotJs.tx.lzRouter.sendMessageToEthereum(
                    Array.from(lzDestinationAddress),
                    lzDestinationEndpoint,
                    Array.from(payload),
                    reward,
                    gas
                );

                const blockNumber1 = (await polkadotJs.query.system.number()).toNumber();

                // This should fail because the sovereign account has insufficient balance
                await sendCallAsChildPara(sendMessageCall, testParaId, context, fundAmount / 100n);

                await context.createBlock();

                const blockNumber2 = (await polkadotJs.query.system.number()).toNumber();

                // Check that no OutboundMessageSent event was emitted (from our para)
                const outboundEventResult = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => {
                        if (record.event.section === "lzRouter" && record.event.method === "OutboundMessageSent") {
                            const data: any = record.event.data.toJSON();
                            return data[1].sourceChain === testParaId;
                        }
                        return false;
                    }
                );
                expect(outboundEventResult, "OutboundMessageSent should not be emitted").to.be.null;

                console.log("Message correctly failed due to insufficient balance");

                // Restore balance for subsequent tests
                const restoreTx = polkadotJs.tx.balances.transferAllowDeath(paraSovereignAccount, fundAmount);
                await context.createBlock(await restoreTx.signAsync(alice), { allowFailures: false });
                console.log(`Restored sovereign account balance for subsequent tests`);
            },
        });

        it({
            id: "E03",
            title: "send_message_to_ethereum with large payload (8KB)",
            test: async () => {
                if (shouldSkipStarlightSnV2) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const lzDestinationAddress = hexToU8a("0x2222222222222222222222222222222222222222");
                const lzDestinationEndpoint = 30168; // Solana endpoint
                const largePayload = new Uint8Array(8 * 1024 - 100); // Nearly 8KB (leaving room for encoding)
                largePayload.fill(0xff);
                const reward = 1_500_000_000_000n;
                const gas = 800_000n;

                const outboundNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const blockNumber1 = (await polkadotJs.query.system.number()).toNumber();

                const sendMessageCall = polkadotJs.tx.lzRouter.sendMessageToEthereum(
                    Array.from(lzDestinationAddress),
                    lzDestinationEndpoint,
                    Array.from(largePayload),
                    reward,
                    gas
                );

                await sendCallAsChildPara(sendMessageCall, paraId, context, fundAmount / 100n);
                await context.createBlock();

                const blockNumber2 = (await polkadotJs.query.system.number()).toNumber();
                const outboundNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                expect(outboundNonceAfter.toNumber()).to.be.equal(outboundNonceBefore.toNumber() + 1);

                // Verify event
                const outboundEventResult = await findEventInBlockRange(
                    polkadotJs,
                    blockNumber1,
                    blockNumber2,
                    (record) => record.event.section === "lzRouter" && record.event.method === "OutboundMessageSent"
                );
                expect(outboundEventResult, "OutboundMessageSent event should exist").to.not.be.null;

                console.log(
                    `Large payload (${largePayload.length} bytes) sent successfully at block ${outboundEventResult.blockNum}!`
                );
            },
        });
    },
});
