import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { hasEnoughCredits } from "util/payment";

describeSuite({
    id: "S04",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api;
        let blocksPerSession;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;
        const atBlockHash="0x0740fec046eccbbde9a63bb859e5a0a170fe6f5449b01074dab012f39f96eb23"

        beforeAll(async () => {
            const overallApi = context.polkadotJs();
            const chain = overallApi.consts.system.version.specName.toString();
            if (chain=="Flashbox") {
                api = await overallApi.at(atBlockHash);
            }
            else {
                api = overallApi
            }

            blocksPerSession = chain == "Dancebox" ? 300n : 5n;
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
                    const liveContainers = await api.query.registrar.registeredParaIds();

                    expect(Object.keys(authorities["containerChains"]).length).to.be.equal(liveContainers.length);

                    // This should be true as long as they have enough credits for getting collators
                    for (const container of liveContainers) {
                        // we should only check those who have enough credits
                        if (
                            await hasEnoughCredits(api, container, blocksPerSession, 2n, costPerSession, costPerBlock)
                        ) {
                            // A different test checks that this number is correct with respect to configuration
                            // test-collator-number-consistency
                            // Here we only check that  that we have collators
                            expect(authorities["containerChains"][container.toString()].length).to.be.greaterThan(0);
                        }
                    }
                }
            },
        });
    },
});
