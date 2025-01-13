import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S23",
    title: "Smoke tests for validators matching external validators",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Validators should match external validators",

            test: async function () {
                const sessionValidators = await api.query.session.validators();
                const externalValidators = await api.query.externalValidators.whitelistedValidators();

                expect(sessionValidators.length).to.equal(externalValidators.length);

                for (const validatorId of sessionValidators) {
                    expect(externalValidators.includes(validatorId)).to.be.true;
                }
            },
        });
    },
});
