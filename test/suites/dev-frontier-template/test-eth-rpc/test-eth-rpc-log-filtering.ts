import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { deployCreateCompiledContract } from "@moonwall/util";
import { TransactionReceipt } from "viem";
import { customWeb3Request } from "@moonwall/util";

describeSuite({
  id: "D1203",
  title: "Ethereum RPC - Filtering non-matching logs",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let nonMatchingCases: ReturnType<typeof getNonMatchingCases>;

    const getNonMatchingCases = (receipt: TransactionReceipt) => {
      return [
        // Non-existant address.
        {
          fromBlock: "0x0",
          toBlock: "latest",
          address: "0x0000000000000000000000000000000000000000",
        },
        // Non-existant topic.
        {
          fromBlock: "0x0",
          toBlock: "latest",
          topics: ["0x0000000000000000000000000000000000000000000000000000000000000000"],
        },
        // Existant address + non-existant topic.
        {
          fromBlock: "0x0",
          toBlock: "latest",
          address: receipt.contractAddress,
          topics: ["0x0000000000000000000000000000000000000000000000000000000000000000"],
        },
        // Non-existant address + existant topic.
        {
          fromBlock: "0x0",
          toBlock: "latest",
          address: "0x0000000000000000000000000000000000000000",
          topics: receipt.logs[0].topics,
        },
      ];
    };

    beforeAll(async () => {
      const { hash } = await deployCreateCompiledContract(context, "EventEmitter");
      const receipt = await context.viemClient("public").getTransactionReceipt({ hash });
      nonMatchingCases = getNonMatchingCases(receipt);
    });

    it({
      id: "T01",
      title: "EthFilterApi::getFilterLogs - should filter out non-matching cases.",
      test: async function () {
        let create_filter;
        for (var item of nonMatchingCases) {
            create_filter = await customWeb3Request(context.web3(), "eth_newFilter", [item]);
            let poll = await customWeb3Request(context.web3(), "eth_getFilterLogs", [create_filter.result]);
            expect(poll.result.length).to.be.eq(0);
        }
      },
    });
    it({
      id: "T02",
      title: "EthApi::getLogs - should filter out non-matching cases.",
      test: async function () {
        for (var item of nonMatchingCases) {
            let request = await customWeb3Request(context.web3(), "eth_getLogs", [item]);
            expect(request.result.length).to.be.eq(0);
        }
      },
    });
  },
});