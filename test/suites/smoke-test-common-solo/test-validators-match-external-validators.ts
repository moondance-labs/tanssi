import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getCurrentEraStartBlock } from "utils";

describeSuite({
    id: "SMOK08",
    title: "Smoke tests for validators matching external validators",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Validators should match external validators",

            test: async () => {
                const blockToCheck = await getCurrentEraStartBlock(api);

                const externalValidatorsList = (
                    await (
                        await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1))
                    ).query.externalValidators.externalValidators()
                ).map((validator) => validator.toHuman());
                const whitelistedValidatorsList = (
                    await (
                        await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1))
                    ).query.externalValidators.whitelistedValidators()
                ).map((validator) => validator.toHuman());

                const sessionValidators = (await api.query.session.validators()).map((acc) => acc.toHuman());
                const externalValidators = [...externalValidatorsList, ...whitelistedValidatorsList];

                if (externalValidators.length <= sessionValidators.length) {
                    const failures = externalValidators.filter(
                        (externalValidator) => !sessionValidators.includes(externalValidator)
                    );

                    if (failures.length > 0) {
                        for (const failure of failures) {
                            log(`External validator ${failure} should be in session validators list`);
                        }

                        log(`Session validators list: [ ${sessionValidators.join(", ")} ]`);
                    }

                    expect(
                        failures.length,
                        "Equal/Fewer ext than session validators: all external validators must be session validators"
                    ).toBe(0);
                } else {
                    const failures = sessionValidators.filter(
                        (sessionValidator) => !externalValidators.includes(sessionValidator)
                    );

                    if (failures.length > 0) {
                        for (const failure of failures) {
                            log(`Session validator ${failure} should be in external validators list`);
                        }

                        log(`External validators list: [ ${externalValidators.join(", ")} ]`);
                    }

                    expect(
                        failures.length,
                        "More ext than session validators: all session validators must be external validators"
                    ).toBe(0);
                }
            },
        });
    },
});
