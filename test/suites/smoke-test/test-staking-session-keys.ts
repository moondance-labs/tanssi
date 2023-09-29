import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S07",
    title: "Check staking eligible candidates have session keys",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "All eligible candidates have session keys registered",
            test: async function () {
                if (runtimeVersion < 200) {
                    return;
                }
                const allEntries = await api.query.session.keyOwner.entries();
                const accounts = allEntries.map(([, account]) => account.toHuman());
                const keys = allEntries.map(([key]) => key.toHuman());

                const eligibleCandidates = await api.query.pooledStaking.sortedEligibleCandidates();

                for (const c of eligibleCandidates) {
                    const index = accounts.indexOf(c.candidate.toHuman());

                    expect(index, `Candidate ${c.candidate.toHuman()} should have session keys`).not.toBe(-1);

                    const allCandidateKeyTypes = keys[index].map(([keyType]) => keyType.toString());
                    expect(
                        allCandidateKeyTypes.indexOf("nmbs"),
                        `Candidate ${c.candidate.toHuman()} should have nimbus keys`
                    ).not.toBe(-1);
                }
            },
        });
    },
});
