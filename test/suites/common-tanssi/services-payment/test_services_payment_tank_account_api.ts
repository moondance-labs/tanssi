import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { paraIdTank } from "utils";
import { u8aToHex } from "@polkadot/util";

describeSuite({
    id: "COMM0206",
    title: "Services payment parachain tank account API test",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "E01",
            title: "parachain_tank_account API should return correct account for para_id",
            test: async () => {
                const paraId2000 = 2000;
                const paraId2001 = 2001;
                const paraId2004 = 2004;

                // Call the new runtime API
                const tankAccount2000 = await polkadotJs.call.servicesPaymentApi.parachainTankAccount(paraId2000);
                const tankAccount2001 = await polkadotJs.call.servicesPaymentApi.parachainTankAccount(paraId2001);
                const tankAccount2004 = await polkadotJs.call.servicesPaymentApi.parachainTankAccount(paraId2004);

                // Calculate expected tank accounts using the utility function
                const expectedTank2000 = paraIdTank(paraId2000);
                const expectedTank2001 = paraIdTank(paraId2001);
                const expectedTank2004 = paraIdTank(paraId2004);

                // Convert both to hex for comparison
                const apiResult2000 = u8aToHex(tankAccount2000);
                const apiResult2001 = u8aToHex(tankAccount2001);
                const apiResult2004 = u8aToHex(tankAccount2004);

                const expected2000 = u8aToHex(expectedTank2000);
                const expected2001 = u8aToHex(expectedTank2001);
                const expected2004 = u8aToHex(expectedTank2004);

                // Verify the API returns the same result as our calculation
                expect(apiResult2000, `Tank account for paraId ${paraId2000} should match`).toBe(expected2000);
                expect(apiResult2001, `Tank account for paraId ${paraId2001} should match`).toBe(expected2001);
                expect(apiResult2004, `Tank account for paraId ${paraId2004} should match`).toBe(expected2004);

                // For para_id 2004, verify it matches the documented expected value
                // As per the requirements: 0xd5eae3eea344c346d648beb985d6619a1589f22e1880ff35f7840aa8288e5a87
                const expectedHex2004 =
                    "0xd5eae3eea344c346d648beb985d6619a1589f22e1880ff35f7840aa8288e5a87";
                expect(apiResult2004, "Tank account for paraId 2004 should match the documented value").toBe(
                    expectedHex2004
                );
            },
        });
    },
});

