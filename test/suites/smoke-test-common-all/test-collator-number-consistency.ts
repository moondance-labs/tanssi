import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S02",
    title: "Test collator number consistency for parathreads and parachains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;
        let chain;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
            chain = api.consts.system.version.specName.toString();
        });

        it({
            id: "C01",
            title: "Collator assignation length should be different if parachain or parathread",
            test: async function () {
                if (runtimeVersion < 500) {
                    return;
                }
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();

                const assignmentCollatorKey = (
                    chain == "dancelight"
                        ? await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                        : await api.query.authorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();

                const configuration =
                    chain == "dancelight"
                        ? await api.query.collatorConfiguration.activeConfig()
                        : await api.query.configuration.activeConfig();

                if (assignmentCollatorKey["containerChains"] != undefined) {
                    for (const container of Object.keys(assignmentCollatorKey["containerChains"])) {
                        const parathreadParams =
                            chain == "dancelight"
                                ? await api.query.containerRegistrar.parathreadParams(container)
                                : await api.query.registrar.parathreadParams(container);

                        // This is a parathread if this is Some
                        if (parathreadParams.isNone) {
                            expect(
                                assignmentCollatorKey["containerChains"][container].length,
                                `Container chain ${container} has ${assignmentCollatorKey["containerChains"][container].length} but it should have  ${configuration.collatorsPerContainer}`
                            ).toBe(configuration.collatorsPerContainer.toNumber());
                        } else {
                            expect(
                                assignmentCollatorKey["containerChains"][container].length,
                                `Parathread ${container} has ${assignmentCollatorKey["containerChains"][container].length} but it should have  ${configuration.collatorsPerParathread}`
                            ).toBe(configuration.collatorsPerParathread.toNumber());
                        }
                    }
                }
            },
        });
    },
});
