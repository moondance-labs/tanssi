import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { EthereumTokenTransfersNativeTokenTransferred } from "@polkadot/types/lookup";
import { hexToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { Interface } from "ethers";
import { ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS, SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS } from "utils";

const SS58_FORMAT = 42;

let BLOCKS_AMOUNT_TO_CHECK = 100;
// For debug purposes only, specify block here to check it
const BLOCK_NUMBER_TO_DEBUG = undefined;

const customTypes = {
    VersionedXcmMessage: {
        _enum: {
            V1: "MessageV1",
        },
    },
    MessageV1: {
        chain_id: "u64",
        command: "Command",
    },
    Command: {
        _enum: {
            RegisterToken: "RegisterToken",
            SendToken: "SendToken",
            SendNativeToken: "SendNativeToken",
        },
    },
    RegisterToken: {
        token: "H160",
        fee: "u128",
    },
    SendToken: {
        token: "H160",
        destination: "Destination",
        amount: "u128",
        fee: "u128",
    },
    SendNativeToken: {
        token_id: "TokenId",
        destination: "Destination",
        amount: "u128",
        fee: "u128",
    },
    Destination: {
        _enum: {
            AccountId32: "AccountId",
        },
    },
    TokenId: "H256",
};
//https://github.com/moondance-labs/tanssi-bridge-relayer/blob/247bc96365c5f8a9cdbcf3fae09a8ede79ac4c91/overridden_contracts/src/libraries/OSubstrateTypes.sol#L41
const MAGIC_BYTES = "0x70150038";

describeSuite({
    id: "SMOK15",
    title: "Ethereum token transfers smoke tests",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let sovereignAccount: string;

        beforeAll(async () => {
            api = context.polkadotJs();

            api.registry.register(customTypes);

            const runtimeName = api.runtimeVersion.specName.toString();
            const isStarlight = runtimeName === "starlight";
            sovereignAccount = isStarlight
                ? ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS
                : SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;

            sovereignAccount = encodeAddress(hexToU8a(sovereignAccount), SS58_FORMAT);
        });

        it({
            id: "C01",
            title: "Token transfer channels exists",
            test: async () => {
                const channels = await api.query.ethereumSystem.channels.entries();
                expect(channels.length).toBeGreaterThan(0);
            },
        });

        it({
            id: "C02",
            title: "Sovereign account collects funds when native token is transferred",
            test: async () => {
                // Go through the last BLOCKS_AMOUNT_TO_CHECK blocks and check if the sovereign account is collecting
                // the amount for each native token transfer event.
                let currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();

                if (BLOCK_NUMBER_TO_DEBUG !== undefined) {
                    BLOCKS_AMOUNT_TO_CHECK = 1;
                    currentBlock = BLOCK_NUMBER_TO_DEBUG + 1;
                }

                for (let i = 1; i <= BLOCKS_AMOUNT_TO_CHECK; i++) {
                    const blockNumber = currentBlock - i;
                    process.stdout.write(`\rProcessing block [${blockNumber}]: ${i}/${BLOCKS_AMOUNT_TO_CHECK}`);

                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const apiAtBlock = await api.at(blockHash);
                    const events = await apiAtBlock.query.system.events();

                    const tokenTransferEvents = events.filter(({ event }) =>
                        api.events.ethereumTokenTransfers.NativeTokenTransferred.is(event)
                    );

                    const balanceTransferEvents = events.filter(
                        ({ event }) =>
                            api.events.balances?.Transfer?.is(event) || api.events.currencies?.Transferred?.is(event)
                    );

                    // Check sovereign is collecting the amount for each native token transfer event
                    for (const { event } of tokenTransferEvents) {
                        const nativeTokenTransferEvent = event.data as EthereumTokenTransfersNativeTokenTransferred;

                        const recipientReceived = balanceTransferEvents.some(({ event }) => {
                            const [from, to, amount] = event.data;
                            return (
                                from?.toString() === nativeTokenTransferEvent.source.toString() &&
                                to?.toString() === sovereignAccount &&
                                amount?.toString() === nativeTokenTransferEvent.amount.toString()
                            );
                        });

                        expect(
                            recipientReceived,
                            `Expected transfer to ${sovereignAccount} not found in block ${blockNumber}`
                        ).toBe(true);
                    }
                }
            },
        });

        it({
            id: "C03",
            title: "Sovereign account releases funds when token is received via ethereuminboundqueue.submit",
            test: async ({ skip }) => {
                let currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();

                if (BLOCK_NUMBER_TO_DEBUG !== undefined) {
                    BLOCKS_AMOUNT_TO_CHECK = 1;
                    currentBlock = BLOCK_NUMBER_TO_DEBUG + 1;
                }

                for (let i = 1; i <= BLOCKS_AMOUNT_TO_CHECK; i++) {
                    const blockNumber = currentBlock - i;
                    process.stdout.write(`\rProcessing block [${blockNumber}]: ${i}/${BLOCKS_AMOUNT_TO_CHECK}`);

                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const block = await api.rpc.chain.getBlock(blockHash);
                    const events = await api.query.system.events.at(blockHash);
                    const extrinsics = block.block.extrinsics;

                    for (const [index, extrinsic] of extrinsics.entries()) {
                        const { section, method } = extrinsic.method;

                        if (section === "ethereumInboundQueue" && method === "submit") {
                            const message = extrinsic.args[0];
                            const { eventLog } = message.toJSON();

                            const decodedEvent = iface.decodeEventLog(
                                "OutboundMessageAccepted",
                                eventLog.data,
                                eventLog.topics
                            );

                            let versioned;
                            if (decodedEvent.payload.startsWith(MAGIC_BYTES)) {
                                // There was an error decoding as versionedXcmMessage, probably because the message
                                // was a validator update. in any case we will check that the nonce has increased
                                // This message is received in the primary channel
                                const channelId = "0x0000000000000000000000000000000000000000000000000000000000000001";
                                const previousNonce = await (
                                    await api.at(block.block.header.parentHash)
                                ).query.ethereumInboundQueue.nonce(channelId);
                                const currentNonce = await (await api.at(blockHash)).query.ethereumInboundQueue.nonce(
                                    channelId
                                );
                                expect(currentNonce.toBigInt()).to.be.equal(previousNonce.toBigInt() + 1n);
                                skip();
                            } else {
                                versioned = api.registry.createType("VersionedXcmMessage", decodedEvent.payload);
                            }

                            const { destination, amount } = versioned.toJSON().v1.command.sendNativeToken;

                            const relatedEvents = events.filter(
                                (e) => e.phase.isApplyExtrinsic && e.phase.asApplyExtrinsic.eq(index)
                            );

                            const matched = relatedEvents.some(({ event }) => {
                                const { section, method, data } = event;
                                if (section !== "balances" || method !== "Transfer") return false;

                                const [from, to, value] = data;

                                return (
                                    from.toString() === sovereignAccount &&
                                    to.toString() === destination.accountId32 &&
                                    value.toString() === amount.toString()
                                );
                            });

                            expect(
                                matched,
                                `Expected Transfer of ${amount.toString()} from sovereign to ${destination.accountId32} in block ${blockNumber}`
                            ).toBe(true);
                        }
                    }
                }
            },
        });
    },
});

const iface = new Interface([
    "event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload)",
]);
