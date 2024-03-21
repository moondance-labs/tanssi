import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect, isExtrinsicSuccessful } from "@moonwall/cli";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { numberToHex } from "@polkadot/util";
import { jumpToBlock } from "../../../util/block";

describeSuite({
    id: "DT0304",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        // TODO: don't hardcode the period here
        const sessionPeriod = 10;

        beforeAll(async () => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "It takes 2 sessions to update pallet_session collators",
            test: async function () {
                const initialValidators = await polkadotJs.query.session.validators();

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
                    return a.event.method == "IncreasedStake";
                });
                expect(ev1.length).to.be.equal(1);
                const ev2 = events.filter((a) => {
                    return a.event.method == "UpdatedCandidatePosition";
                });
                expect(ev2.length).to.be.equal(1);
                const ev3 = events.filter((a) => {
                    return a.event.method == "RequestedDelegate";
                });
                expect(ev3.length).to.be.equal(1);

                const stakingCandidates = await polkadotJs.query.pooledStaking.sortedEligibleCandidates();
                expect(stakingCandidates.toJSON()).to.deep.equal([
                    {
                        candidate: randomAccount.address,
                        stake: numberToHex(10000000000000000, 128),
                    },
                ]);

                // Jump to block 9
                await jumpToBlock(context, 2 * sessionPeriod - 1);

                // Now pallet_session validators should not include the new one from staking
                const validators9 = await polkadotJs.query.session.validators();
                expect(validators9.toJSON()).to.deep.equal(initialValidators.toJSON());

                await context.createBlock();
                // We are now in block 10 but this block cannot include any transactions, so go to 11
                await context.createBlock();

                // Block 11: candidates that joined pallet_pooled_staking in session 0 are now eligible candidates
                const validators11 = await polkadotJs.query.session.validators();
                expect(validators11.toJSON()).to.deep.equal([...initialValidators.toJSON(), randomAccount.address]);
            },
        });
    },
});
