import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK01",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
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

                const failures = containerChains
                    .filter(([container, authorities]) => {
                        const assignedCollators = remappedAssignmentCollatorAccount[container.toString()];
                        return authorities.every(
                            (authority) => !assignedCollators.includes(remappedAuthorityKeyMapping[authority.toHex()])
                        );
                    })
                    .toArray();

                if (failures.length > 0) {
                    for (const [container, authorities] of failures) {
                        log(`❌ Session ${sessionIndex} - Error with container:  ${container}. Authorities:`);
                        log(authorities.map((authority) => authority.toHuman()).join(", "));
                        log("Tanssi Collators assigned:");
                        log(remappedAssignmentCollatorAccount[container.toString()].join(", "));
                    }
                    log(`Tanssi Authorities assigned for session ${sessionIndex}:`);
                    log((await api.query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)).toHuman());
                }

                expect(failures.length).toBe(0);
            },
        });
    },
});
