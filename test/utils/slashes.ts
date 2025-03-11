import type { ApiPromise } from "@polkadot/api";
import type {
    BabeEquivocationProof,
    GrandpaEquivocationProof,
    GrandpaEquivocation,
    GrandpaEquivocationValue,
} from "@polkadot/types/interfaces";
import type { SpRuntimeHeader, SpRuntimeDigestDigestItem, FinalityGrandpaPrevote } from "@polkadot/types/lookup";
import type { KeyringPair } from "@moonwall/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import { u8aToHex, stringToHex, hexToU8a } from "@polkadot/util";

export async function generateBabeEquivocationProof(
    api: ApiPromise,
    pair: KeyringPair
): Promise<BabeEquivocationProof | null> {
    const baseHeader = await api.rpc.chain.getHeader();
    const baseHeader2 = await api.rpc.chain.getHeader();

    const header1: SpRuntimeHeader = api.createType("SpRuntimeHeader", {
        digest: baseHeader.digest,
        extrinsicsRoot: baseHeader.extrinsicsRoot,
        stateRoot: baseHeader.stateRoot,
        parentHash: baseHeader.parentHash,
        number: 1,
    });

    // we just change the block number
    const header2: SpRuntimeHeader = api.createType("SpRuntimeHeader", {
        digest: baseHeader2.digest,
        extrinsicsRoot: baseHeader2.extrinsicsRoot,
        stateRoot: baseHeader2.stateRoot,
        parentHash: baseHeader2.parentHash,
        number: 2,
    });

    const sig1 = pair.sign(blake2AsHex(header1.toU8a()));
    const sig2 = pair.sign(blake2AsHex(header2.toU8a()));

    const slot = await api.query.babe.currentSlot();

    const digestItemSeal1: SpRuntimeDigestDigestItem = api.createType("SpRuntimeDigestDigestItem", {
        Seal: [stringToHex("BABE"), u8aToHex(sig1)],
    });

    const digestItemSeal2: SpRuntimeDigestDigestItem = api.createType("SpRuntimeDigestDigestItem", {
        Seal: [stringToHex("BABE"), u8aToHex(sig2)],
    });

    header1.digest.logs.push(digestItemSeal1);
    header2.digest.logs.push(digestItemSeal2);

    const doubleVotingProof: BabeEquivocationProof = api.createType("BabeEquivocationProof", {
        offender: pair.publicKey,
        slotNumber: slot,
        firstHeader: header1,
        secondHeader: header2,
    });
    return doubleVotingProof;
}

export async function generateGrandpaEquivocationProof(
    api: ApiPromise,
    pair: KeyringPair
): Promise<GrandpaEquivocationProof | null> {
    const prevote1: FinalityGrandpaPrevote = api.createType("FinalityGrandpaPrevote", {
        targetHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
        targetNumber: 1,
    });

    const prevote2: FinalityGrandpaPrevote = api.createType("FinalityGrandpaPrevote", {
        targetHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
        targetNumber: 2,
    });

    const roundNumber = api.createType("u64", 1);
    const setId = await api.query.grandpa.currentSetId();

    // I could not find the proper struct that holds all this into a singl message
    // ergo I need to construct the signing payload myself
    // the first 0 is because of this enum variant
    // https://github.com/paritytech/finality-grandpa/blob/8c45a664c05657f0c71057158d3ba555ba7d20de/src/lib.rs#L228
    // then we have the prevote message
    // then the round number
    // then the set id
    const toSign1 = new Uint8Array([
        ...hexToU8a("0x00"),
        ...prevote1.toU8a(),
        ...roundNumber.toU8a(),
        ...setId.toU8a(),
    ]);

    const toSign2 = new Uint8Array([
        ...hexToU8a("0x00"),
        ...prevote2.toU8a(),
        ...roundNumber.toU8a(),
        ...setId.toU8a(),
    ]);
    const sig1 = pair.sign(toSign1);
    const sig2 = pair.sign(toSign2);

    const equivocationValue: GrandpaEquivocationValue = api.createType("GrandpaEquivocationValue", {
        roundNumber,
        identity: pair.address,
        first: [prevote1, sig1],
        second: [prevote2, sig2],
    });

    const equivocation: GrandpaEquivocation = api.createType("GrandpaEquivocation", {
        Prevote: equivocationValue,
    });

    const doubleVotingProof: GrandpaEquivocationProof = api.createType("GrandpaEquivocationProof", {
        setId,
        equivocation,
    });
    return doubleVotingProof;
}
