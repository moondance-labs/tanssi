import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "util/block";

describeSuite({
    id: "DTR0820",
    title: "Starlight <> Ethereum - Rewards mapping",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "T01",
            title: "Should succeed calling runtimeApi for generating/validating merkle proofs",
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const aliceStash = keyring.addFromUri("//Alice//stash");
                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);
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
