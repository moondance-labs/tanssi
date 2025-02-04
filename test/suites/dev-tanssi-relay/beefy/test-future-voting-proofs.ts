import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "../../../util/block";
import { u8aToHex, hexToU8a, stringToHex, numberToHex } from "@polkadot/util";
import type {
    SpConsensusBeefyFutureBlockVotingProof,
    SpConsensusBeefyPayload,
    SpConsensusBeefyCommitment,
    SpConsensusBeefyVoteMessage,
} from "@polkadot/types/lookup";
import { Keyring } from "@polkadot/keyring";
import { secp256k1Sign } from "@polkadot/util-crypto";

describeSuite({
    id: "DEVT1403",
    title: "BEEFY - Future voting proofs",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let aliceBeefyPair: KeyringPair;
        let bobBeefyPair: KeyringPair;
        let aliceBeefyPrivateKey: `0x${string}`;
        let bob: KeyringPair;
        beforeAll(() => {
            const keyringBeefy = new Keyring({ type: "ecdsa" });
            aliceBeefyPair = keyringBeefy.addFromUri("//Alice");
            bobBeefyPair = keyringBeefy.addFromUri("//Bob");
            aliceBeefyPrivateKey = "0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854";
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Should be able to report a valid FutureVotingProof",
            test: async () => {
                await jumpToSession(context, 1);
                await context.createBlock();

                const mmrRootId = hexToU8a(stringToHex("mh"));
                const payload: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);

                const validatorSetId = 1;
                const currentBlockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toNumber();

                // Commit to a future block
                const blockNumber = currentBlockNumber + 100;
                const commitment: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload,
                    blockNumber,
                    validatorSetId,
                });

                const message = commitment.toU8a();
                const signature = secp256k1Sign(message, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");

                const voteMessage: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment,
                    id: aliceBeefyPair.publicKey,
                    signature,
                });

                // Valid equivocation proof.
                const futureVotingProof: SpConsensusBeefyFutureBlockVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyFutureBlockVotingProof",
                    {
                        vote: voteMessage,
                    }
                );

                const keyOwnershipProof = await polkadotJs.call.beefyApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(aliceBeefyPair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.beefy.reportFutureBlockVoting(futureVotingProof, keyOwnershipProofHex);

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
                const payload: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);

                const validatorSetId = 1;
                const currentBlockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toNumber();

                // Commit to a future block
                const blockNumber = currentBlockNumber + 100;
                const commitment: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload,
                    blockNumber,
                    validatorSetId,
                });

                const message = commitment.toU8a();
                const signature = secp256k1Sign(message, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");

                const voteMessage: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment,
                    id: aliceBeefyPair.publicKey,
                    signature,
                });

                // Valid equivocation proof.
                const futureVotingProof: SpConsensusBeefyFutureBlockVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyFutureBlockVotingProof",
                    {
                        vote: voteMessage,
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

                const tx = polkadotJs.tx.beefy.reportFutureBlockVoting(futureVotingProof, keyOwnershipProofHex);

                const signedTx = await tx.signAsync(bob);
                const { result } = await context.createBlock(signedTx);

                expect(result?.successful).to.be.false;
                expect(result!.error?.section).to.eq("beefy");
                expect(result?.error?.name).to.eq("InvalidKeyOwnershipProof");
            },
        });

        it({
            id: "E03",
            title: "Should fail to report an invalid FutureVotingProof",
            test: async () => {
                await jumpToSession(context, 1);
                await context.createBlock();

                const mmrRootId = hexToU8a(stringToHex("mh"));
                const payload: SpConsensusBeefyPayload = polkadotJs.createType("SpConsensusBeefyPayload", [
                    [mmrRootId, numberToHex(42, 64)],
                ]);

                const validatorSetId = 1;

                // Commit to a past block
                const blockNumber = 1;
                const commitment: SpConsensusBeefyCommitment = polkadotJs.createType("SpConsensusBeefyCommitment", {
                    payload,
                    blockNumber,
                    validatorSetId,
                });

                const message = commitment.toU8a();
                const signature = secp256k1Sign(message, { secretKey: hexToU8a(aliceBeefyPrivateKey) }, "keccak");

                const voteMessage: SpConsensusBeefyVoteMessage = polkadotJs.createType("SpConsensusBeefyVoteMessage", {
                    commitment,
                    id: aliceBeefyPair.publicKey,
                    signature,
                });

                // Invalid equivocation proof.
                const futureVotingProof: SpConsensusBeefyFutureBlockVotingProof = polkadotJs.createType(
                    "SpConsensusBeefyFutureBlockVotingProof",
                    {
                        vote: voteMessage,
                    }
                );

                const keyOwnershipProof = await polkadotJs.call.beefyApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(aliceBeefyPair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.beefy.reportFutureBlockVoting(futureVotingProof, keyOwnershipProofHex);

                const signedTx = await tx.signAsync(bob);
                const { result } = await context.createBlock(signedTx);

                expect(result?.successful).to.be.false;
                expect(result!.error?.section).to.eq("beefy");
                expect(result?.error?.name).to.eq("InvalidFutureBlockVotingProof");
            },
        });
    },
});
