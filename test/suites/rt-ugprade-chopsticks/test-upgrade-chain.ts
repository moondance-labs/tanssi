import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
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

      console.log("before UPGRADE");

      await context.upgradeRuntime(context);

      console.log("AFTER UPGRADE");
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
  },
});
