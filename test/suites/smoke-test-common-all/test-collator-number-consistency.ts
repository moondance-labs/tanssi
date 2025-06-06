import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { isLightRuntime } from "../../utils/runtime.ts";

describeSuite({
    id: "S02",
    title: "Test collator number consistency for parathreads and parachains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion: number;
        let chain: any;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
            chain = api.consts.system.version.specName.toString();
        });

        it({
            id: "C01",
            title: "Collator assignation length should be different if parachain or parathread",
            test: async () => {
                if (runtimeVersion < 500) {
                    return;
                }
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();

                const assignmentCollatorKey = (
                    isLightRuntime(api)
                        ? await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                        : await api.query.authorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();

                const configuration = isLightRuntime(api)
                    ? await api.query.collatorConfiguration.activeConfig()
                    : await api.query.configuration.activeConfig();

                if (assignmentCollatorKey.containerChains !== undefined) {
                    for (const container of Object.keys(assignmentCollatorKey.containerChains)) {
                        const parathreadParams = isLightRuntime(api)
                            ? await api.query.containerRegistrar.parathreadParams(container)
                            : await api.query.registrar.parathreadParams(container);

                        // This is a parathread if this is Some
                        if (parathreadParams.isNone) {
                            expect(
                                assignmentCollatorKey.containerChains[container].length,
                                `Container chain ${container} has ${assignmentCollatorKey.containerChains[container].length} but it should have  ${configuration.collatorsPerContainer}`
                            ).toBe(configuration.collatorsPerContainer.toNumber());
                        } else {
                            expect(
                                assignmentCollatorKey.containerChains[container].length,
                                `Parathread ${container} has ${assignmentCollatorKey.containerChains[container].length} but it should have  ${configuration.collatorsPerParathread}`
                            ).toBe(configuration.collatorsPerParathread.toNumber());
                        }
                    }
                }
            },
        });
    },
});
