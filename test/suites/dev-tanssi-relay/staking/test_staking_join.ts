import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { numberToHex } from "@polkadot/util";
import { jumpToBlock } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_POOLED_STAKING, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1801",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        // TODO: don't hardcode the period here
        const sessionPeriod = 10;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightPS: boolean;

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

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightPS =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_POOLED_STAKING.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Cannot execute stake join before 2 sessions",
            test: async () => {
                const initialSession = 0;
                const tx = polkadotJs.tx.pooledStaking.requestDelegate(
                    alice.address,
                    "AutoCompounding",
                    10000000000000000n
                );

                if (shouldSkipStarlightPS) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));

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

                    // executePendingOperations should be filtered too
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(bob));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);
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
                        candidate: alice.address,
                        stake: numberToHex(10000000000000000, 128),
                    },
                ]);

                // Ensure that executePendingOperations can only be executed after 2 sessions, meaning that if the
                // current session number is 0, we must wait until after the NewSession event for session 2.
                // Jump to block 9
                await jumpToBlock(context, 2 * sessionPeriod - 1);
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

                await context.createBlock([await tx2.signAsync(bob)]);
                // executePendingOperations failed
                const events2 = await polkadotJs.query.system.events();
                const ev4 = events2.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev4.length).to.be.equal(1);

                // We are now in block 19, jump to block 21
                await context.createBlock();
                await context.createBlock();

                // Now the executePendingOperations should succeed
                await context.createBlock([await tx2.signAsync(bob)]);

                const events3 = await polkadotJs.query.system.events();
                const ev5 = events3.filter((a) => {
                    return a.event.method === "StakedAutoCompounding";
                });
                expect(ev5.length).to.be.equal(1);
                const ev6 = events3.filter((a) => {
                    return a.event.method === "ExecutedDelegate";
                });
                expect(ev6.length).to.be.equal(1);
            },
        });
    },
});
