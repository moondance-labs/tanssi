import { describeSuite, expect, beforeAll, Web3, Signer } from "@moonwall/cli";
import { CHARLETH_ADDRESS, BALTATHAR_ADDRESS, alith, baltathar, setupLogger,generateKeyringPair } from "@moonwall/util";
import { WebSocketProvider, parseEther, formatEther } from "ethers";
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
      polkadotJs = context.polkadotJs();
      const chain = polkadotJs.consts.system.version.specName.toString();
      alice = chain == 'frontier-template' ? alith : (new Keyring({ type: 'sr25519' }).addFromUri('//Alice', { name: 'Alice default' }));
      bob = chain == 'frontier-template' ? baltathar : (new Keyring({ type: 'sr25519' }).addFromUri('//Bob', { name: 'Bob default' }));
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

    it({
      id: "E02",
      title: "Checking that substrate txns possible",
      timeout: 20000,
      test: async function () {
        const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;

        await polkadotJs.tx.balances
          .transfer(bob.address, 1000)
          .signAndSend(alice);

        await context.createBlock();
        const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;
        
        expect(balanceBefore.lt(balanceAfter)).to.be.true;
      },
    });

    it({
      id: "E03",
      title: "Checking that sudo can be used",
      test: async function () {
        await context.createBlock();
        const tx = polkadotJs.tx.rootTesting.fillBlock(60 * 10 ** 7);
        await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);

        await context.createBlock();
        const blockFill = await polkadotJs.query.system.blockWeight();
        expect(blockFill.normal.refTime.unwrap().gt(new BN(0))).to.be.true;
      },
    });
  },
});