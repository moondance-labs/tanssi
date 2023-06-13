import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
const fs = require('fs/promises');

import "@polkadot/api-augment";

describeSuite({
  id: "D07",
  title: "Test utils RPC",
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
        id: "E06",
        title: "Checking that fetching registered paraIds is possible",
        test: async function () {
            debugger;
            const addr = polkadotJs._options.provider.endpoint;
            const wsProvider = new WsProvider(addr);
            // If this fails, wait up to 30 seconds after a new block is created
            // to ensure this port is available
            const polkadotJs2 = await ApiPromise.create({ provider: wsProvider, types: {
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
                is_ethereum: "bool",
              },
              ContainerChainGenesisDataItem: {
                key: "Vec<u8>",
                value: "Vec<u8>",
              }
            },
            rpc: {
              utils: {
                raw_chain_spec_into_container_chain_genesis_data: {
                  description: 'Convert a raw chain spec JSON string into a ContainerChainGenesisData',
                  params: [
                    {
                      name: 'raw_chain_spec_json',
                      type: 'Text'
                    }
                  ],
                  type: '(u32, ContainerChainGenesisData)'
                },
                container_chain_genesis_data_into_raw_chain_spec: {
                  description: 'Convert a ContainerChainGenesisData into a raw chain spec JSON string',
                  params: [
                    {
                      name: 'para_id',
                      type: 'u32'
                    },{
                      name: 'container_chain_genesis_data',
                      type: 'ContainerChainGenesisData'
                    },
                  ],
                  type: 'Text'
                },

              }
            }});

            const parasRegistered = await polkadotJs2.query.registrar.registeredParaIds();

            // These are registered in genesis
            expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);

            // Read raw chain spec file
            // Different path in CI: ./specs vs ../specs
            let spec2000 = null;
            try {
                spec2000 = await fs.readFile("./specs/template-container-2000.json", 'utf8');
            } catch {
                spec2000 = await fs.readFile("../specs/template-container-2000.json", 'utf8');
            }

            let spec2000text = polkadotJs2.createType('Text', spec2000);
            const containerChainGenesisDataFromRpc = await polkadotJs2.rpc.utils.raw_chain_spec_into_container_chain_genesis_data(spec2000text);
            expect(containerChainGenesisDataFromRpc[0].toNumber()).to.be.equal(2000);

            console.log(containerChainGenesisDataFromRpc[1].toJSON());

            console.log("----- AFTER ------");
            const chainSpecJsonAgain = await polkadotJs2.rpc.utils.container_chain_genesis_data_into_raw_chain_spec(containerChainGenesisDataFromRpc[0], containerChainGenesisDataFromRpc[1]);
            console.log(chainSpecJsonAgain.toString());
        },
      });
    },
});
