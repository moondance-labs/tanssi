import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { numberToHex } from "@polkadot/util";
import { jumpToBlock } from "../../../util/block";

describeSuite({
    id: "DT0305",
    title: "Staking poolSwap test suite",
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
        });

        it({
            id: "E01",
            title: "poolSwap works",
            test: async function () {
                const initialSession = 0;
                const tx = polkadotJs.tx.pooledStaking.requestDelegate(
                    alice.address,
                    "AutoCompounding",
                    10000000000000000n
                );
                await context.createBlock([await tx.signAsync(alice)]);
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
                        candidate: alice.address,
                        stake: numberToHex(10000000000000000, 128),
                    },
                ]);

                await jumpToBlock(context, 2 * sessionPeriod + 1);
                const tx2 = polkadotJs.tx.pooledStaking.executePendingOperations([
                    {
                        delegator: alice.address,
                        operation: {
                            JoiningAutoCompounding: {
                                candidate: alice.address,
                                at: initialSession,
                            },
                        },
                    },
                ]);

                // Now the executePendingOperations should succeed
                await context.createBlock([await tx2.signAsync(bob)]);

                const events3 = await polkadotJs.query.system.events();
                const ev5 = events3.filter((a) => {
                    return a.event.method == "StakedAutoCompounding";
                });
                expect(ev5.length).to.be.equal(1);
                const ev6 = events3.filter((a) => {
                    return a.event.method == "ExecutedDelegate";
                });
                expect(ev6.length).to.be.equal(1);

                // We now try to swap
                const tx3 = polkadotJs.tx.pooledStaking.swapPool(alice.address, "AutoCompounding", {
                    Stake: 10000000000000000n,
                });
                await context.createBlock([await tx3.signAsync(alice)]);

                const events4 = await polkadotJs.query.system.events();
                const ev7 = events4.filter((a) => {
                    return a.event.method == "SwappedPool";
                });
                expect(ev7.length).to.be.equal(1);
            },
        });
    },
});
