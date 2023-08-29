import { describeSuite, extractInfo, expect } from "@moonwall/cli";
import { BALTATHAR_ADDRESS, GLMR, createRawTransfer } from "@moonwall/util";

// We use ethers library in this test as apparently web3js's types are not fully EIP-1559
// compliant yet.
describeSuite({
    id: "DF0302",
    title: "Ethereum - PaysFee",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: `T01`,
            title: `should be false for successful ethereum transactions`,
            test: async function () {
                const { result } = await context.createBlock(await createRawTransfer(context, BALTATHAR_ADDRESS, GLMR));
                const info = extractInfo(result!.events)!;
                expect(info).to.not.be.empty;
                expect(info.paysFee.isYes, "Transaction should be marked as paysFees == no").to.be.false;
            },
        });
    },
});
