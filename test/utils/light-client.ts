import bls from "@chainsafe/bls";
import { Tree } from "@chainsafe/persistent-merkle-tree";
import { type altair, ssz, type deneb, type Slot } from "@lodestar/types";
import {
    EPOCHS_PER_SYNC_COMMITTEE_PERIOD,
    SLOTS_PER_EPOCH,
    SYNC_COMMITTEE_SIZE,
    BLOCK_BODY_EXECUTION_PAYLOAD_GINDEX,
} from "@lodestar/params";
import { Trie } from "@ethereumjs/trie";
import { encodeReceipt, type PostByzantiumTxReceipt } from "@ethereumjs/vm";
import type { Log } from "@ethereumjs/evm";
import type { SyncCommitteeFast } from "@lodestar/light-client";
import {
    SnowbridgeBeaconPrimitivesBeaconHeader,
    SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader,
    SnowbridgeBeaconPrimitivesExecutionProof,
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate,
    SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader, SnowbridgeCoreInboundLog,
    SnowbridgeCoreInboundMessage,
    SnowbridgeCoreInboundProof,
} from "@polkadot/types/lookup";
import type { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import type { H256 } from "@polkadot/types/interfaces";
import type { Bytes, Vec } from "@polkadot/types-codec";
import { TransactionType } from "@ethereumjs/tx";
import { AbiCoder } from "ethers/abi";
import { getBytes } from "ethers/utils";

export const BLOCKS_ROOTS_ROOT_GINDEX_ELECTRA = 69;

export const CURRENT_SYNC_COMMITTEE_GINDEX_ELECTRA = 86;

function defaultBeaconBlockHeader(slot: Slot): deneb.LightClientHeader {
    const header = ssz.deneb.LightClientHeader.defaultValue();
    header.beacon.slot = slot;
    return header;
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

async function createReceiptTrie(
    snowbridgeLogs: Array<SnowbridgeCoreInboundLog>,
    logsBloom: Uint8Array
) {
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

export async function generateEventLog(api: ApiPromise, gatewayAddress: Uint8Array, channel_id: Uint8Array, message_id: Uint8Array, nonce: number, payload: Uint8Array) {
    // Signature for event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload);
    const signature = new Uint8Array([113, 83, 249, 53, 124, 142, 164, 150, 187, 166, 11, 248, 46, 103, 20, 62, 39, 182, 68, 98, 180, 144, 65, 248, 230, 137, 225, 176, 87, 40, 248, 79]);
    const topics = [signature, channel_id, message_id];

    let defaultAbiCoder = AbiCoder.defaultAbiCoder();
    let encodedDataString = defaultAbiCoder.encode(["uint64", "bytes"], [nonce, payload]);
    let encodedData = getBytes(encodedDataString);

    return api.createType<SnowbridgeCoreInboundLog>("SnowbridgeCoreInboundLog", {
        address: gatewayAddress,
        topics,
        data: [].slice.call(encodedData)
    });
}

export async function generateUpdate(
    api: ApiPromise,
    logs: Array<SnowbridgeCoreInboundLog>
) {
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

    const { root, proof } = await createReceiptTrie(
        logs,
        checkPointBeaconBlockBody.executionPayloadHeader.logsBloom
    );
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
        " SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate",
        {
            header,
            validatorsRoot: genValiRoot,
            blockRootsRoot: checkPointblocksRootRoot,
            blockRootsBranch: checkPointblockRootsBranch,
            currentSyncCommittee: checkPointHeaderState.currentSyncCommittee.toValue(),
            currentSyncCommitteeBranch: checkPointStateCurrentSyncCommitteeBranch,
        }
    );

    const messageProof = api.createType<SnowbridgeCoreInboundProof>("SnowbridgeCoreInboundProof", {
        receiptProof: [dummyReceiptRoot, receiptProof],
        executionProof: executionProof,
    });

    const messageExtrinsics = [];
    for (const log of logs) {
        messageExtrinsics.push(
            api.createType<SnowbridgeCoreInboundMessage>("SnowbridgeCoreInboundMessage", {
                eventLog: log,
                proof: messageProof,
            })
        );
    }

    return { checkpointUpdate, messageExtrinsics };
}
