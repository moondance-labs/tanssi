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
                const externalValidators = await api.query.externalValidators.whitelistedValidators<Vec<AccountId32>>();

                expect(sessionValidators.eq(externalValidators.map((v) => v.toHex()))).to.be.eq(
                    true,
                    `Validators ${sessionValidators.toString()} are inconsistent with external validators ${externalValidators.toString()}`
                );
            },
        });
    },
});
