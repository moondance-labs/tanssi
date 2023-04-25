import { expect, describeSuite, beforeAll, ApiPromise, MoonwallContext } from "@moonwall/cli";
import fs from "node:fs";
import { BALTATHAR_ADDRESS, charleth } from "@moonwall/util";

describeSuite({
  id: "ZTN",
  title: "Zombie Tanssi Test",
  foundationMethods: "zombie",
  testCases: function ({ it, context, log }) {
    let paraApi: ApiPromise;
    let relayApi: ApiPromise;

    beforeAll(async () => {
      paraApi = context.polkadotJs({ type: "polkadotJs" });
      relayApi = context.polkadotJs({ type: "polkadotJs" });

      const relayNetwork = relayApi.consts.system.version.specName.toString();
      expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

      const paraNetwork = paraApi.consts.system.version.specName.toString();
      expect(paraNetwork, "Para API incorrect").to.contain("moonbase");

      const currentBlock = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
      expect(currentBlock, "Parachain not producing blocks").to.be.greaterThan(0);
    }, 120000);

    it({
      id: "T01",
      title: "Blocks are being produced on parachain",
      test: async function () {
       // const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
       // expect(blockNum).to.be.greaterThan(0);
      },
    });
  },
});