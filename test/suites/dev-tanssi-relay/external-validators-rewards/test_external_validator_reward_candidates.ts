import "@tanssi/api-augment";
import { describeSuite, customDevRpcRequest, expect, beforeAll } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "util/block";

describeSuite({
    id: "DEVT0601",
    title: "Paras inherent tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "E01",
            title: "para candidates should trigger reward info",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const aliceStash = keyring.addFromUri("//Alice//stash");

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

                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);
                // Since collators are not assigned until session 2, we need to go till session 2 to actually see heads being injected
                await jumpToSession(context, 3);
                const sessionStartBlockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toBigInt();
                await context.createBlock();

                // we are now in era 1
                const validatorRewards = await context
                    .polkadotJs()
                    .query.externalValidatorsRewards.rewardPointsForEra(1);
                const totalRewards = validatorRewards.total.toBigInt();

                const blockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toBigInt();

                // Validators get 20 points for creating a block, so if they included a candidate, they will get more than 20
                expect(totalRewards).to.be.greaterThan(20n * (blockNumber - sessionStartBlockNumber));

                // Create another block to make sure
                await context.createBlock();
                const validatorRewards2 = await context
                    .polkadotJs()
                    .query.externalValidatorsRewards.rewardPointsForEra(1);
                const totalRewards2 = validatorRewards2.total.toBigInt();
                // 20 points per block + 20 points per candidate
                expect(totalRewards2 - totalRewards).toBe(40n);

                // All of them come from alice as she is the only one validating candidates
                expect(validatorRewards.individual.toHuman()[aliceStash.address]).to.be.eq(totalRewards.toString());
            },
        });

        /*
        it({
            id: "E02",
            title: "Check rewards storage clears after historyDepth",
            test: async function () {
                const sessionsPerEra = await polkadotJs.consts.externalValidators.sessionsPerEra;
                const historyDepth = await polkadotJs.consts.externalValidatorsRewards.historyDepth;

                const currentIndex = await polkadotJs.query.session.currentIndex();

                const targetSession =
                    currentIndex.toNumber() + sessionsPerEra.toNumber() * (historyDepth.toNumber() + 1);

                await jumpToSession(context, targetSession);

                const validatorRewards = await context
                    .polkadotJs()
                    .query.externalValidatorsRewards.rewardPointsForEra(0);
                const totalRewards = validatorRewards.total.toBigInt();

                // rewards should have expired
                expect(totalRewards).to.be.equal(0n);
            },
        });
         */
    },
});
