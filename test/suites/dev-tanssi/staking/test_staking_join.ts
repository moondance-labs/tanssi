import "@tanssi/api-augment";
import { describeSuite, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpToBlock } from "../../../util/block";

describeSuite({
    id: "DT3301",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        // TODO: don't hardcode the period here
        const sessionPeriod = 5;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Cannot execute stake join before 2 sessions",
            test: async function () {
                const tx = polkadotJs.tx.pooledStaking.requestDelegate(
                    alice.address,
                    "AutoCompounding",
                    10000000000000000n
                );
                await context.createBlock([await tx.signAsync(alice)]);
                const initialSession = 0;

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

                // TODO: check events, execute failed

                // We are now in block 10 but this block cannot include any transactions, so go to 11
                await context.createBlock();

                // Now the executePendingOperations should succeed
                await context.createBlock([await tx2.signAsync(bob)]);

                // TODO: check events, execute succeeded
            },
        });
    },
});
