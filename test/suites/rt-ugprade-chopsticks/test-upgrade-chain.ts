import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import fs from "node:fs";

describeSuite({
  id: "CAN",
  title: "Chopsticks Dancebox Upgrade Test",
  foundationMethods: "chopsticks",
  testCases: function ({ it, context, log }) {
    let paraApi: ApiPromise;
    let relayApi: ApiPromise;
    let alice: KeyringPair;
    let api: ApiPromise;

    beforeAll(async () => {
      api = context.polkadotJs();

      const rtBefore = api.consts.system.version.specVersion.toNumber();
      log(`About to upgrade to runtime at:`);
      log(MoonwallContext.getContext().rtUpgradePath);

      await context.upgradeRuntime(context);

      const rtafter = api.consts.system.version.specVersion.toNumber();
      log(`RT upgrade has increased specVersion from ${rtBefore} to ${rtafter}`);

      const specName = api.consts.system.version.specName.toString();
      log(`Currently connected to chain: ${specName}`);
    });

    it({
      id: "T1",
      timeout: 60000,
      title: "Can create new blocks",
      test: async () => {
        const currentHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
        await context.createBlock({ count: 2 });
        const newHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(newHeight - currentHeight).to.be.equal(2);
      },
    });
    it({
      id: "T2",
      timeout: 60000,
      title: "Can send balance transfers",
      test: async () => {
        const randomAccount = generateKeyringPair("sr25519");
        const keyring = new Keyring({ type: "sr25519" });
        const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

        const balanceBefore = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();
        await api.tx.balances.transfer(randomAccount.address, 1_000_000_000).signAndSend(alice);
        await context.createBlock({ count: 2 });
        const balanceAfter = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();
        expect(balanceBefore < balanceAfter).to.be.true;
      },
    });
  },
});
