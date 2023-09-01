import "@tanssi/api-augment";
import { describeSuite, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DT3301",
    title: "Fee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

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
                const initialBlock = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();

                // Ensure that executePendingOperations can only be executed after 2 sessions, meaning that if the
                // current session number is 0, we must wait until after the NewSession event for session 2.
                // TODO: this does not actually test anything yet, the output needs to be inspected manually
                for (let i = 0; i < 100; i++) {
                    try {
                        const tx2 = polkadotJs.tx.pooledStaking.executePendingOperations([
                            {
                                delegator: alice.address,
                                operation: {
                                    JoiningAutoCompounding: {
                                        candidate: alice.address,
                                        atBlock: initialBlock,
                                    },
                                },
                            },
                        ]);

                        await context.createBlock([await tx2.signAsync(bob)]);
                    } catch {
                        await context.createBlock();
                    }
                }
            },
        });
    },
});
