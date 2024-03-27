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
            const atBlockHash="0x45edf89b5048fa29d2b3df548b3f3105195af7964413eb5713e126d13c914e60"
            const overallApi = context.polkadotJs();
            api = await overallApi.at(atBlockHash);
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Invulnerables have priority over staking candidates",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }

                const overallApi = context.polkadotJs();
                const sessionLength = 300;
                const currentBlock = (await overallApi.rpc.chain.getBlock()).block.header.number.toNumber();

                const blockToCheck = (Math.trunc(currentBlock / sessionLength) - 1) * sessionLength;

                const apiJustBeforeTheSession = await overallApi.at(await overallApi.rpc.chain.getBlockHash(blockToCheck + 1));
                console.log("Current block:", currentBlock, "Api at block:", blockToCheck + 1);

                // TODO: we should read the invulnerables at the start of this session, because that's when collators are updated
                const invulnerables = await apiJustBeforeTheSession.query.invulnerables.invulnerables();
                const eligibleCandidates = (await apiJustBeforeTheSession.query.pooledStaking.sortedEligibleCandidates()).map(
                    ({ candidate }) => candidate.toString()
                );
                const collators = await api.query.session.validators();

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

                    console.log("Collators not part of eligible candidates", collatorsNotInvulnerables.filter((collator) => !eligibleCandidates.includes(collator.toString())))

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
