import { customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";

describeSuite({
  id: "DF0901",
  title: "Frontier RPC Methods - frnt_isBlockFinalized ",
  foundationMethods: "dev",
  testCases: ({ context, it }) => {
    it({
      id: "T01",
      title: "should return as finalized when true",
      test: async function () {
        const blockHash = (await context.createBlock([], { finalize: true })).block.hash;
        const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
        expect(resp, "Block finalization status mismatch").toBe(true);
      },
    });

    it({
      id: "T02",
      title: "should return as unfinalized when false",
      test: async function () {
        const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
        const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
        expect(resp, "Block finalization status mismatch").toBe(false);
      },
    });

    it({
      id: "T03",
      title: "should return as unfinalized when block not found",
      test: async function () {
        const blockHash = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
        expect(resp, "Block finalization status mismatch").toBe(false);
      },
    });

    it({
      id: "T04",
      title: "should return as finalized when new block is true",
      test: async function () {
        const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
        await context.createBlock([], { finalize: true });
        const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
        expect(resp, "Block finalization status mismatch").toBe(true);
      },
    });

    it({
      id: "T05",
      title: "should return as finalized when new block reorg happens",
      test: async function () {
        const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
        await context.createBlock([], { finalize: false });
        await context.createBlock([], { finalize: true, parentHash: blockHash });

        const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
        expect(resp, "Block finalization status mismatch").toBe(true);
      },
    });
  },
});
