import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "util/block";

describeSuite({
    id: "DEVT0603",
    title: "Starlight <> Ethereum - Rewards mapping",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "T01",
            title: "Should succeed calling runtimeApi for generating/validating merkle proofs",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const aliceStash = keyring.addFromUri("//Alice//stash");
                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Register Alice as an external validator, because it starts as a whitelisted validator and whitelisted
                // validators don't get rewards.
                let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();

                await context.createBlock([
                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address))
                        .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.externalValidators.setExternalValidators([aliceStash.address], 1))
                        .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                ]);

                // Since collators are not assigned until session 2, we need to go till session 2 to actually see heads being injected
                await jumpToSession(context, 3);
                await context.createBlock();

                // We will only check alice's proof as she is the only one validating candidates
                const aliceMerkleProof = await polkadotJs.call.externalValidatorsRewardsApi.generateRewardsMerkleProof(
                    aliceStash.address,
                    1
                );
                expect(aliceMerkleProof.isEmpty).to.be.false;

                const isValidProofAlice = await polkadotJs.call.externalValidatorsRewardsApi.verifyRewardsMerkleProof(
                    aliceMerkleProof.toJSON()
                );

                expect(isValidProofAlice.toJSON()).to.be.eq(true);
            },
        });
    },
});
