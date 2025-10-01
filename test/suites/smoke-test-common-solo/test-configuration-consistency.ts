import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { Vec, u32 } from "@polkadot/types-codec";
import { getBlockNumberForDebug, hasEnoughCredits, PER_BILL_RATIO } from "utils";
import type { ApiDecoration } from "@polkadot/api/types";

const getMaxPossibleAssignmentsBecauseOfCores = async (apiAtBlock: ApiDecoration<"promise">): Promise<bigint> => {
    const configuration = (await apiAtBlock.query.configuration.activeConfig()).toJSON();
    const collatorConfiguration = (await apiAtBlock.query.collatorConfiguration.activeConfig()).toJSON();
    const maxCores = (configuration.schedulerParams as { numCores: number }).numCores;

    return (BigInt(maxCores) * BigInt(collatorConfiguration.maxParachainCoresPercentage as number)) / PER_BILL_RATIO;
};

describeSuite({
    id: "SMOK04",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let blocksPerSession: bigint;
        let costPerSession: bigint;
        let costPerBlock: bigint;
        const blockNumberToDebug = getBlockNumberForDebug();
        beforeAll(async () => {
            api = context.polkadotJs();
            const chain = api.consts.system.version.specName.toString();
            blocksPerSession = chain === "dancelight" ? 600n : 3600n;
            costPerSession = BigInt((await api.call.servicesPaymentApi.collatorAssignmentCost(1000)).toString());
            costPerBlock = BigInt((await api.call.servicesPaymentApi.blockCost(1000)).toString());
        });

        it({
            id: "C01",
            title: "Config for registered paras should be consistent",
            test: async () => {
                let currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                if (blockNumberToDebug) {
                    currentBlock = blockNumberToDebug;
                }
                console.log("Current block: ", currentBlock);
                const apiAtBlock = await api.at(await api.rpc.chain.getBlockHash(currentBlock));

                const sessionIndex = (await apiAtBlock.query.session.currentIndex()).toNumber();
                const currentSessionStartBlockNumber = (await apiAtBlock.query.babe.epochStart())[1].toNumber();

                const apiBeforeLatestNewSession = await api.at(
                    await api.rpc.chain.getBlockHash(currentSessionStartBlockNumber - 1)
                );
                const config = await apiAtBlock.query.collatorConfiguration.activeConfig();

                // get pending authorities
                // the reason for getting pending is that the hasEnoughCredits check it's done over the pending ones
                const pendingAuthorityAssignment = Object.fromEntries(
                    [
                        ...(await apiAtBlock.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex + 1))
                            .unwrap()
                            .containerChains.entries(),
                    ].map(([key, value]) => [key.toString(), value.map((v) => v.toHuman())])
                );
                // get current authorities
                // we need to know whether a chain is assigned currently
                const currentAuthorityAssignment = Object.fromEntries(
                    [
                        ...(await apiAtBlock.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex))
                            .unwrap()
                            .containerChains.entries(),
                    ].map(([key, value]) => [key.toString(), value.map((v) => v.toHuman())])
                );
                const currentAuthorities = await apiAtBlock.query.session.validators();

                const currentCollatorNumber = Math.min(currentAuthorities.length, config.maxCollators.toNumber());

                const maxParas = Math.trunc(
                    (currentCollatorNumber - config.minOrchestratorCollators.toNumber()) /
                        config.collatorsPerContainer.toNumber()
                );
                const maxPossibleAssignmentsBecauseOfCores =
                    await getMaxPossibleAssignmentsBecauseOfCores(apiBeforeLatestNewSession);

                // If we have container chain collators, is because the collator number is higher
                if (maxParas > 0) {
                    let containersToCompareAgainst: Vec<u32>;
                    // If pending para ids for the session are empty we compare with registered para id, otherwise
                    // we compare with pending para ids.
                    const liveContainers = await apiAtBlock.query.containerRegistrar.registeredParaIds();
                    const pendingContainers = await apiAtBlock.query.containerRegistrar.pendingParaIds();

                    if (pendingContainers.length === 0) {
                        containersToCompareAgainst = liveContainers;
                    } else {
                        const foundEntry = pendingContainers.find((entry) => entry[0].toNumber() === sessionIndex + 1);
                        if (foundEntry) {
                            containersToCompareAgainst = foundEntry[1];
                        } else {
                            containersToCompareAgainst = liveContainers;
                        }
                    }

                    let numWithNoCredits = 0;

                    // This should be true as long as they have enough credits for getting collators
                    for (const container of containersToCompareAgainst) {
                        // if not currently assigned, then one session
                        // if currently assigned, then 2
                        let sessionRequirements: bigint;

                        if (currentAuthorityAssignment[container.toString()]?.length === 0) {
                            sessionRequirements = 1n;
                        } else {
                            sessionRequirements = 2n;
                        }
                        if (
                            await hasEnoughCredits(
                                apiBeforeLatestNewSession,
                                container,
                                blocksPerSession,
                                1n,
                                sessionRequirements,
                                costPerSession,
                                costPerBlock
                            )
                        ) {
                            // A different test checks that this number is correct with respect to configuration
                            // test-collator-number-consistency
                            // Here we only check that we have collators
                            // If we are able to cover all paras, then all of them should have collators if credits
                            if (maxParas >= containersToCompareAgainst.length) {
                                const assignments = pendingAuthorityAssignment[container.toString()];
                                // We might have the situation that we don't have enough cores to cover all the containers,
                                // In this case we won't have assignments for this container, let's confirm that
                                if (assignments === undefined) {
                                    if (
                                        BigInt(containersToCompareAgainst.length) > maxPossibleAssignmentsBecauseOfCores
                                    ) {
                                        continue;
                                    }

                                    // This should never happen, but if it does, we want to know about it
                                    throw new Error(
                                        `Unhandled exception: container ${container.toString()} has no assignment.`
                                    );
                                }

                                expect(
                                    assignments.length,
                                    `[Block: ${currentBlock}] Expect pending authority for the container: ${container.toString()} to have at least one collator, but it has ${assignments.length} collators.`
                                ).to.be.greaterThan(0);
                            }
                        } else {
                            numWithNoCredits += 1;
                        }
                    }

                    // There might be some chains that because demand is higher than offer do not get collators
                    // However we are going to check that at least the expected number of chains was assigned
                    const expectedNumberOfChainsAssigned = Math.min(
                        containersToCompareAgainst.length - numWithNoCredits,
                        maxParas,
                        Number(maxPossibleAssignmentsBecauseOfCores)
                    );
                    expect(Object.keys(pendingAuthorityAssignment).length).to.be.equal(expectedNumberOfChainsAssigned);
                }
            },
        });
    },
});
