import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { numberToHex } from "@polkadot/util";
import { jumpToBlock } from "utils";
import { encodeAddress } from "@polkadot/util-crypto";

describeSuite({
    id: "DEVT1806",
    title: "Pooled staking summaries",
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
            title: "Easy to fetch delegator's delegations",
            test: async () => {
                const initialSession = 0;

                const tx = polkadotJs.tx.pooledStaking.requestDelegate(
                    alice.address,
                    "AutoCompounding",
                    10000000000000000n
                );
                await context.createBlock([await tx.signAsync(alice)]);

                const tx2 = polkadotJs.tx.pooledStaking.requestDelegate(
                    bob.address,
                    "ManualRewards",
                    10000000000000000n
                );
                await context.createBlock([await tx2.signAsync(alice)]);

                // Delegation not from alice to test we can fetch delegations
                // from Alice only
                const tx3 = polkadotJs.tx.pooledStaking.requestDelegate(
                    bob.address,
                    "ManualRewards",
                    10000000000000000n
                );
                await context.createBlock([await tx3.signAsync(bob)]);

                // Jump to execute 1 delegation from alice
                await jumpToBlock(context, 2 * sessionPeriod + 2);

                const tx4 = polkadotJs.tx.pooledStaking.executePendingOperations([
                    {
                        delegator: alice.address,
                        operation: {
                            JoiningAutoCompounding: {
                                candidate: alice.address,
                                at: initialSession,
                            },
                        },
                    },
                    // we also execute bob's delegation to check its summary too
                    {
                        delegator: bob.address,
                        operation: {
                            JoiningManualRewards: {
                                candidate: bob.address,
                                at: initialSession,
                            },
                        },
                    },
                ]);

                // Now the executePendingOperations should succeed
                await context.createBlock([await tx4.signAsync(bob)]);
                const events3 = await polkadotJs.query.system.events();
                const ev = events3.filter((a) => {
                    return a.event.method === "StakedAutoCompounding";
                });
                expect(ev.length).to.be.equal(1);

                // Query list
                const keys = await polkadotJs.query.pooledStaking.delegatorCandidateSummaries.keys(alice.addressRaw);
                expect(keys.length).to.be.equal(2);

                const delegationCandidates = keys.map(({ args: [_delegator, candidate] }) => candidate);
                expect(delegationCandidates).to.deep.eq([bob.addressRaw, alice.addressRaw]);

                expect(
                    (
                        await polkadotJs.query.pooledStaking.delegatorCandidateSummaries(alice.address, alice.address)
                    ).toJSON()
                ).to.eq(2); // auto bitmask
                expect(
                    (
                        await polkadotJs.query.pooledStaking.delegatorCandidateSummaries(alice.address, bob.address)
                    ).toJSON()
                ).to.eq(1); // joining bitmask
                expect(
                    (
                        await polkadotJs.query.pooledStaking.delegatorCandidateSummaries(bob.address, bob.address)
                    ).toJSON()
                ).to.eq(4); // manual bitmask

                // Query candidate summaries
                const aliceSummary = await polkadotJs.query.pooledStaking.candidateSummaries(alice.addressRaw);
                expect(aliceSummary.toJSON()).to.deep.eq({
                    delegators: 1,
                    joiningDelegators: 0,
                    autoCompoundingDelegators: 1,
                    manualRewardsDelegators: 0,
                    leavingDelegators: 0,
                });

                const bobSummary = await polkadotJs.query.pooledStaking.candidateSummaries(bob.addressRaw);
                expect(bobSummary.toJSON()).to.deep.eq({
                    delegators: 2,
                    joiningDelegators: 1,
                    autoCompoundingDelegators: 0,
                    manualRewardsDelegators: 1,
                    leavingDelegators: 0,
                });
            },
        });
    },
});
