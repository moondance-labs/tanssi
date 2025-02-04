import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import type { ApiPromise } from "@polkadot/api";
import { hasEnoughCredits } from "util/payment";
import type { u32, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "S08",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let blocksPerSession;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;

        beforeAll(() => {
            api = context.polkadotJs();
            const chain = api.consts.system.version.specName.toString();
            blocksPerSession = chain === "dancebox" ? 600n : 50n;
        });

        it({
            id: "C01",
            title: "Config orchestrator max collators parameters should be respected",
            test: async () => {
                const config = await api.query.configuration.activeConfig();
                // get current session
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                // get current authorities
                const authorities = await api.query.authorityAssignment.collatorContainerChain(sessionIndex);

                // We cannot exced max collators
                expect(authorities.toJSON()["orchestratorChain"].length).to.be.lessThanOrEqual(
                    config["maxOrchestratorCollators"].toNumber()
                );
            },
        });

        it({
            id: "C02",
            title: "Config orchestrator min collators parameters should be respected",
            test: async () => {
                const config = await api.query.configuration.activeConfig();
                // get current session
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                // get current authorities
                const authorities = (await api.query.authorityAssignment.collatorContainerChain(sessionIndex)).toJSON();

                // If we have container chain collators, is because we at least assigned min to orchestrator
                if (Object.keys(authorities["containerChains"]).length != 0) {
                    expect(authorities["orchestratorChain"].length).to.be.greaterThanOrEqual(
                        config["minOrchestratorCollators"].toNumber()
                    );
                }
            },
        });

        it({
            id: "C03",
            title: "Config registered paras should be filled if more than min collators in orchestrator",
            test: async () => {
                const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();

                const blockToCheck = Math.trunc(currentBlock / Number(blocksPerSession)) * Number(blocksPerSession);
                const apiBeforeLatestNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));

                const config = await api.query.configuration.activeConfig();
                // get current session
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                // get pending authorities
                // the reason for getting pending is that the hasEnoughCredits check it's done over the pending ones
                const pendingAuthorityAssignment = (
                    await api.query.authorityAssignment.collatorContainerChain(sessionIndex + 1)
                ).toJSON();

                // get current authorities
                // we need to know whether a chain is assigned currently
                const currentAuthorityAssignment = (
                    await api.query.authorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();

                const currentAuthorities = await api.query.session.validators();

                const currentCollatorNumber = Math.min(currentAuthorities.length, config.maxCollators);

                const maxParas = Math.trunc(
                    (currentCollatorNumber - config.minOrchestratorCollators) / config.collatorsPerContainer
                );

                // If we have container chain collators, is because the collator number is higher
                if (maxParas > 0) {
                    let containersToCompareAgainst: Vec<u32>;
                    // If pending para ids for the session are empty we compare with registered para id, otherwise
                    // we compare with pending para ids.
                    const liveContainers = await api.query.registrar.registeredParaIds();
                    const pendingContainers = await api.query.registrar.pendingParaIds();

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

                        if (
                            currentAuthorityAssignment["containerChains"][container.toString()] === null ||
                            currentAuthorityAssignment["containerChains"][container.toString()].length === 0
                        ) {
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
                                expect(
                                    pendingAuthorityAssignment["containerChains"][container.toString()].length
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
                        maxParas
                    );
                    expect(Object.keys(pendingAuthorityAssignment["containerChains"]).length).to.be.equal(
                        expectedNumberOfChainsAssigned
                    );
                }
            },
        });
    },
});
