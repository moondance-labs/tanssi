import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "util/block";
import { encodeAddress } from "@polkadot/util-crypto";
import { MultiLocation } from "../../../util/xcm";

describeSuite({
    id: "DTR1602",
    title: "Ethereum reward tests",
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
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const aliceStash = keyring.addFromUri("//Alice//stash");

                // We need to register the token otherwise rewards are not sent to ethereum
                const tokenLocation: MultiLocation = {
                    parents: 0,
                    interior: "Here",
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "dance",
                    symbol: "dance",
                    decimals: 12,
                };

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
                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                        .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                ]);

                await context.createBlock();

                // Since collators are not assigned until session 2, we need to go till session 2 to actually see heads being injected
                await jumpToSession(context, 3);
                await context.createBlock();

                // we are now in era 1
                const validatorRewards = await context
                    .polkadotJs()
                    .query.externalValidatorsRewards.rewardPointsForEra(1);
                const totalRewards = validatorRewards.total.toBigInt();

                // 20 points per block
                expect(totalRewards).toBe(20n);

                // Create another block to make sure
                await context.createBlock();
                const validatorRewards2 = await context
                    .polkadotJs()
                    .query.externalValidatorsRewards.rewardPointsForEra(1);
                const totalRewards2 = validatorRewards2.total.toBigInt();
                // 20 points per block
                expect(totalRewards2).toBe(40n);

                // All of them come from alice as she is the only one producing blocks
                expect(validatorRewards2.individual.toHuman()[aliceStash.address]).to.be.eq(totalRewards2.toString());
            },
        });

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

        it({
            id: "E03",
            title: "Ethereum Sovereign Account balance should increase on session change",
            test: async function () {
                const currentIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                const account = encodeAddress("0xce796ae65569a670d0c1cc1ac12515a3ce21b5fbf729d63d7b289baad070139d", 0);
                const sessionsPerEra = await polkadotJs.consts.externalValidators.sessionsPerEra;

                const {
                    data: { free: balanceBefore },
                } = await context.polkadotJs().query.system.account(account);

                // We need to jump at least one era
                await jumpToSession(context, currentIndex + sessionsPerEra.toNumber());

                const {
                    data: { free: balanceAfter },
                } = await context.polkadotJs().query.system.account(account);

                expect(balanceAfter.toBigInt()).to.be.greaterThan(balanceBefore.toBigInt());
            },
        });
    },
});
