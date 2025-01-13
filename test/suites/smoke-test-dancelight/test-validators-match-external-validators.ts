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

                const validators = await api.query.staking.validators.entries();
                const externalValidators = await api.query.externalValidators.externalValidators.entries();

                for (const key of validators.keys()) {
                    const validator = externalValidators.find(([validatorKey]) => validatorKey.eq(key));
                    expect(validator).to.not.be.undefined;
                }
            },
        });
    },
});
