import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect, isExtrinsicSuccessful } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { numberToHex } from "@polkadot/util";
import { jumpToBlock } from "../../../util/block";

describeSuite({
    id: "DT0304",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        // TODO: don't hardcode the period here
        const sessionPeriod = 10;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();

            // Add alice and box keys to pallet session. In dancebox they are already there in genesis.
            const newKey1 = await polkadotJs.rpc.author.rotateKeys();
            const newKey2 = await polkadotJs.rpc.author.rotateKeys();

            await context.createBlock([
                await polkadotJs.tx.session.setKeys(newKey1, []).signAsync(alice),
                await polkadotJs.tx.session.setKeys(newKey2, []).signAsync(bob),
            ]);
        });

        it({
            id: "E01",
            title: "It takes 2 sessions to update pallet_session collators",
            test: async () => {
                const initialCollators = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();

                const randomAccount = generateKeyringPair("sr25519");

                const tx = polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, 2n * 10000000000000000n);
                await context.createBlock([await tx.signAsync(alice)]);
                expect(isExtrinsicSuccessful(await polkadotJs.query.system.events())).to.be.true;

                // Register keys in pallet_session
                const newKey = await polkadotJs.rpc.author.rotateKeys();
                const tx2 = polkadotJs.tx.session.setKeys(newKey, []);
                await context.createBlock([await tx2.signAsync(randomAccount)]);
                expect(isExtrinsicSuccessful(await polkadotJs.query.system.events())).to.be.true;

                // Self-delegate in pallet_pooled_staking
                const tx3 = polkadotJs.tx.pooledStaking.requestDelegate(
                    randomAccount.address,
                    "AutoCompounding",
                    10000000000000000n
                );
                await context.createBlock([await tx3.signAsync(randomAccount)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "IncreasedStake";
                });
                expect(ev1.length).to.be.equal(1);
                const ev2 = events.filter((a) => {
                    return a.event.method === "UpdatedCandidatePosition";
                });
                expect(ev2.length).to.be.equal(1);
                const ev3 = events.filter((a) => {
                    return a.event.method === "RequestedDelegate";
                });
                expect(ev3.length).to.be.equal(1);

                const stakingCandidates = await polkadotJs.query.pooledStaking.sortedEligibleCandidates();
                expect(stakingCandidates.toJSON()).to.deep.equal([
                    {
                        candidate: randomAccount.address,
                        stake: numberToHex(10000000000000000, 128),
                    },
                ]);

                // Jump to block 19
                await jumpToBlock(context, 2 * sessionPeriod);

                // Now pallet_session validators should not include the new one from staking
                const collators19 = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(collators19.toJSON()).to.deep.equal(initialCollators.toJSON());

                await context.createBlock();
                // We are now in block 20 but this block cannot include any transactions, so go to 21
                await context.createBlock();

                // Block 21: candidates that joined pallet_pooled_staking in session 0 are now eligible candidates
                const collators21 = await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain();
                expect(collators21.toJSON().containerChains[2000].length).to.equal(2);
                expect(collators21.toJSON().containerChains[2001].length).to.equal(2);
            },
        });
    },
});
