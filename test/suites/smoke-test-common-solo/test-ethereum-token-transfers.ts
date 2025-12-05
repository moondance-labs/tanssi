// @ts-nocheck

import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { filterAndApply } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { EthereumTokenTransfersNativeTokenTransferred } from "@polkadot/types/lookup";
import { hexToBigInt, hexToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { Interface } from "ethers";
import {
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
    getBlockNumberForDebug,
    PRIMARY_GOVERNANCE_CHANNEL_ID,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
} from "utils";

const SS58_FORMAT = 42;

let BLOCKS_AMOUNT_TO_CHECK = 100;
const BLOCK_NUMBER_TO_DEBUG = getBlockNumberForDebug();

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

                            if (decodedEvent.payload.startsWith(MAGIC_BYTES)) {
                                // There was an error decoding as versionedXcmMessage, probably because the message
                                // was a validator update. in any case we will check that the nonce has increased
                                // This message is received in the primary channel
                                const previousNonce = await (
                                    await api.at(block.block.header.parentHash)
                                ).query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);
                                const currentNonce = await (await api.at(blockHash)).query.ethereumInboundQueue.nonce(
                                    PRIMARY_GOVERNANCE_CHANNEL_ID
                                );
                                expect(
                                    currentNonce.toBigInt(),
                                    `Block: ${blockNumber}. Current nonce ${currentNonce.toBigInt()} should be greater than the previous one ${previousNonce.toBigInt()}.`
                                ).to.be.equal(previousNonce.toBigInt() + 1n);
                                skip();
                            }

                            let versioned = null;
                            try {
                                versioned = api.registry.createType("VersionedXcmMessage", decodedEvent.payload);
                            } catch (e) {
                                throw new Error(
                                    `Unrecognized event payload for "ethereumInboundQueue.submit" for block #${blockNumber}. Details: ${decodedEvent.payload}. Decoder is missing.`
                                );
                            }

                            const messageV1 = versioned.toJSON().v1;
                            if (!messageV1 || !messageV1.command?.sendNativeToken) continue;

                            const { destination, amount } = versioned.toJSON().v1.command.sendNativeToken;

                            const relatedEvents = events.filter(
                                (e) => e.phase.isApplyExtrinsic && e.phase.asApplyExtrinsic.eq(index)
                            );

                            const matched = filterAndApply(
                                relatedEvents,
                                "balances",
                                ["Transfer"],
                                ({ event: { data } }) => {
                                    const [from, to, value] = data;
                                    return (
                                        from.toString() === sovereignAccount &&
                                        to.toString() === destination.accountId32 &&
                                        value.toString() === amount.toString()
                                    );
                                }
                            ).find(Boolean);

                            expect(
                                matched,
                                `Expected Transfer of ${amount.toString()} from sovereign to ${destination.accountId32} in block ${blockNumber}`
                            ).toBe(true);
                        }
                    }
                }
            },
        });

        it({
            id: "C04",
            title: "foreignAsset is issued when ERC20 token is received",
            test: async ({ skip }) => {
                // Find blocks where ethereum inbound queue receives a message, if the message is a SendToken message,
                // then foreignAsset should be issued and the amount should be minted to the destination address

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

                            if (decodedEvent.payload.startsWith(MAGIC_BYTES)) {
                                const previousNonce = await (
                                    await api.at(block.block.header.parentHash)
                                ).query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);
                                const currentNonce = await (await api.at(blockHash)).query.ethereumInboundQueue.nonce(
                                    PRIMARY_GOVERNANCE_CHANNEL_ID
                                );
                                expect(
                                    currentNonce.toBigInt(),
                                    `Block: ${blockNumber}. Current nonce ${currentNonce.toBigInt()} should be greater than the previous one ${previousNonce.toBigInt()}.`
                                ).to.be.equal(previousNonce.toBigInt() + 1n);
                                skip();
                            }

                            let versioned = null;
                            try {
                                versioned = api.registry.createType("VersionedXcmMessage", decodedEvent.payload);
                            } catch (e) {
                                throw new Error(
                                    `Unrecognized event payload for "ethereumInboundQueue.submit" for block #${blockNumber}. Details: ${decodedEvent.payload}. Decoder is missing.`
                                );
                            }

                            const messageV1 = versioned.toJSON().v1;
                            if (!messageV1 || !messageV1.command?.sendToken) continue;

                            const { destination, amount } = messageV1.command.sendToken;

                            const matched = filterAndApply(
                                events,
                                "foreignAssets",
                                ["Issued"],
                                ({ event: { data } }) => {
                                    const [_, owner, issuedAmount] = data;
                                    return (
                                        owner.toString() === destination.accountId32.toString() &&
                                        issuedAmount.toString() === hexToBigInt(amount).toString()
                                    );
                                }
                            ).find(Boolean);

                            expect(
                                matched,
                                `Expected foreignAssets.Issued of ${hexToBigInt(amount).toString()} to ${destination.accountId32} in block ${blockNumber}`
                            ).toBe(true);
                        }
                    }
                }
            },
        });

        it({
            id: "C05",
            title: "foreignAsset is burned when ERC20token is sent",
            test: async () => {
                // Find blocks where xcm transfer assets is called transfering assets to ethereum, then check that
                // the asset is burned from the account that signed the call
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

                        if (section === "xcmPallet" && method === "transferAssets") {
                            const signer = extrinsic.signer.toString();

                            const [dest, _, assets] = extrinsic.args;

                            const jsonDest = dest.toJSON();
                            const versionKey = Object.keys(jsonDest)[0]; // e.g., "v3"
                            const location = jsonDest[versionKey];

                            // Is not a message to ethereum
                            if (!location?.interior?.x1?.globalConsensus?.ethereum) continue;

                            const jsonAssets = assets.toJSON();
                            const assetsArray = jsonAssets[versionKey];

                            const relatedEvents = events.filter(
                                (e) => e.phase.isApplyExtrinsic && e.phase.asApplyExtrinsic.eq(index)
                            );

                            for (const asset of assetsArray) {
                                const amount = asset.fun.fungible;

                                const matched = filterAndApply(
                                    relatedEvents,
                                    "foreignAssets",
                                    ["Burned"],
                                    ({ event: { data } }) => {
                                        const [_, owner, burnedAmount] = data;
                                        return (
                                            owner.toString() === signer.toString() &&
                                            burnedAmount.toString() === hexToBigInt(amount).toString()
                                        );
                                    }
                                ).find(Boolean);

                                expect(
                                    matched,
                                    `Expected foreignAssets.Burned of ${amount.toString()} from ${signer} in block ${blockNumber}`
                                ).toBe(true);
                            }
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
