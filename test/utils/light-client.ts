import {describe, it, expect, beforeAll, vi} from "vitest";
import bls from "@chainsafe/bls";
import {Tree} from "@chainsafe/persistent-merkle-tree";
import {altair, ssz, deneb, Slot} from "@lodestar/types";
import {chainConfig} from "@lodestar/config/default";
import {BeaconConfig, createBeaconConfig} from "@lodestar/config";
import {
    EPOCHS_PER_SYNC_COMMITTEE_PERIOD,
    FINALIZED_ROOT_GINDEX,
    NEXT_SYNC_COMMITTEE_GINDEX,
    SLOTS_PER_EPOCH,
    SYNC_COMMITTEE_SIZE,
    BLOCK_BODY_EXECUTION_PAYLOAD_GINDEX,
    HISTORICAL_ROOTS_LIMIT,
    SLOTS_PER_HISTORICAL_ROOT,
    DOMAIN_SYNC_COMMITTEE
} from "@lodestar/params";
import {Trie} from "@ethereumjs/trie";
import {RLP} from "@ethereumjs/rlp";
import {PostByzantiumTxReceipt} from "@ethereumjs/vm";
import { intToBytes, bigIntToBytes } from "@ethereumjs/util";
import {Log} from "@ethereumjs/evm";
import {keccak256} from "ethereum-cryptography/keccak.js";
import {SyncCommitteeFast} from "@lodestar/light-client";
import { computeSigningRoot } from "@lodestar/light-client/utils";
import { SecretKey } from "@chainsafe/bls/types";
import { BitArray } from "@chainsafe/ssz";
import { toHex } from "viem";

export const BLOCKS_ROOTS_ROOT_GINDEX = 37;
export const BLOCKS_ROOTS_ROOT_DEPTH = 5;
export const BLOCKS_ROOTS_ROOT_INDEX = 5;

export const BLOCKS_ROOT_AT_INDEX_DEPTH = 13;
export const BLOCKS_ROOT_AT_GINDEX_START = 2**BLOCKS_ROOT_AT_INDEX_DEPTH;

export function getSyncAggregateSigningRoot(
    config: BeaconConfig,
    syncAttestedBlockHeader: altair.LightClientHeader
  ): Uint8Array {
    const domain = config.getDomain(syncAttestedBlockHeader.beacon.slot, DOMAIN_SYNC_COMMITTEE);
    return computeSigningRoot(ssz.altair.LightClientHeader, syncAttestedBlockHeader, domain);
}


type LightClientSnapshotFast = {
    /** Beacon block header */
    header: deneb.LightClientHeader;
    /** Sync committees corresponding to the header */
    currentSyncCommittee: SyncCommitteeFast;
    nextSyncCommittee: SyncCommitteeFast;
};

export function signAndAggregate(message: Uint8Array, sks: SecretKey[]): altair.SyncAggregate {
    const sigs = sks.map((sk) => sk.sign(message));
    const aggSig = bls.Signature.aggregate(sigs).toBytes();
    return {
      syncCommitteeBits: BitArray.fromBoolArray(sks.map(() => true)),
      syncCommitteeSignature: aggSig,
    };
}
  
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

async function createReceiptTrie() {
    const log: Log = [Buffer.alloc(20), // 20-byte address
        [Buffer.alloc(0)], Buffer.alloc(20)];

    const trie = await Trie.create();
    let receipt: PostByzantiumTxReceipt  = {
        status: 1,
        cumulativeBlockGasUsed: 0n,
        bitvector: Buffer.alloc(0),
        logs: [log],
    };
    let encodedReceipt = RLP.encode(
        [
            intToBytes(receipt.status),
            bigIntToBytes(receipt.cumulativeBlockGasUsed),
            RLP.encode(receipt.logs),
        ],
    );

    const key = keccak256(encodedReceipt);
    await trie.put(key, encodedReceipt);
    return trie.root();
}

