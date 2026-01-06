import bls from "@chainsafe/bls";
import { Tree } from "@chainsafe/persistent-merkle-tree";
import type { Log } from "@ethereumjs/evm";
import { Trie } from "@ethereumjs/trie";
import { TransactionType } from "@ethereumjs/tx";
import { type PostByzantiumTxReceipt, encodeReceipt } from "@ethereumjs/vm";
import type { SyncCommitteeFast } from "@lodestar/light-client";
import {
    BLOCK_BODY_EXECUTION_PAYLOAD_GINDEX,
    EPOCHS_PER_SYNC_COMMITTEE_PERIOD,
    SLOTS_PER_EPOCH,
    SYNC_COMMITTEE_SIZE,
} from "@lodestar/params";
import { type Slot, type altair, type deneb, ssz } from "@lodestar/types";
import type { ApiPromise } from "@polkadot/api";
import type { Bytes, Vec } from "@polkadot/types-codec";
import type { H256 } from "@polkadot/types/interfaces";
import type {
    SnowbridgeBeaconPrimitivesBeaconHeader,
    SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader,
    SnowbridgeBeaconPrimitivesExecutionProof,
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate,
    SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader,
    SnowbridgeVerificationPrimitivesEventProof,
    SnowbridgeVerificationPrimitivesLog,
    SnowbridgeVerificationPrimitivesProof,
} from "@polkadot/types/lookup";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { AbiCoder } from "ethers/abi";
import { getBytes } from "ethers/utils";

export const BLOCKS_ROOTS_ROOT_GINDEX_ELECTRA = 69;

export const CURRENT_SYNC_COMMITTEE_GINDEX_ELECTRA = 86;

function defaultBeaconBlockHeader(slot: Slot): deneb.LightClientHeader {
    const header = ssz.deneb.LightClientHeader.defaultValue();
    header.beacon.slot = slot;
    return header;
}

function encodeAsNativeTokenERC20(tokenAddress: string, value: bigint): Uint8Array {
    const defaultAbiCoder = AbiCoder.defaultAbiCoder();

    const encoded = defaultAbiCoder.encode(["address", "uint128"], [tokenAddress, value]);
    return getBytes(encoded);
}

function encodeAsForeignTokenERC20(tokenId: string, value: bigint): Uint8Array {
    const defaultAbiCoder = AbiCoder.defaultAbiCoder();

    const encoded = defaultAbiCoder.encode(["bytes32", "uint128"], [tokenId, value]);

    return getBytes(encoded);
}

function encodeXcmMessage(api: ApiPromise, xcmInstructions): Uint8Array {
    const xcm = api.createType("XcmVersionedXcm", {
        V5: xcmInstructions,
    });

    return xcm.toU8a();
}

function encodeSymbioticMessage(api: ApiPromise, symbioticValidators: string[]): Uint8Array {
    const MAGIC_BYTES = new Uint8Array([112, 21, 0, 56]);

    api.registry.register({
        SymbioticReceiveValidators: {
            validators: "Vec<AccountId32>",
            external_index: "u64",
        },
        SymbioticInboundCommand: {
            _enum: {
                ReceiveValidators: "SymbioticReceiveValidators",
            },
        },
        SymbioticMessage: {
            _enum: {
                V1: "SymbioticInboundCommand",
            },
        },
        SymbioticPayload: {
            magic_bytes: "[u8; 4]",
            message: "SymbioticMessage",
        },
    });

    const validators = symbioticValidators.map((validator) => {
        const hexStr = validator.startsWith("0x") ? validator.slice(2) : validator;
        return hexStr;
    });

    const payload = api.createType("SymbioticPayload", {
        magic_bytes: Array.from(MAGIC_BYTES),
        message: {
            V1: {
                ReceiveValidators: {
                    validators: validators,
                    external_index: 0,
                },
            },
        },
    });

    return payload.toU8a();
}

