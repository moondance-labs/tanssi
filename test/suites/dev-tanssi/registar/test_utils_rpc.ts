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
        id: "E01",
        title: "Read a ChainSpec, convert it to ContainerChainGenesisData, and back to the same ChainSpec",
        test: async function () {
            // Create a new provider with extended rpc and types
            const addr = polkadotJs._options.provider.endpoint;
            const wsProvider = new WsProvider(addr);
            const polkadotJs2 = await ApiPromise.create({ provider: wsProvider, types: {
              ContainerChainGenesisData: {
                storage: "Vec<ContainerChainGenesisDataItem>",
                name: "Bytes",
                id: "Bytes",
                fork_id: "Option<Vec<u8>>",
                extensions: "Bytes",
                properties: "Properties",
              },
              Properties: {
                token_metadata: "TokenMetadata",
                is_ethereum: "bool",
              },
              TokenMetadata: {
                // TODO: this is actually a Vec<u8>, but that doesn't work because polkadot.js converts the
                // Vec<u8> into hex bytes, while the Rust code doesn't work with hex bytes because this is
                // actually a BoundedVec, and there is no easy way to serialize a BoundedVec as hex bytes.
                // Ideally this should simply be a string, because this is a token name like "UNIT".
                token_symbol: "Vec<u16>",
                ss58_format: "u32",
                token_decimals: "u32",
              },
              ContainerChainGenesisDataItem: {
                key: "Bytes",
                value: "Bytes",
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

            // Mock raw chain spec file
            let spec2000 = `{
              "name": "Local Testnet",
              "id": "local_testnet",
              "chainType": "Local",
              "bootNodes": [],
              "telemetryEndpoints": null,
              "protocolId": "container-chain-2000",
              "properties": {
                "isEthereum": false,
                "ss58Format": 42,
                "tokenDecimals": 12,
                "tokenSymbol": "UNIT"
              },
              "relay_chain": "rococo-local",
              "para_id": 2000,
              "codeSubstitutes": {},
              "genesis": {
                "raw": {
                  "top": {
                    "0xf0c365c3cf59d671eb72da0e7a4113c44e7b9012096b41c4eb3aaf947f6ea429": "0x0000"
                  },
                  "childrenDefault": {}
                }
              }
            }`;

            let spec2000text = polkadotJs2.createType('Text', spec2000);
            const containerChainGenesisDataFromRpc = await polkadotJs2.rpc.utils.raw_chain_spec_into_container_chain_genesis_data(spec2000text);
            expect(containerChainGenesisDataFromRpc[0].toNumber()).to.be.equal(2000);

            const chainSpecJsonAgain = await polkadotJs2.rpc.utils.container_chain_genesis_data_into_raw_chain_spec(containerChainGenesisDataFromRpc[0], containerChainGenesisDataFromRpc[1]);
            const chainSpecFromFile = JSON.parse(spec2000);
            // The chainType is set dependending on the chain name of the running node, it is not
            // set to the chain type present in the chain spec file. So override it to make the expect pass.
            chainSpecFromFile.chainType = "Development";

            expect(chainSpecFromFile).to.deep.equal(JSON.parse(chainSpecJsonAgain.toString()));
        },
      });
    },
});
