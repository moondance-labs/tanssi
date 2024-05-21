import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { hasEnoughCredits } from "util/payment";
import { u32, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "S04",
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
            blocksPerSession = chain == "Dancebox" ? 600n : 50n;
        });

        it({
            id: "C01",
            title: "Config orchestrator max collators parameters should be respected",
            test: async function () {
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
            test: async function () {
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
            title: "Config all registered paras should be filled if more than min collators in orchestrator",
            test: async function () {
                const config = await api.query.configuration.activeConfig();
                // get current session
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                // get pending authorities
                // the reason for getting pending is that the hasEnoughCredits check it's done over the pending ones
                const authorities = (
                    await api.query.authorityAssignment.collatorContainerChain(sessionIndex + 1)
                ).toJSON();

                // If we have container chain collators, is because we at least assigned min to orchestrator
                if (
                    Object.keys(authorities["orchestratorChain"]).length > config["minOrchestratorCollators"].toNumber()
                ) {
                    let containersToCompareAgainst: Vec<u32>;
                    // If pending para ids for the session are empty we compare with registered para id, otherwise
                    // we compare with pending para ids.
                    const liveContainers = await api.query.registrar.registeredParaIds();
                    const pendingContainers = await api.query.registrar.pendingParaIds();

                    if (pendingContainers.length == 0) {
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
                        // we should only check those who have enough credits
                        if (
                            await hasEnoughCredits(api, container, blocksPerSession, 2n, costPerSession, costPerBlock)
                        ) {
                            // A different test checks that this number is correct with respect to configuration
                            // test-collator-number-consistency
                            // Here we only check that  that we have collators
                            expect(authorities["containerChains"][container.toString()].length).to.be.greaterThan(0);
                        } else {
                            numWithNoCredits += 1;
                        }
                    }

                    expect(Object.keys(authorities["containerChains"]).length).to.be.equal(
                        containersToCompareAgainst.length - numWithNoCredits
                    );
                }
            },
        });
    },
});