function encodeLayerZeroMessage(
    lzSourceAddress: Uint8Array,
    lzSourceEndpoint: number,
    destinationChain: number,
    message: Uint8Array
): Uint8Array {
    if (lzSourceAddress.length !== 32) {
        throw new Error(
            `lzSourceAddress must be exactly 32 bytes, got ${lzSourceAddress.length} bytes`
        );
    }

    const MAGIC_BYTES = new Uint8Array([0x6c, 0x7a, 0x62, 0x31]); // "lzb1"
    const defaultAbiCoder = AbiCoder.defaultAbiCoder();

    // Encode as InboundSolPayload struct: tuple(bytes4 magicBytes, tuple(bytes32,uint32,uint32,bytes) message)
    // The nested tuple is InboundSolMessage: (lzSourceAddress, lzSourceEndpoint, destinationChain, message)
    const encoded = defaultAbiCoder.encode(
        ["tuple(bytes4,tuple(bytes32,uint32,uint32,bytes))"],
        [[MAGIC_BYTES, [lzSourceAddress, lzSourceEndpoint, destinationChain, message]]]
    );
    return getBytes(encoded);
}

export enum PayloadEnum {
    XCM = "XCM",
    SYMBIOTIC = "SYMBIOTIC",
    LAYER_ZERO = "LAYER_ZERO",
}

export function encodeRawPayload(api: ApiPromise, bytes: Uint8Array, payloadEnum: PayloadEnum): Uint8Array {
    api.registry.register({
        RawPayload: {
            _enum: {
                Xcm: "Vec<u8>",
                Symbiotic: "Vec<u8>",
                LayerZero: "Vec<u8>",
            },
        },
    });

    if (payloadEnum === PayloadEnum.XCM) {
        const rawPayload = api.createType("RawPayload", {
            Xcm: Array.from(bytes),
        });

        return rawPayload.toU8a();
    }

    if (payloadEnum === PayloadEnum.SYMBIOTIC) {
        const rawPayload = api.createType("RawPayload", {
            Symbiotic: Array.from(bytes),
        });

        return rawPayload.toU8a();
    }

    if (payloadEnum === PayloadEnum.LAYER_ZERO) {
        const rawPayload = api.createType("RawPayload", {
            LayerZero: Array.from(bytes),
        });

        return rawPayload.toU8a();
    }

    throw new Error(`Unsupported PayloadEnum: ${payloadEnum}`);
}

function createXcmData(api: ApiPromise, xcmInstructions: any[]): Uint8Array {
    const xcmBytes = encodeXcmMessage(api, xcmInstructions);

    return encodeRawPayload(api, xcmBytes, PayloadEnum.XCM);
}

function createSymbioticData(api: ApiPromise, symbioticValidators: string[]) {
    const bytes = encodeSymbioticMessage(api, symbioticValidators);

    return encodeRawPayload(api, bytes, PayloadEnum.SYMBIOTIC);
}

function createLayerZeroData(
    api: ApiPromise,
    lzSourceAddress: Uint8Array,
    lzSourceEndpoint: number,
    destinationChain: number,
    message: Uint8Array
) {
    const bytes = encodeLayerZeroMessage(lzSourceAddress, lzSourceEndpoint, destinationChain, message);

    return encodeRawPayload(api, bytes, PayloadEnum.LAYER_ZERO);
}

export interface LayerZeroMessageParams {
    lzSourceAddress: Uint8Array;
    lzSourceEndpoint: number;
    destinationChain: number;
    message: Uint8Array;
}

