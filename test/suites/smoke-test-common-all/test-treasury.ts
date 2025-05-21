import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getTreasuryAddress } from "../../utils";

describeSuite({
    id: "S08",
    title: "Verify treasury consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C100",
            title: "should have value > 0",
            test: async () => {
                const treasuryAddress = getTreasuryAddress(api);
                const treasuryAccount = await api.query.system.account(treasuryAddress);

                expect(
                    treasuryAccount.data.free.toBigInt() > 0n,
                    `Free balance (${treasuryAccount.data.free.toBigInt()}) should be more than 0`
                ).to.be.true;
                expect(
                    treasuryAccount.data.reserved.toBigInt(),
                    `Reserved balance (${treasuryAccount.data.reserved.toBigInt()}) should be more than 0`
                ).to.be.equal(0n);
            },
        });
    },
});
