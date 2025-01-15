import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { Vec } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces";

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
                const externalValidators = await api.query.externalValidators.externalValidators<Vec<AccountId32>>();

                if (externalValidators.length <= sessionValidators.length) {
                    // Less external validators than session validators: all external validators must be session validators
                    for (const externalValidator of externalValidators) {
                        expect(
                            sessionValidators.toString().includes(externalValidator.toString()),
                            `External validator should be in validators list: ${externalValidator.toString()}`
                        ).to.be.true;
                    }
                } else {
                    // More external validators than session validators: all session validators must be external validators
                    for (const validator of sessionValidators) {
                        expect(
                            externalValidators.toString().includes(validator.toString()),
                            `Validator should be in external validators list: ${validator.toString()}`
                        ).to.be.true;
                    }
                }
            },
        });
    },
});