export async function generateOutboundMessageAcceptedLog(
    api: ApiPromise,
    nonce: number,
    ethValue: bigint,
    instructions: any[] | null,
    nativeERC20Params: { value: bigint; tokenAddress: string }[] = [],
    foreignTokenParams: { value: bigint; tokenId: string }[] = [],
    symbioticValidators: string[] | null = null,
    layerZeroParams: LayerZeroMessageParams | null = null
) {
    const gatewayHex = "EDa338E4dC46038493b885327842fD3E301CaB39";
    const origin = `0x${gatewayHex}`;
    const gatewayAddress = Uint8Array.from(Buffer.from(gatewayHex, "hex"));

    const defaultAbiCoder = AbiCoder.defaultAbiCoder();

    const symbioticData = symbioticValidators !== null ? createSymbioticData(api, symbioticValidators) : null;
    const xcmData = instructions !== null ? createXcmData(api, instructions) : null;
    const layerZeroData =
        layerZeroParams !== null
            ? createLayerZeroData(
                  api,
                  layerZeroParams.lzSourceAddress,
                  layerZeroParams.lzSourceEndpoint,
                  layerZeroParams.destinationChain,
                  layerZeroParams.message
              )
            : null;

    const payloadData = xcmData || symbioticData || layerZeroData;
    if (!payloadData) {
        throw new Error("You need to specify XCM instructions, Symbiotic payload, or LayerZero params!");
    }

    const payload = {
        origin,
        assets: [
            ...nativeERC20Params.map((nativeTokenParam) => ({
                kind: 0,
                data: encodeAsNativeTokenERC20(nativeTokenParam.tokenAddress, nativeTokenParam.value),
            })),
            ...foreignTokenParams.map((foreignTokenParam) => ({
                kind: 1,
                data: encodeAsForeignTokenERC20(foreignTokenParam.tokenId, foreignTokenParam.value),
            })),
        ],
        xcm: { kind: 0, data: payloadData },
        claimer: "0x",
        value: ethValue,
        executionFee: 0n,
        relayerFee: 0n,
    };

    // Signature for event OutboundMessageAccepted(uint64 nonce, Payload payload) - fixed value
    const signature = hexToU8a("0x550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c");
    const topics = [signature];
    const assetsEncoded = payload.assets.map((asset) => [asset.kind, asset.data]);
    const xcmEncoded = [payload.xcm.kind, payload.xcm.data];

    const encodedDataString = defaultAbiCoder.encode(
        ["uint64", "tuple(address,tuple(uint8,bytes)[],tuple(uint8,bytes),bytes,uint128,uint128,uint128)"],
        [
            nonce,
            [
                payload.origin,
                assetsEncoded,
                xcmEncoded,
                payload.claimer,
                payload.value,
                payload.executionFee,
                payload.relayerFee,
            ],
        ]
    );

    const encodedData = getBytes(encodedDataString);

    return api.createType<SnowbridgeVerificationPrimitivesLog>("SnowbridgeVerificationPrimitivesLog", {
        address: gatewayAddress,
        topics,
        data: [].slice.call(encodedData),
    });
}

/**
 * Convenience function for generating LayerZero outbound message logs.
 * Only requires the parameters relevant to LayerZero messages.
 */
export async function generateLayerZeroOutboundLog(
    api: ApiPromise,
    nonce: number,
    layerZeroParams: LayerZeroMessageParams
) {
    return generateOutboundMessageAcceptedLog(
        api,
        nonce,
        0n, // ethValue
        null, // instructions
        [], // nativeERC20Params
        [], // foreignTokenParams
        null, // symbioticValidators
        layerZeroParams
    );
}

/**
 * Convenience function for generating Symbiotic outbound message logs.
 * Only requires the parameters relevant to Symbiotic messages.
 */
export async function generateSymbioticOutboundLog(api: ApiPromise, nonce: number, validators: string[]) {
    return generateOutboundMessageAcceptedLog(
        api,
        nonce,
        0n, // ethValue
        null, // instructions
        [], // nativeERC20Params
        [], // foreignTokenParams
        validators, // symbioticValidators
        null // layerZeroParams
    );
}

function createSyncCommittee(seed: number) {
    const skBytes: Buffer[] = [];
    for (let i = seed; i < seed + SYNC_COMMITTEE_SIZE; i++) {
        const buffer = Buffer.alloc(32, 0);
        buffer.writeInt16BE(i + 1, 30); // Offset to ensure the SK is less than the order
        skBytes.push(buffer);
    }
    const sks = skBytes.map((skBytes) => bls.SecretKey.fromBytes(skBytes));
    const pks = sks.map((sk) => sk.toPublicKey());
    const pubkeys = pks.map((pk) => pk.toBytes());

    const syncCommittee: altair.SyncCommittee = {
        pubkeys,
        aggregatePubkey: bls.aggregatePublicKeys(pubkeys),
    };

    const syncCommitteeFast: SyncCommitteeFast = {
        pubkeys: pks,
        aggregatePubkey: bls.PublicKey.fromBytes(bls.aggregatePublicKeys(pubkeys)),
    };

    return {
        syncCommittee,
        syncCommitteeFast,
        sks,
    };
}

