import { expect, describeSuite, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { BN } from "@polkadot/util";
const fs = require('fs/promises');
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
      // TODO: this breaks the hack of starting 2001 nodes as 2000 and then rotating, for testing
      //expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
      //expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

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
            "nmbs",
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

    it({
      id: "T05",
      title: "Test container chain 2000 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toJSON()[paraId];

        const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

        for (let i = 0; i < containerChainCollators.length; i++) {
          expect(containerChainCollators[i]).to.be.equal(writtenCollators[i]);
        }
      },
    });

    it({
      id: "T06",
      title: "Test container chain 2001 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toJSON()[paraId];

        const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toJSON();

        for (let i = 0; i < containerChainCollators.length; i++) {
          expect(containerChainCollators[i]).to.be.equal(writtenCollators[i]);
        }
      },
    });

    it({
      id: "T07",
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
      id: "T08",
      title: "Test author is correct in Orchestrator",
      test: async function () {
        const authorities = (await paraApi.query.aura.authorities());
        const author = await getAuthorFromDigest(paraApi);
        expect(authorities.toJSON().includes(author.toString())).to.be.true;
      },
    });

    it({
      id: "T09",
      title: "Test live registration of container chain 2002",
      timeout: 600000,
      test: async function () {
        const keyring = new Keyring({ type: 'sr25519' });
        let alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

        // Read raw chain spec file
        // Different path in CI: ./specs vs ../specs
        let spec2002 = null;
        try {
            spec2002 = await fs.readFile("./specs/template-container-2002.json", 'utf8');
        } catch {
            spec2002 = await fs.readFile("../specs/template-container-2002.json", 'utf8');
        }

        // Augment paraApi with new RPC method
        // TODO: latest moonwall version supports this in beforeAll
        const wsProvider2 = new WsProvider('ws://127.0.0.1:9948');
        let paraApi2 = await ApiPromise.create({ provider: wsProvider2,
          types: {
            ContainerChainGenesisData: {
              storage: "Vec<ContainerChainGenesisDataItem>",
              name: "Vec<u8>",
              id: "Vec<u8>",
              fork_id: "Option<Vec<u8>>",
              extensions: "Vec<u8>",
              properties: "TokenMetadata",
            },
            TokenMetadata: {
              token_symbol: "Vec<u8>",
              ss58_format: "u32",
              token_decimals: "u32",
            },
            ContainerChainGenesisDataItem: {
              key: "Vec<u8>",
              value: "Vec<u8>",
            }
          },
          rpc: {
            utils: {
              raw_chain_spec_into_container_chain_genesis_data: {
                description: 'Convert a raw chain spec string into a ContainerChainGenesisData',
                params: [
                  {
                    name: 'raw_chain_spec',
                    type: 'Text'
                  }
                ],
                type: '(u32, ContainerChainGenesisData)'
              }
            }
          }
        });
        let spec2002text = paraApi2.createType('Text', spec2002);
        const containerChainGenesisDataFromRpc = await paraApi2.rpc.utils.raw_chain_spec_into_container_chain_genesis_data(spec2002text);
        expect(containerChainGenesisDataFromRpc[0].toNumber()).to.be.equal(2002);

        // Before registering container chain 2002, ensure that it has 0 blocks
        // Since the RPC doesn't exist at this point, we need to get that from the relay
        const header2002 = await getHeaderFromRelay(relayApi, 2002);
        expect(header2002.number.toNumber()).to.be.equal(0);
        const registered1 = (await paraApi.query.registrar.registeredParaIds());
        expect(registered1.toJSON().includes(2002)).to.be.false;

        const tx = paraApi.tx.registrar.register(2002, containerChainGenesisDataFromRpc[1]);
        await paraApi.tx.sudo.sudo(tx).signAndSend(alice);
        
        const tanssiBlockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
        // TODO: this should wait 2 sessions. We are waiting 10 + 20 blocks
        await countUniqueBlockAuthors(context, paraApi, 10, 4);
        await context.waitBlock(20, "Tanssi");

        // Check that pending para ids contains 2002
        const registered = (await paraApi.query.registrar.registeredParaIds());
        expect(registered.toJSON().includes(2002)).to.be.true;

        // This ws api is only available after the node detects its assignment
        // TODO: wait up to 30 seconds after a new block is created to ensure this port is available
        const wsProvider = new WsProvider('ws://127.0.0.1:9951');
        let container2002Api = await ApiPromise.create({ provider: wsProvider });

        const container2002Network = container2002Api.consts.system.version.specName.toString();
        const paraId2002 = (await container2002Api.query.parachainInfo.parachainId()).toString();
        expect(container2002Network, "Container2002 API incorrect").to.contain("container-chain-template");
        expect(paraId2002, "Container2002 API incorrect").to.be.equal("2002");

        // Check authors of tanssi blocks
        // Should be 2 different keys because 2002 has been registered
        await countUniqueBlockAuthors(context, paraApi, 4, 2);

        let blockNum = (await container2002Api.rpc.chain.getBlock()).block.header.number.toNumber();

        while (blockNum == 0) {
            // Wait a bit
            // Cannot use context.waitBlock because the container2002Api is not part of moonwall
            const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
            await sleep(1_000);

            blockNum = (await container2002Api.rpc.chain.getBlock()).block.header.number.toNumber();
        }
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T10",
      title: "Deregister container chain 2002, collators should move to tanssi",
      timeout: 600000,
      test: async function () {
        const keyring = new Keyring({ type: 'sr25519' });
        let alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

        const registered1 = (await paraApi.query.registrar.registeredParaIds());
        expect(registered1.toJSON().includes(2002)).to.be.true;

        const tx = paraApi.tx.registrar.deregister(2002);
        await paraApi.tx.sudo.sudo(tx).signAndSend(alice);
        
        const tanssiBlockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
        // TODO: this should wait 2 sessions. We are waiting 10 + 20 blocks
        await countUniqueBlockAuthors(context, paraApi, 10, 2);
        await context.waitBlock(20, "Tanssi");

        // Check that pending para ids removes 2002
        const registered = (await paraApi.query.registrar.registeredParaIds());
        expect(registered.toJSON().includes(2002)).to.be.false;

        // Check authors of tanssi blocks
        // Should be 2 different keys when 2002 is registered, and 4 different keys when 2002 is deregistered
        await countUniqueBlockAuthors(context, paraApi, 4, 4);
      },
    });
  },
});

// Verify that the next `numBlocks` have `numAuthors` different unique authors
async function countUniqueBlockAuthors(context, paraApi, numBlocks, numAuthors) {
  const authorities = (await paraApi.query.aura.authorities());
  const actualAuthors = [];

  for (let i = 0; i < numBlocks; i++) {
      const author = await getAuthorFromDigest(paraApi);
      actualAuthors.push(author);
      await context.waitBlock(1, "Tanssi");
  }

  let uniq = [...new Set(actualAuthors)];

  if (uniq.length != numAuthors) {
    console.error("Mismatch between authorities and actual block authors: authorities: ", authorities.toJSON(), ", actual authors: ", actualAuthors);
    expect(false).to.be.true;
  }
}