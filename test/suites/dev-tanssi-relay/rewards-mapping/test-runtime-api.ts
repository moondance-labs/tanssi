import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { XcmFragment } from "util/xcm.ts";

const runtimeApi = {
    runtime: {
        ExternalValidatorsRewardsApi: [
            {
                methods: {
                    generate_rewards_merkle_root: {
                        description: "Get rewards merkle root for a specific era",
                        params: [
                            {
                                name: "era_index",
                                type: "EraIndex",
                            },
                        ],
                        type: "H256",
                    },
                },
                version: 1,
            },
        ],
    },
};

describeSuite({
    id: "DTR0820",
    title: "Starlight <> Ethereum - Rewards mapping",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain;

        beforeAll(async function () {
/*             polkadotJs = await ApiPromise.create({
                provider: new WsProvider(`ws://localhost:${process.env.MOONWALL_RPC_PORT}/`),
                ...runtimeApi,
            }); */ 

            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
        });

        it({
            id: "T01",
            title: "Should succeed calling runtime api",
            test: async function () {

                await polkadotJs.call.externalValidatorsRewardsApi.generate_rewards_merkle_proof(0);

            },
        });
    },
});