async function createReceiptTrie(snowbridgeLogs: Array<SnowbridgeVerificationPrimitivesLog>, logsBloom: Uint8Array) {
    const rawLogs = [];
    for (const log of snowbridgeLogs) {
        const rawLog: Log = [log.address, log.topics, log.data];
        rawLogs.push(rawLog);
    }

    const trie = await Trie.create();
    const receipt: PostByzantiumTxReceipt = {
        status: 0,
        cumulativeBlockGasUsed: 1n,
        bitvector: logsBloom,
        logs: rawLogs,
    };

    const encodedReceipt = encodeReceipt(receipt, TransactionType.Legacy);

    await trie.put(encodedReceipt, encodedReceipt);

    const proof = await trie.createProof(encodedReceipt);
    const root = trie.root();

    // Hack to get correct Vec<Bytes>
    const transformedProof = [[]];
    for (let i = 0; i < proof.length; i++) {
        for (let j = 0; j < proof[i].length; j++) {
            transformedProof[i][j] = proof[i][j];
        }
    }

    return { root: root, proof: transformedProof };
}

export async function generateEventLog(
    api: ApiPromise,
    gatewayAddress: Uint8Array,
    channel_id: Uint8Array,
    message_id: Uint8Array,
    nonce: number,
    payload: Uint8Array
) {
    // Signature for event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload);
    const signature = new Uint8Array([
        113, 83, 249, 53, 124, 142, 164, 150, 187, 166, 11, 248, 46, 103, 20, 62, 39, 182, 68, 98, 180, 144, 65, 248,
        230, 137, 225, 176, 87, 40, 248, 79,
    ]);
    const topics = [signature, channel_id, message_id];

    const defaultAbiCoder = AbiCoder.defaultAbiCoder();
    const encodedDataString = defaultAbiCoder.encode(["uint64", "bytes"], [nonce, payload]);
    const encodedData = getBytes(encodedDataString);

    return api.createType<SnowbridgeVerificationPrimitivesLog>("SnowbridgeVerificationPrimitivesLog", {
        address: gatewayAddress,
        topics,
        data: [].slice.call(encodedData),
    });
}

export async function generateOutboundEventLogV2(
    api: ApiPromise,
    gatewayAddress: Uint8Array,
    messageId: Uint8Array,
    nonce: number,
    success: boolean,
    rewardAddress: Uint8Array
) {
    // Signature for event InboundMessageDispatched(uint64 indexed nonce, bytes32 topic, bool success, bytes32 rewardAddress);
    const signature = hexToU8a("0x8856ab63954e6c2938803a4654fb704c8779757e7bfdbe94a578e341ec637a95");

    const defaultAbiCoder = AbiCoder.defaultAbiCoder();
    const encodedTopic = getBytes(defaultAbiCoder.encode(["uint64"], [nonce]));
    const topics = [signature, encodedTopic];
    const encodedDataString = defaultAbiCoder.encode(
        ["bytes32", "bool", "bytes32"],
        [messageId, success, rewardAddress]
    );
    const encodedData = getBytes(encodedDataString);

    return api.createType<SnowbridgeVerificationPrimitivesLog>("SnowbridgeVerificationPrimitivesLog", {
        address: gatewayAddress,
        topics,
        data: [].slice.call(encodedData),
    });
}