async function generateUpdate() {
    // Global variables
    const genValiRoot = Buffer.alloc(32, 9);
    const config = createBeaconConfig(chainConfig, genValiRoot);

    let update: deneb.LightClientUpdate;
    let snapshot: LightClientSnapshotFast;
    let blockRootsBranch: Uint8Array<ArrayBufferLike>[];
    let blocksRootRoot: Uint8Array;


    // Update slot must > snapshot slot
    // attestedHeaderSlot must == updateHeaderSlot + 1
    const snapshotHeaderSlot = 1;
    const updateHeaderSlot = EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH + 1;
    const attestedHeaderSlot = updateHeaderSlot + 1;

    const {syncCommittee: nextSyncCommittee, syncCommitteeFast: nextSyncCommitteeFast, sks} = createSyncCommittee(1);
    const {syncCommittee, syncCommitteeFast, sks: syncCommitteeSks} = createSyncCommittee(2);

    const ancestorState = ssz.deneb.BeaconState.defaultViewDU();
    // TODO: Make changes to ancestorState here
    ancestorState.commit();

    const ancestorHeader = defaultBeaconBlockHeader(updateHeaderSlot - 2);

    // Add receipt trie to execution payload header
    const beaconBlockBody = ssz.deneb.BlindedBeaconBlockBody.defaultViewDU();
    beaconBlockBody.executionPayloadHeader.receiptsRoot = await createReceiptTrie();
    beaconBlockBody.commit();//TODO: Take receiptsRoot from argument
    const beaconBlockBodyRoot = beaconBlockBody.hashTreeRoot();
    const executionPayloadHeaderBranch = new Tree(beaconBlockBody.node).getSingleProof(BigInt(BLOCK_BODY_EXECUTION_PAYLOAD_GINDEX));

    ancestorHeader.beacon.bodyRoot = beaconBlockBodyRoot;
    ancestorHeader.beacon.stateRoot = ancestorState.hashTreeRoot();

    const finalizedState = ssz.deneb.BeaconState.defaultViewDU();
    finalizedState.blockRoots.set(ancestorHeader.beacon.slot % SLOTS_PER_HISTORICAL_ROOT, ssz.deneb.LightClientHeader.hashTreeRoot(ancestorHeader));
    finalizedState.blockRoots.commit();
    finalizedState.commit();

    blockRootsBranch = new Tree(finalizedState.node).getSingleProof(BigInt(BLOCKS_ROOTS_ROOT_GINDEX));

    blocksRootRoot = finalizedState.blockRoots.hashTreeRoot();
    const blockRootBranch = new Tree(finalizedState.blockRoots.node).getSingleProof(BigInt(SLOTS_PER_HISTORICAL_ROOT + (ancestorHeader.beacon.slot % SLOTS_PER_HISTORICAL_ROOT)));

    // finalized header must have stateRoot to finalizedState
    const finalizedHeader = defaultBeaconBlockHeader(updateHeaderSlot);
    finalizedHeader.beacon.stateRoot = finalizedState.hashTreeRoot();

    // attestedState must have `finalizedHeader` as finalizedCheckpoint
    const attestedState = ssz.deneb.BeaconState.defaultViewDU();
    attestedState.finalizedCheckpoint = ssz.phase0.Checkpoint.toViewDU({
        epoch: 0,
        root: ssz.phase0.BeaconBlockHeader.hashTreeRoot(finalizedHeader.beacon),
    });

    // attested state must contain next sync committees
    attestedState.nextSyncCommittee = ssz.altair.SyncCommittee.toViewDU(nextSyncCommittee);

    // attestedHeader must have stateRoot to attestedState
    const attestedHeader = defaultBeaconBlockHeader(attestedHeaderSlot);
    attestedHeader.beacon.stateRoot = attestedState.hashTreeRoot();

    // Creates proofs for nextSyncCommitteeBranch and finalityBranch rooted in attested state
    const nextSyncCommitteeBranch = new Tree(attestedState.node).getSingleProof(BigInt(NEXT_SYNC_COMMITTEE_GINDEX));
    const finalityBranch = new Tree(attestedState.node).getSingleProof(BigInt(FINALIZED_ROOT_GINDEX));

    const signingRoot = getSyncAggregateSigningRoot(config, attestedHeader);
    const syncAggregate = signAndAggregate(signingRoot, syncCommitteeSks);

    update = {
        attestedHeader,
        nextSyncCommittee,
        nextSyncCommitteeBranch,
        finalizedHeader,
        finalityBranch,
        syncAggregate,
        signatureSlot: updateHeaderSlot,
    };

    snapshot = {
        header: defaultBeaconBlockHeader(snapshotHeaderSlot),
        currentSyncCommittee: syncCommitteeFast,
        nextSyncCommittee: nextSyncCommitteeFast,
    };

    console.dir(update, {depth: 3});
}



