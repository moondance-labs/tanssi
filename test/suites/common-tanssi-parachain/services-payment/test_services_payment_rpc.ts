import "@tanssi/api-augment";
import { describeSuite, expect, customDevRpcRequest } from "@moonwall/cli";

describeSuite({
    id: "CPT0609",
    title: "Services payment RPC",
    foundationMethods: "dev",
    testCases: ({ it }) => {
        it({
            id: "E01",
            title: "Services payment RPC",
            test: async () => {
                try {
                    await customDevRpcRequest("tanssi_servicesPaymentBlockCost", []);
                    throw { message: "Should have returned an error" };
                } catch (e: any) {
                    expect(e.message.toString()).to.eq("Invalid params");
                }

                expect(await customDevRpcRequest("tanssi_servicesPaymentBlockCost", [1000])).eq(1000000);
                expect(await customDevRpcRequest("tanssi_servicesPaymentCollatorAssignmentCost", [1000])).eq(100000000);
            },
        });
    },
});