export async function generateUpdate(api: ApiPromise, logs: Array<SnowbridgeVerificationPrimitivesLog>) {
    // Global variables
    const genValiRoot = Buffer.alloc(32, 9);

    const updateHeaderSlot = EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH + 1;

    const { syncCommittee, syncCommitteeFast, sks: syncCommitteeSks } = createSyncCommittee(2);

    // Prepare beacon body
    const checkPointHeader = defaultBeaconBlockHeader(updateHeaderSlot - 4);
    const checkPointBeaconBlockBody = ssz.electra.BlindedBeaconBlockBody.defaultViewDU();

    // Somehow the Bytes type does not like Uint8Array so we define number array instead
    const bareLogsBloom = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1,
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3,
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4,
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5,
        6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6,
        7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    checkPointBeaconBlockBody.executionPayloadHeader.logsBloom = new Uint8Array(bareLogsBloom);

    const { root, proof } = await createReceiptTrie(logs, checkPointBeaconBlockBody.executionPayloadHeader.logsBloom);
    checkPointBeaconBlockBody.executionPayloadHeader.receiptsRoot = root;

    checkPointBeaconBlockBody.commit();
    const blockBodyRoot = checkPointBeaconBlockBody.hashTreeRoot();
    const executionPayloadHeaderBranch = new Tree(checkPointBeaconBlockBody.node).getSingleProof(
        BigInt(BLOCK_BODY_EXECUTION_PAYLOAD_GINDEX)
    );
    // Link beacon body to checkpoint ancestor header
    checkPointHeader.beacon.bodyRoot = blockBodyRoot;

    const checkPointHeaderState = ssz.electra.BeaconState.defaultViewDU();
    checkPointHeaderState.currentSyncCommittee = ssz.altair.SyncCommittee.toViewDU(syncCommittee);
    checkPointHeaderState.commit();
    const checkPointStateCurrentSyncCommitteeBranch = new Tree(checkPointHeaderState.node).getSingleProof(
        BigInt(CURRENT_SYNC_COMMITTEE_GINDEX_ELECTRA)
    );
    const checkPointblockRootsBranch = new Tree(checkPointHeaderState.node).getSingleProof(
        BigInt(BLOCKS_ROOTS_ROOT_GINDEX_ELECTRA)
    );
    const checkPointblocksRootRoot = checkPointHeaderState.blockRoots.hashTreeRoot();
    checkPointHeader.beacon.stateRoot = checkPointHeaderState.hashTreeRoot();

    const header = api.createType<SnowbridgeBeaconPrimitivesBeaconHeader>("SnowbridgeBeaconPrimitivesBeaconHeader", {
        slot: updateHeaderSlot - 4,
        bodyRoot: blockBodyRoot,
        stateRoot: checkPointHeader.beacon.stateRoot,
    });

    const logsBloomInBytes = api.createType<Bytes>("Bytes", bareLogsBloom);
    const executionHeader = api.createType<SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader>(
        "SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader",
        {
            receiptsRoot: root,
            logsBloom: logsBloomInBytes,
        }
    );

    const executionPayloadHeader = api.createType<SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader>(
        "SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader",
        {
            Deneb: executionHeader,
        }
    );

    const executionBranch = api.createType<Vec<H256>>(
        "Vec<H256>",
        executionPayloadHeaderBranch.map((v) => u8aToHex(v))
    );

    const executionProof = api.createType<SnowbridgeBeaconPrimitivesExecutionProof>(
        "SnowbridgeBeaconPrimitivesExecutionProof",
        {
            header: header,
            executionHeader: executionPayloadHeader,
            executionBranch: executionBranch,
            ancestryProof: null,
        }
    );

    // This is not really used in verification so we can pass whatever
    const dummyReceiptRoot = api.createType<Vec<Bytes>>("Vec<Bytes>", []);

    const receiptProof = api.createType<Vec<Bytes>>("Vec<Bytes>", proof);

    const checkpointUpdate = api.createType<SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate>(
        "SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate",
        {
            header,
            validatorsRoot: genValiRoot,
            blockRootsRoot: checkPointblocksRootRoot,
            blockRootsBranch: checkPointblockRootsBranch,
            currentSyncCommittee: checkPointHeaderState.currentSyncCommittee.toValue(),
            currentSyncCommitteeBranch: checkPointStateCurrentSyncCommitteeBranch,
        }
    );

    const messageProof = api.createType<SnowbridgeVerificationPrimitivesProof>(
        "SnowbridgeVerificationPrimitivesProof",
        {
            receiptProof: [dummyReceiptRoot, receiptProof],
            executionProof: executionProof,
        }
    );

    const messageExtrinsics = [];
    for (const log of logs) {
        messageExtrinsics.push(
            api.createType<SnowbridgeVerificationPrimitivesEventProof>("SnowbridgeVerificationPrimitivesEventProof", {
                eventLog: log,
                proof: messageProof,
            })
        );
    }

    return { checkpointUpdate, messageExtrinsics };
}
