import { beforeEach, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, createRawTransfer, sendRawTransaction } from "@moonwall/util";
import { parseGwei } from "viem";
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";

describeSuite({
  id: "D1105",
  title: "Resubmit transations",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let randomAddress: `0x${string}`;
    let currentNonce: number;

    beforeEach(async function () {
      randomAddress = privateKeyToAccount(generatePrivateKey()).address;
      currentNonce = await context
        .viemClient("public")
        .getTransactionCount({ address: ALITH_ADDRESS });
    });
    it({
      id: "T01",
      title: "should allow resubmitting with higher gas",
      test: async function () {
        await context.createBlock([
          await createRawTransfer(context, randomAddress, 1_000_000_000_000, {
            nonce: currentNonce,
            maxFeePerGas: parseGwei("10"),
          }),
          await createRawTransfer(context, randomAddress, 3_000_000_000_000, {
            nonce: currentNonce,
            maxFeePerGas: parseGwei("20"),
            maxPriorityFeePerGas: parseGwei("10"),
          }),
        ]);

        expect(await context.viemClient("public").getBalance({ address: randomAddress })).to.equal(
          3_000_000_000_000n
        );
      },
    });
  },
});