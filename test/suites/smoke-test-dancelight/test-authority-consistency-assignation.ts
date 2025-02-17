import "@tanssi/api-augment/dancelight";
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
                const assignmentCollatorAccount = await api.query.tanssiCollatorAssignment.collatorContainerChain();
                const sessionIndex = (await api.query.session.currentIndex()).toNumber();
                const assignmentCollatorKey = (
                    await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                ).unwrap();
                const authorityKeyMapping = (
                    await api.query.tanssiAuthorityMapping.authorityIdMapping(sessionIndex)
                ).unwrap();

                const remappedAssignmentCollatorAccount = Object.fromEntries(
                    [...assignmentCollatorAccount.containerChains.entries()].map(([key, value]) => [
                        key.toString(),
                        value.map((v) => v.toHuman()),
                    ])
                );

                const remappedAuthorityKeyMapping = Object.fromEntries(
                    [...authorityKeyMapping.entries()].map(([key, value]) => [key.toHex(), value.toHuman()])
                );

                const containerChains = assignmentCollatorKey.containerChains.entries();

                for (const [container, authorities] of containerChains) {
                    for (const authority of authorities) {
                        const assignedAccount = remappedAuthorityKeyMapping[authority.toHex()];
                        expect(remappedAssignmentCollatorAccount[container.toString()]).toContain(assignedAccount);
                    }
                }
            },
        });
    },
});
