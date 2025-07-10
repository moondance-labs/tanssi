import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { Vec, u32 } from "@polkadot/types-codec";
import { hasEnoughCredits } from "utils";

describeSuite({
    id: "SMOK04",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        const blocksPerSession = 600n;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Config for registered paras should be consistent",
            test: async () => {
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                const blockToCheck = (await api.query.babe.epochStart())[1].toNumber();

                const apiBeforeLatestNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));
                const config = await api.query.collatorConfiguration.activeConfig();

                // get pending authorities
                // the reason for getting pending is that the hasEnoughCredits check it's done over the pending ones
                const pendingAuthorityAssignment = Object.fromEntries(
                    [
                        ...(await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex + 1))
                            .unwrap()
                            .containerChains.entries(),
                    ].map(([key, value]) => [key.toString(), value.map((v) => v.toHuman())])
                );
                // get current authorities
                // we need to know whether a chain is assigned currently
                const currentAuthorityAssignment = Object.fromEntries(
                    [
                        ...(await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex))
                            .unwrap()
                            .containerChains.entries(),
                    ].map(([key, value]) => [key.toString(), value.map((v) => v.toHuman())])
                );
                const currentAuthorities = await api.query.session.validators();

                const currentCollatorNumber = Math.min(currentAuthorities.length, config.maxCollators.toNumber());

                const maxParas = Math.trunc(
                    (currentCollatorNumber - config.minOrchestratorCollators.toNumber()) /
                        config.collatorsPerContainer.toNumber()
                );

                // If we have container chain collators, is because the collator number is higher
                if (maxParas > 0) {
                    let containersToCompareAgainst: Vec<u32>;
                    // If pending para ids for the session are empty we compare with registered para id, otherwise
                    // we compare with pending para ids.
                    const liveContainers = await api.query.containerRegistrar.registeredParaIds();
                    const pendingContainers = await api.query.containerRegistrar.pendingParaIds();

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
                            // Here we only check that  that we have collators
                            // If we are able to cover all paras, then all of them should have collators if credits
                            if (maxParas >= containersToCompareAgainst.length) {
                                expect(pendingAuthorityAssignment[container.toString()].length).to.be.greaterThan(0);
                            }
                        } else {
                            numWithNoCredits += 1;
                        }
                    }

                    // There might be some chains that because demand is higher than offer do not get collators
                    // However we are going to check that at least the expected number of chains was assigned
                    const expectedNumberOfChainsAssigned = Math.min(
                        containersToCompareAgainst.length - numWithNoCredits,
                        maxParas
                    );
                    expect(Object.keys(pendingAuthorityAssignment).length).to.be.equal(expectedNumberOfChainsAssigned);
                }
            },
        });
    },
});
