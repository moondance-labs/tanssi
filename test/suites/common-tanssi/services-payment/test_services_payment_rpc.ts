import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect, customDevRpcRequest } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "CT0609",
    title: "Stream payment RPC",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "E01",
            title: "Stream payment RPC",
            test: async function () {
                try {
                    await customDevRpcRequest("tanssi_servicesPaymentBlockCost", []);
                    throw { message: "Should have returned an error" };
                } catch (e: any) {
                    expect(e.message.toString()).to.eq("No more params");
                }

                expect(await customDevRpcRequest("tanssi_servicesPaymentBlockCost", [1000])).eq(1000000);
                expect(await customDevRpcRequest("tanssi_servicesPaymentCollatorAssignmentCost", [1000])).eq(100000000);
            },
        });
    },
});
