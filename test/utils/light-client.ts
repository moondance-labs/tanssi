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
import type {
    SnowbridgeBeaconPrimitivesBeaconHeader,
    SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader,
    SnowbridgeBeaconPrimitivesExecutionProof,
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate,
    SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader,
    SnowbridgeCoreInboundMessage,
    SnowbridgeCoreInboundProof,
} from "@polkadot/types/lookup";
import type { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import type { H256 } from "@polkadot/types/interfaces";
import type { Bytes, Vec } from "@polkadot/types-codec";
import { TransactionType } from "@ethereumjs/tx";

export const BLOCKS_ROOTS_ROOT_GINDEX = 37;

export const CURRENT_SYNC_COMMITTEE_GINDEX = 54;

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
    contractAddress: Uint8Array,
    topics: Array<Uint8Array>,
    messages: Array<Uint8Array>,
    logsBloom: Uint8Array
) {
    const logs = [];
    for (const message of messages) {
        const log: Log = [contractAddress, topics, message];
        logs.push(log);
    }

    const trie = await Trie.create();
    const receipt: PostByzantiumTxReceipt = {
        status: 1,
        cumulativeBlockGasUsed: 1n,
        bitvector: logsBloom,
        logs: logs,
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

export async function generateUpdate(
    api: ApiPromise,
    contractAddress: Uint8Array,
    topics: Array<Uint8Array>,
    messages: Array<Uint8Array>
) {
    // Global variables
    const genValiRoot = Buffer.alloc(32, 9);

    const updateHeaderSlot = EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH + 1;

    const { syncCommittee, syncCommitteeFast, sks: syncCommitteeSks } = createSyncCommittee(2);

    // Prepare beacon body
    const checkPointHeader = defaultBeaconBlockHeader(updateHeaderSlot - 4);
    const checkPointBeaconBlockBody = ssz.deneb.BlindedBeaconBlockBody.defaultViewDU();

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
        contractAddress,
        topics,
        messages,
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

    const checkPointHeaderState = ssz.deneb.BeaconState.defaultViewDU();
    checkPointHeaderState.currentSyncCommittee = ssz.altair.SyncCommittee.toViewDU(syncCommittee);
    checkPointHeaderState.commit();
    const checkPointStateCurrentSyncCommitteeBranch = new Tree(checkPointHeaderState.node).getSingleProof(
        BigInt(CURRENT_SYNC_COMMITTEE_GINDEX)
    );
    const checkPointblockRootsBranch = new Tree(checkPointHeaderState.node).getSingleProof(
        BigInt(BLOCKS_ROOTS_ROOT_GINDEX)
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
    for (const message of messages) {
        const convertedMessage = [].slice.call(message);
        messageExtrinsics.push(
            api.createType<SnowbridgeCoreInboundMessage>("SnowbridgeCoreInboundMessage", {
                eventLog: {
                    address: contractAddress,
                    topics,
                    data: convertedMessage,
                },
                proof: messageProof,
            })
        );
    }

    return { checkpointUpdate, messageExtrinsics };
}
