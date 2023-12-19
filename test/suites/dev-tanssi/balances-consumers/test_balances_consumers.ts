import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, isExtrinsicSuccessful } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";

describeSuite({
    id: "DT0101",
    title: "Consumers balances holds test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that authority assignment is correct on genesis",
            test: async function () {
                const randomAccount = generateKeyringPair("sr25519");

                const tx = polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, 2n * 10000000000000000n);
                await context.createBlock([await tx.signAsync(alice)]);
                expect(isExtrinsicSuccessful(await polkadotJs.query.system.events())).to.be.true;

                const consumersAfterTx1 = await polkadotJs.query.system.account(randomAccount.address);
                expect(consumersAfterTx1.consumers.toNumber()).to.be.equal(0);

                // Register keys in pallet_session
                const newKey = await polkadotJs.rpc.author.rotateKeys();
                const tx2 = polkadotJs.tx.session.setKeys(newKey, []);
                await context.createBlock([await tx2.signAsync(randomAccount)]);
                expect(isExtrinsicSuccessful(await polkadotJs.query.system.events())).to.be.true;
                const consumersAfterTx2 = await polkadotJs.query.system.account(randomAccount.address);
                expect(consumersAfterTx2.consumers.toNumber()).to.be.equal(1);

                // Self-delegate in pallet_pooled_staking
                const tx3 = polkadotJs.tx.pooledStaking.requestDelegate(
                    randomAccount.address,
                    "ManualRewards",
                    10000000000000000n
                );

                await context.createBlock([await tx3.signAsync(randomAccount)]);
                const consumersAfterTx3 = await polkadotJs.query.system.account(randomAccount.address);
                // We created a second consumer, which in this case is pooledStaking
                expect(consumersAfterTx3.consumers.toNumber()).to.be.equal(2);

                await jumpSessions(context, 2);

                // All pending operations where in session 0
                const tx4 = polkadotJs.tx.pooledStaking.executePendingOperations([
                    {
                        delegator: randomAccount.address,
                        operation: {
                            JoiningManualRewards: {
                                candidate: randomAccount.address,
                                at: 0,
                            },
                        },
                    },
                ]);
                await context.createBlock([await tx4.signAsync(randomAccount)]);

                const consumersAfterTx4 = await polkadotJs.query.system.account(randomAccount.address);
                expect(consumersAfterTx4.consumers.toNumber()).to.be.equal(2);

                // Self-delegate in pallet_pooled_staking
                const tx5 = polkadotJs.tx.pooledStaking.requestUndelegate(randomAccount.address, "ManualRewards", {
                    Stake: 10000000000000000n,
                });
                await context.createBlock([await tx5.signAsync(randomAccount)]);
                const consumersAfterTx5 = await polkadotJs.query.system.account(randomAccount.address);
                expect(consumersAfterTx5.consumers.toNumber()).to.be.equal(2);

                await jumpSessions(context, 2);

                // Leaving pending operations where in session 2
                const tx6 = polkadotJs.tx.pooledStaking.executePendingOperations([
                    {
                        delegator: randomAccount.address,
                        operation: {
                            Leaving: {
                                candidate: randomAccount.address,
                                at: 2,
                            },
                        },
                    },
                ]);
                await context.createBlock([await tx6.signAsync(randomAccount)]);
                // It is only after we leave that the consumer is cleaned
                const consumersAfterTx6 = await polkadotJs.query.system.account(randomAccount.address);
                expect(consumersAfterTx6.consumers.toNumber()).to.be.equal(1);
            },
        });
    },
});
