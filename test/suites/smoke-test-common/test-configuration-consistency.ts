import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S04",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
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
                // get current authorities
                const authorities = (await api.query.authorityAssignment.collatorContainerChain(sessionIndex)).toJSON();

                // If we have container chain collators, is because we at least assigned min to orchestrator
                if (
                    Object.keys(authorities["orchestratorChain"]).length > config["minOrchestratorCollators"].toNumber()
                ) {
                    const liveContainers = await api.query.registrar.registeredParaIds();

                    expect(Object.keys(authorities["containerChains"]).length).to.be.equal(liveContainers.length);

                    for (const container of liveContainers) {
                        expect(authorities["containerChains"][container.toString()].length).to.be.equal(
                            config["collatorsPerContainer"].toNumber()
                        );
                    }
                }
            },
        });
    },
});
