import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK11",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Invulnerables have priority over staking candidates",
            test: async () => {
                const currentBlock = await api.rpc.chain.getBlock();
                const currentBlockApi = await context.polkadotJs().at(currentBlock.block.hash);

                let collators = [];
                const blockBabeEpochStart = (await api.query.babe.epochStart())[0].toNumber();
                const apiJustBeforeTheSession = await api.at(await api.rpc.chain.getBlockHash(blockBabeEpochStart - 1));
                const invulnerables = (
                    await apiJustBeforeTheSession.query.tanssiInvulnerables.invulnerables()
                ).toJSON() as string[];
                // we need to get current
                const currentAssignment = await currentBlockApi.query.tanssiCollatorAssignment.collatorContainerChain();
                // there should be no collator in orchestrator
                expect(
                    currentAssignment.orchestratorChain.toHuman(),
                    "In tanssi-solo there should be no collator assigned to orchestrator"
                ).to.be.empty;

                const containerAssignment = currentAssignment.containerChains.toHuman();

                for (const para in containerAssignment) {
                    const collatorsForPara = containerAssignment[para.toString()];
                    collators = collators.concat(collatorsForPara);
                }

                const eligibleCandidates = (
                    await apiJustBeforeTheSession.query.pooledStaking.sortedEligibleCandidates()
                ).map(({ candidate }) => candidate.toString());

                if (collators.length <= invulnerables.length) {
                    // Less collators than invulnerables: all collators must be invulnerables
                    for (const collator of collators) {
                        expect(invulnerables.includes(collator), `Collator should be in invulnerable list: ${collator}`)
                            .to.be.true;
                    }
                } else {
                    // More collators than invulnerables: all invulnerables must be collators
                    for (const invulnerable of invulnerables) {
                        expect(
                            collators.includes(invulnerable),
                            `Invulnerable should be in collators list: ${invulnerable}`
                        ).to.be.true;
                    }

                    // Remaining collators must be from staking
                    const collatorsNotInvulnerables = collators.filter((collator) => !invulnerables.includes(collator));
                    for (const collator of collatorsNotInvulnerables) {
                        expect(
                            eligibleCandidates.includes(collator),
                            `Collator should be a staking candidate: ${collator}`
                        ).to.be.true;
                    }
                }
            },
        });
    },
});
