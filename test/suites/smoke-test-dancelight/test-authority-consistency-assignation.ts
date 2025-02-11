import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK01",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Collator assignation and authority assignation should match with observed mapping in containers",
            test: async () => {
                const assignmentCollatorAccount = (
                    await api.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                const assignmentCollatorKey = (
                    await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();
                const authorityKeyMapping = (
                    await api.query.tanssiAuthorityMapping.authorityIdMapping(sessionIndex)
                ).toJSON();
                for (const container of Object.keys(assignmentCollatorKey.containerChains)) {
                    for (const key of assignmentCollatorKey.containerChains[container]) {
                        const assignedAccount = authorityKeyMapping[key.toString()];
                        expect(
                            assignmentCollatorAccount.containerChains[container].includes(assignedAccount.toString())
                        ).to.be.true;
                    }
                }
            },
        });
    },
});
