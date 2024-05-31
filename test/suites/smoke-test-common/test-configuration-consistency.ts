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
            blocksPerSession = chain == "dancebox" ? 600n : 50n;
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
            title: "Config registered paras should be filled if more than min collators in orchestrator",
            test: async function () {
                const blockApi = await api.at("0x3f11946482210b71bf9a2689ffa8704b2d2b030503f22a1d4b3158a899423a53");
                const currentBlock = (await api.rpc.chain.getBlock("0x3f11946482210b71bf9a2689ffa8704b2d2b030503f22a1d4b3158a899423a53")).block.header.number.toNumber();

                const blockToCheck = Math.trunc(currentBlock / Number(blocksPerSession)) * Number(blocksPerSession);
                const apiBeforeLatestNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));

                const config = await blockApi.query.configuration.activeConfig();
                // get current session
                const sessionIndex = (await blockApi.query.session.currentIndex()).toNumber();
                // get pending authorities
                // the reason for getting pending is that the hasEnoughCredits check it's done over the pending ones
                const authorities = (
                    await blockApi.query.authorityAssignment.collatorContainerChain(sessionIndex + 1)
                ).toJSON();

                const currentAuthorities = await blockApi.query.session.validators();

                const currentCollatorNumber = Math.min(currentAuthorities.length, config.maxCollators);

                console.log(currentCollatorNumber, config.minOrchestratorCollators.toNumber(), config.collatorsPerContainer.toNumber());

                const maxParas = Math.trunc(
                    (currentCollatorNumber - config.minOrchestratorCollators) / config.collatorsPerContainer
                );

                console.log("Max paras", maxParas);

                // If we have container chain collators, is because the collator number is higher
                if (maxParas > 0) {
                    let containersToCompareAgainst: Vec<u32>;
                    // If pending para ids for the session are empty we compare with registered para id, otherwise
                    // we compare with pending para ids.
                    const liveContainers = await blockApi.query.registrar.registeredParaIds();
                    const pendingContainers = await blockApi.query.registrar.pendingParaIds();

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

                    console.log(containersToCompareAgainst.toJSON());

                    let numWithNoCredits = 0;

                    // This should be true as long as they have enough credits for getting collators
                    for (const container of containersToCompareAgainst) {
                        // we should only check those who have enough credits
                        // we compare against the credits they had just before new session obviously
                        // that is when they were charged for tokens
                        if (
                            await hasEnoughCredits(
                                apiBeforeLatestNewSession,
                                container,
                                blocksPerSession,
                                1n,
                                2n,
                                costPerSession,
                                costPerBlock
                            )
                        ) {
                            // A different test checks that this number is correct with respect to configuration
                            // test-collator-number-consistency
                            // Here we only check that  that we have collators
                            // If we are able to cover all paras, then all of them should have collators if credits
                            if (maxParas >= containersToCompareAgainst.length) {
                                expect(authorities["containerChains"][container.toString()].length).to.be.greaterThan(
                                    0
                                );
                            }
                        } else {
                            console.log(container.toString(), " has no credits");
                            numWithNoCredits += 1;
                        }
                    }

                    // There might be some chains that because demand is higher than offer do not get collators
                    // However we are going to check that at least the expected number of chains was assigned
                    const expectedNumberOfChainsAssigned = Math.min(
                        containersToCompareAgainst.length - numWithNoCredits,
                        maxParas
                    );

                    console.log(Object.keys(authorities["containerChains"]));

                    expect(Object.keys(authorities["containerChains"]).length).to.be.equal(
                        expectedNumberOfChainsAssigned
                    );
                }
            },
        });
    },
});
