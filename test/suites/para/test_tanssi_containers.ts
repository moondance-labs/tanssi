import { expect, describeSuite, beforeAll, ApiPromise } from "@moonwall/cli";
describeSuite({
  id: "ZTN",
  title: "Zombie Tanssi Test",
  foundationMethods: "zombie",
  testCases: function ({ it, context, log }) {
    let paraApi: ApiPromise;
    let relayApi: ApiPromise;
    let container2000Api: ApiPromise;
    let container2001Api: ApiPromise;

    beforeAll(async () => {
      
      paraApi = context.polkadotJs({ apiName: "Tanssi", type: "polkadotJs" });
      relayApi = context.polkadotJs({ apiName: "Relay", type: "polkadotJs" });
      container2000Api = context.polkadotJs({ apiName: "Container2000", type: "polkadotJs" });
      container2001Api = context.polkadotJs({ apiName: "Container2001", type: "polkadotJs" });

      const relayNetwork = relayApi.consts.system.version.specName.toString();
      expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

      const paraNetwork = paraApi.consts.system.version.specName.toString();
      expect(paraNetwork, "Para API incorrect").to.contain("orchestrator-template-parachain");

      const container2000Network = container2000Api.consts.system.version.specName.toString();
      expect(container2000Network, "Container2000 API incorrect").to.contain("template-parachain");

      const container2001Network = container2001Api.consts.system.version.specName.toString();
      expect(container2001Network, "Container2001 API incorrect").to.contain("template-parachain");

    }, 120000);

    it({
      id: "T01",
      title: "Blocks are being produced on parachain",
      test: async function () {
        const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T02",
      title: "Blocks are being produced on container 2000",
      test: async function () {
        const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T03",
      title: "Blocks are being produced on container 2001",
      test: async function () {
        const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

   
    it({
      id: "T04",
      title: "Test assignation is correct",
      test: async function () {
        const tanssiCollators = (await paraApi.query.collatorAssignment.collatorContainerChain()).orchestratorChain.map((v): string =>
        v.toString()
        );
        const authorities = (await paraApi.query.aura.authorities());

        let getKeyOwnersFromAuthorities = [];

        for (var authority of authorities) {
          const owner = (await paraApi.query.session.keyOwner([
            "aura",
             authority
          ]
          ));
          getKeyOwnersFromAuthorities.push(owner.toString());
        }

        for (let i = 0; i < tanssiCollators.length; i++) {
          expect(tanssiCollators[i]).to.be.equal(getKeyOwnersFromAuthorities[i]);
        }
      },
    });

  },
});
