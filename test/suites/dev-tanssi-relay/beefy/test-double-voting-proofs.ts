import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "../../../util/block";
import { u8aToHex, hexToU8a, stringToHex, numberToHex } from "@polkadot/util";
import type {
    SpConsensusBeefyDoubleVotingProof,
    SpConsensusBeefyPayload,
    SpConsensusBeefyCommitment,
    SpConsensusBeefyVoteMessage,
} from "@polkadot/types/lookup";
import { Keyring } from "@polkadot/keyring";
import { secp256k1Sign } from "@polkadot/util-crypto";

describeSuite({
    id: "DEVT1402",
    title: "BEEFY - Double voting proofs",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let aliceBeefyPair: KeyringPair;
        let bobBeefyPair: KeyringPair;
        let aliceBeefyPrivateKey: `0x${string}`;
        let bobBeefyPrivateKey: `0x${string}`;
        let bob: KeyringPair;
        beforeAll(() => {
            const keyringBeefy = new Keyring({ type: "ecdsa" });
            aliceBeefyPair = keyringBeefy.addFromUri("//Alice");
            bobBeefyPair = keyringBeefy.addFromUri("//Bob");
            aliceBeefyPrivateKey = "0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854";
            bobBeefyPrivateKey = "0x79c3b7fc0b7697b9414cb87adcb37317d1cab32818ae18c0e97ad76395d1fdcf";
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Should be able to report a valid double voting proof",
            test: async () => {
                await jumpToSession(context, 1);
                await context.createBlock();

                const mmrRootId = hexToU8a(stringToHex("mh"));
                const payload1: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);
                const payload2: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(128, 64)],
                ]);

                const validatorSetId = 1;
                const blockNumber = 1;
                const commitment1: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload1,
                    blockNumber,
                    validatorSetId,
                });

                const commitment2: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload2,
                    blockNumber,
                    validatorSetId,
                });

                const message1 = commitment1.toU8a();
                const message2 = commitment2.toU8a();

                const signature1 = secp256k1Sign(message1, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");
                const signature2 = secp256k1Sign(message2, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");

                const voteMessage1: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment1,
                    id: aliceBeefyPair.publicKey,
                    signature: signature1,
                });

                const voteMessage2: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment2,
                    id: aliceBeefyPair.publicKey,
                    signature: signature2,
                });

                // Valid equivocation proof.
                // Equivocation proof with two votes in the same round for different payloads signed by the same key.
                const doubleVotingProof: SpConsensusBeefyDoubleVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyDoubleVotingProof",
                    {
                        first: voteMessage1,
                        second: voteMessage2,
                    }
                );

                const keyOwnershipProof = await polkadotJs.call.beefyApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(aliceBeefyPair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.beefy.reportDoubleVoting(doubleVotingProof, keyOwnershipProofHex);

                const signedTx = await tx.signAsync(bob);
                const { result } = await context.createBlock(signedTx, { allowFailures: false });

                expect(result?.successful, result?.error?.name).to.be.true;
            },
        });

        it({
            id: "E02",
            title: "Should fail to report proof if KeyOwnershipProof is invalid",
            test: async () => {
                await jumpToSession(context, 1);
                await context.createBlock();

                const mmrRootId = hexToU8a(stringToHex("mh"));
                const payload1: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);
                const payload2: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(128, 64)],
                ]);

                const validatorSetId = 1;
                const blockNumber = 1;
                const commitment1: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload1,
                    blockNumber,
                    validatorSetId,
                });

                const commitment2: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload2,
                    blockNumber,
                    validatorSetId,
                });

                const message1 = commitment1.toU8a();
                const message2 = commitment2.toU8a();

                const signature1 = secp256k1Sign(message1, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");
                const signature2 = secp256k1Sign(message2, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");

                const voteMessage1: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment1,
                    id: aliceBeefyPair.publicKey,
                    signature: signature1,
                });

                const voteMessage2: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment2,
                    id: aliceBeefyPair.publicKey,
                    signature: signature2,
                });

                // Valid equivocation proof.
                // Equivocation proof with two votes in the same round for different payloads signed by the same key.
                const doubleVotingProof: SpConsensusBeefyDoubleVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyDoubleVotingProof",
                    {
                        first: voteMessage1,
                        second: voteMessage2,
                    }
                );

                // Invalid proof: Bob is not part of the BEEFY validator set.
                const keyOwnershipProof = await polkadotJs.call.beefyApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(bobBeefyPair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.beefy.reportDoubleVoting(doubleVotingProof, keyOwnershipProofHex);

                const signedTx = await tx.signAsync(bob);
                const { result } = await context.createBlock(signedTx);

                expect(result?.successful).to.be.false;
                expect(result?.error?.section).to.eq("beefy");
                expect(result?.error?.name).to.eq("InvalidKeyOwnershipProof");
            },
        });

        it({
            id: "E03",
            title: "Should fail to report an invalid DoubleVotingProof",
            test: async () => {
                await jumpToSession(context, 1);
                await context.createBlock();

                const mmrRootId = hexToU8a(stringToHex("mh"));
                const payload1: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);
                const payload2: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(128, 64)],
                ]);

                const validatorSetId = 1;
                const blockNumber = 1;
                const commitment1: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload1,
                    blockNumber,
                    validatorSetId,
                });

                const commitment2: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload: payload2,
                    blockNumber,
                    validatorSetId,
                });

                const message1 = commitment1.toU8a();
                const message2 = commitment2.toU8a();

                const signature1 = secp256k1Sign(message1, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");
                const signature2 = secp256k1Sign(message2, { secretKey: hexToU8a(bobBeefyPrivateKey) }, "keccak");

                const voteMessage1: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment1,
                    id: aliceBeefyPair.publicKey,
                    signature: signature1,
                });

                const voteMessage2: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment: commitment2,
                    id: aliceBeefyPair.publicKey,
                    signature: signature2,
                });

                // Invalid equivocation proof.
                // Equivocation proof with two votes by different authorities.
                const doubleVotingProof: SpConsensusBeefyDoubleVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyDoubleVotingProof",
                    {
                        first: voteMessage1,
                        second: voteMessage2,
                    }
                );

                const keyOwnershipProof = await polkadotJs.call.beefyApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(aliceBeefyPair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.beefy.reportDoubleVoting(doubleVotingProof, keyOwnershipProofHex);

                const signedTx = await tx.signAsync(bob);
                const { result } = await context.createBlock(signedTx);

                expect(result?.successful).to.be.false;
                expect(result?.error?.section).to.eq("beefy");
                expect(result?.error?.name).to.eq("InvalidDoubleVotingProof");
            },
        });
    },
});
