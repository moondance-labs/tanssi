import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S01",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Collator assignation and authority assignation should match with observed mapping in orchestrator",
            test: async function () {
                const assignmentCollatorAccount = (
                    await api.query.collatorAssignment.collatorContainerChain()
                ).toJSON();
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();

                const assignmentCollatorKey = (
                    await api.query.authorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();
                const authorityKeyMapping = (
                    await api.query.authorityMapping.authorityIdMapping(sessionIndex)
                ).toJSON();
                for (const key of assignmentCollatorKey["orchestratorChain"]) {
                    const assignedAccount = authorityKeyMapping[key.toString()];
                    expect(assignmentCollatorAccount["orchestratorChain"].includes(assignedAccount.toString())).to.be
                        .true;
                }
            },
        });

        it({
            id: "C02",
            title: "Collator assignation and authority assignation should match with observed mapping in containers",
            test: async function () {
                const assignmentCollatorAccount = (
                    await api.query.collatorAssignment.collatorContainerChain()
                ).toJSON();
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                const assignmentCollatorKey = (
                    await api.query.authorityAssignment.collatorContainerChain(sessionIndex)
                ).toJSON();
                const authorityKeyMapping = (
                    await api.query.authorityMapping.authorityIdMapping(sessionIndex)
                ).toJSON();
                for (const container of Object.keys(assignmentCollatorKey["containerChains"])) {
                    for (const key of assignmentCollatorKey["containerChains"][container]) {
                        const assignedAccount = authorityKeyMapping[key.toString()];
                        expect(
                            assignmentCollatorAccount["containerChains"][container].includes(assignedAccount.toString())
                        ).to.be.true;
                    }
                }
            },
        });
    },
});
