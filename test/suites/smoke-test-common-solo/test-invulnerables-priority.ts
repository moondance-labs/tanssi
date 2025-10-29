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
                const currentBlockNumber = currentBlock.block.header.number.toNumber();
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
                    `[#${currentBlockNumber}] In tanssi-solo there should be no collator assigned to orchestrator`
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
                    // Less collators than invulnerables - all collators must be invulnerables
                    for (const collator of collators) {
                        expect(
                            invulnerables.includes(collator),
                            `[#${currentBlockNumber}] Collator should be in invulnerable list: ${collator}`
                        ).to.be.true;
                    }
                } else {
                    // More collators than invulnerables: all invulnerables must be collators
                    for (const invulnerable of invulnerables) {
                        if (!collators.includes(invulnerable)) {
                            // Sometimes if the chain is de-registered, the invulnerable is not in the collators list
                            // we check if it was in the list after the full_rotation
                            await checkIfInvulnerableWasAssignedAfterFullRotation(api, invulnerable);
                        }
                    }

                    // Remaining collators must be from staking
                    const collatorsNotInvulnerables = collators.filter((collator) => !invulnerables.includes(collator));
                    for (const collator of collatorsNotInvulnerables) {
                        expect(
                            eligibleCandidates.includes(collator),
                            `[#${currentBlockNumber}] Collator should be a staking candidate: ${collator}`
                        ).to.be.true;
                    }
                }
            },
        });
    },
});

export const checkIfInvulnerableWasAssignedAfterFullRotation = async (
    api: ApiPromise,
    invulnerable: string
): Promise<void> => {
    const epochDuration = api.consts.babe.epochDuration.toNumber();
    const fullRotationSessions = 24; // for both Dancelight and Starlight

    const epochStart = (await api.query.babe.epochStart())[0].toNumber();

    for (let i = 1; i <= fullRotationSessions; i++) {
        const sessionFirstBlock = epochStart - i * epochDuration;
        if (sessionFirstBlock < 0) break;

        const blockHash = await api.rpc.chain.getBlockHash(sessionFirstBlock);
        const apiAt = await api.at(blockHash);
        const events = await apiAt.query.system.events();

        for (const { event } of events) {
            const eventJSON = event.toHuman() as {
                section: string;
                method: string;
                data: Record<string, unknown>;
            };
            if (eventJSON.section === "tanssiCollatorAssignment" && eventJSON.method === "NewPendingAssignment") {
                const fullRotation = eventJSON.data?.fullRotation;

                if (fullRotation) {
                    console.log(`Found full rotation assignment at block ${sessionFirstBlock}`);

                    let collators = [];
                    const blockHash = await api.rpc.chain.getBlockHash(sessionFirstBlock);
                    const currentBlockApi = await api.at(blockHash);
                    const currentAssignment =
                        await currentBlockApi.query.tanssiCollatorAssignment.collatorContainerChain();
                    const containerAssignment = currentAssignment.containerChains.toHuman();

                    for (const para in containerAssignment) {
                        const collatorsForPara = containerAssignment[para.toString()];
                        collators = collators.concat(collatorsForPara);
                    }

                    if (!collators.includes(invulnerable)) {
                        throw new Error(
                            `${invulnerable} not found in full-rotation assignment at block ${sessionFirstBlock}`
                        );
                    }

                    // Collator found in full-rotation assignment, all good
                    return;
                }
            }
        }
    }

    throw new Error(`No full-rotation assignment event found in the last ${fullRotationSessions} sessions.`);
};
