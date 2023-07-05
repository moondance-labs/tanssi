import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { formatEther } from "ethers";
import { BN } from "@polkadot/util";
import { ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
  id: "D01",
  title: "Dev test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob;
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      polkadotJs = context.polkadotJs();
    });

    it({
      id: "E01",
      title: "Checking that launched node can create blocks",
      test: async function () {
        const block = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
        await context.createBlock();

        const block2 = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
        log(`Original block #${block}, new block #${block2}`);
        expect(block2).to.be.greaterThan(block);
      },
    });
  },
});