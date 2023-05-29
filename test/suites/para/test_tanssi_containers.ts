import { expect, describeSuite, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { getHeaderFromRelay } from "../../util/relayInterface";
import { getAuthorFromDigest } from "../../util/author";

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
      
      paraApi = context.polkadotJs({ apiName: "Tanssi" });
      relayApi = context.polkadotJs({ apiName: "Relay" });
      container2000Api = context.polkadotJs({ apiName: "Container2000" });
      container2001Api = context.polkadotJs({ apiName: "Container2001" });

      const relayNetwork = relayApi.consts.system.version.specName.toString();
      expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

      const paraNetwork = paraApi.consts.system.version.specName.toString();
      const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
      expect(paraNetwork, "Para API incorrect").to.contain("orchestrator-template-parachain");
      expect(paraId1000, "Para API incorrect").to.be.equal("1000");

      const container2000Network = container2000Api.consts.system.version.specName.toString();
      const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
      expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
      expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

      const container2001Network = container2001Api.consts.system.version.specName.toString();
      const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
      expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
      expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

      // Test block numbers in relay are 0 yet
      const header2000 = await getHeaderFromRelay(relayApi, 2000);
      const header2001 = await getHeaderFromRelay(relayApi, 2001);

      expect(header2000.number.toNumber()).to.be.equal(0);
      expect(header2001.number.toNumber()).to.be.equal(0);

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
      title: "Test Tanssi assignation is correct",
      test: async function () {
        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        expect(currentSession).to.be.equal(0);
        const tanssiCollators = (await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)).toJSON().orchestratorChain;
        const authorities = (await paraApi.query.aura.authorities()).toJSON();

        expect(tanssiCollators).to.deep.equal(authorities);
      },
    });

    it({
      id: "T03",
      title: "Test assignation did not change",
      test: async function () {
        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        expect(currentSession).to.be.equal(0);
        const allCollators = (await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)).toJSON();
        const keyring = new Keyring({ type: 'sr25519' });
        const keyToHex = (name) => {
          const key = keyring.addFromUri('//' + name, { name: name + ' default' });
          return u8aToHex(key.publicKey);
        }
        const expectedAllCollators = {
            orchestratorChain: [
              keyToHex('Collator1000-01'),
              keyToHex('Collator1000-02'),
            ],
            containerChains: {
              '2000': [
                keyToHex('Collator2000-01'),
                keyToHex('Collator2000-02'),
              ],
              '2001': [
                keyToHex('Collator2001-01'),
                keyToHex('Collator2001-02'),
              ]
          }
        };

        expect(allCollators).to.deep.equal(expectedAllCollators);
      },
    });

    it({
      id: "T04",
      title: "Blocks are being produced on container 2000",
      test: async function () {
        const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T05",
      title: "Blocks are being produced on container 2001",
      test: async function () {
        const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T06",
      title: "Test container chain 2000 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toJSON()[paraId];

        const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

        expect(containerChainCollators).to.deep.equal(writtenCollators);
      },
    });

    it({
      id: "T07",
      title: "Test container chain 2001 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toJSON()[paraId];

        const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toJSON();

        expect(containerChainCollators).to.deep.equal(writtenCollators);
      },
    });

    it({
      id: "T08",
      title: "Test author noting is correct for both containers",
      timeout: 60000,
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId2000 = (await container2000Api.query.parachainInfo.parachainId());
        const paraId2001 = (await container2001Api.query.parachainInfo.parachainId());

        const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];
        const containerChainCollators2001 = assignment.containerChains.toJSON()[paraId2001.toString()];

        await context.waitBlock(3, "Tanssi");
        const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);
        const author2001 = await paraApi.query.authorNoting.latestAuthor(paraId2001);

        expect(containerChainCollators2000.includes(author2000.toString())).to.be.true;
        expect(containerChainCollators2001.includes(author2001.toString())).to.be.true;
      },
    });

    it({
      id: "T09",
      title: "Test author is correct in Orchestrator",
      test: async function () {
        const authorities = (await paraApi.query.aura.authorities());
        const author = await getAuthorFromDigest(paraApi);
        expect(authorities.toJSON().includes(author.toString())).to.be.true;
      },
    });
  },
});
