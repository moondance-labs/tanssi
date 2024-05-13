import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S06",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Invulnerables have priority over staking candidates",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }

                const sessionLength = 600;

                const currentBlock = await api.rpc.chain.getBlock();
                const currentBlockNumber = currentBlock.block.header.number.toNumber();
                const currentBlockApi = await context.polkadotJs().at(currentBlock.block.hash);

                const currentSession = Math.trunc(currentBlockNumber / sessionLength);
                const blockToCheck = (currentSession - 1) * sessionLength - 1;

                const apiJustBeforeTheSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));

                const invulnerables = await apiJustBeforeTheSession.query.invulnerables.invulnerables();
                const eligibleCandidates = (
                    await apiJustBeforeTheSession.query.pooledStaking.sortedEligibleCandidates()
                ).map(({ candidate }) => candidate.toString());
                const collators = await currentBlockApi.query.session.validators();

                if (collators.length <= invulnerables.length) {
                    // Less collators than invulnerables: all collators must be invulnerables
                    for (const collator of collators) {
                        expect(
                            invulnerables.toJSON().includes(collator.toString()),
                            `Collator should be in invulnerable list: ${collator.toString()}`
                        ).to.be.true;
                    }
                } else {
                    // More collators than invulnerables: all invulnerables must be collators
                    for (const invulnerable of invulnerables) {
                        expect(
                            collators.toJSON().includes(invulnerable.toString()),
                            `Invulnerable should be in collators list: ${invulnerable.toString()}`
                        ).to.be.true;
                    }

                    // Remaining collators must be from staking
                    const collatorsNotInvulnerables = collators
                        .toJSON()
                        .filter((collator) => !invulnerables.toJSON().includes(collator.toString()));
                    for (const collator of collatorsNotInvulnerables) {
                        expect(
                            eligibleCandidates.includes(collator.toString()),
                            `Collator should be a staking candidate: ${collator.toString()}`
                        ).to.be.true;
                    }
                }
            },
        });
    },
});
