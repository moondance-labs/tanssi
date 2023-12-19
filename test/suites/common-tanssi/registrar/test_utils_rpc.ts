import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { chainSpecToContainerChainGenesisData, containerChainGenesisDataToChainSpec } from "../../../util/genesis_data";
import "@polkadot/api-augment";

describeSuite({
    id: "CT0503",
    title: "Test ContainerChainGenesisData utils",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Read a ChainSpec, convert it to ContainerChainGenesisData, and back to the same ChainSpec",
            test: async function () {
                // Mock raw chain spec file
                const chainSpec2000 = {
                    name: "Local Testnet",
                    id: "local_testnet",
                    chainType: "Local",
                    forkId: null,
                    bootNodes: [],
                    telemetryEndpoints: null,
                    protocolId: "container-chain-2000",
                    properties: {
                        isEthereum: false,
                        ss58Format: 42,
                        tokenDecimals: 12,
                        tokenSymbol: "UNIT",
                    },
                    relay_chain: "rococo-local",
                    para_id: 2000,
                    codeSubstitutes: {},
                    genesis: {
                        raw: {
                            top: {
                                "0xf0c365c3cf59d671eb72da0e7a4113c44e7b9012096b41c4eb3aaf947f6ea429": "0x0000",
                            },
                            childrenDefault: {},
                        },
                    },
                };
                const containerChainGenesisData = chainSpecToContainerChainGenesisData(polkadotJs, chainSpec2000);
                const chainSpecJsonAgain = containerChainGenesisDataToChainSpec(
                    containerChainGenesisData,
                    2000,
                    "Local",
                    "rococo-local"
                );

                expect(chainSpec2000).to.deep.equal(chainSpecJsonAgain);
            },
        });
    },
});
