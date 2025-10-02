// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {hexToU8a, u8aToHex} from "@polkadot/util";
import {ETHEREUM_NETWORK_TESTNET, XcmFragment} from "utils";
import { BN } from "@polkadot/util";

describeSuite({
    id: "COMMON0305",
    title: "XCM - DryRunApi",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: any;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
        });

        it({
            id: "T01",
            title: "Should succeed calling dryRunCall",
            test: async () => {
                const metadata = await polkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();
                const randomReceiver = "0x1111111111111111111111111111111111111111111111111111111111111111";

                const versionedBeneficiary = {
                    V4: {
                        parents: 0,
                        interior: {
                            X1: [{
                                AccountId32: {
                                    network: null,
                                    id: randomReceiver,
                                },
                            }],
                        },
                    },
                };

                const versionedAssets = {
                    V4: [
                        {
                            id: {
                                Concrete: {
                                    parents: 0,
                                    interior: {
                                        X1: [
                                            { PalletInstance: Number(balancesPalletIndex) }
                                        ],
                                    },
                                },
                            },
                            fun: {
                                Fungible: 1_000_000_000_000_000n,
                            },
                        },
                    ],
                };
                const dest = {
                    V4: {
                        parents: 1,
                        interior: {
                            Here: null,
                        },
                    },
                };
                const tx = polkadotJs.tx.polkadotXcm.transferAssets(
                    dest,
                    versionedBeneficiary,
                    versionedAssets,
                    0,
                    "Unlimited"
                );

                const XCM_VERSION = 4;
                const dryRunCall = await polkadotJs.call.dryRunApi.dryRunCall(
                    { System: { signed: alice.address } },
                    tx,
                    XCM_VERSION
                );
                console.log("dryRunCall:", dryRunCall.toJSON());
                console.log("dryRunCall.asOk.executionResult", dryRunCall.asOk.executionResult.toJSON());
                /*
dryRunCall.asOk.executionResult {
  err: {
    postInfo: { actualWeight: null, paysFee: 'Yes' },
    error: { module: [Object] }
  }
}
*/
                console.log(
                    "dryRunCall.asOk.executionResult.err.error.module",
                    dryRunCall.asOk.executionResult.toJSON().err.error.module
                );
                const idx = 53;
                //const pallets = (await polkadotJs.rpc.state.getMetadata()).asLatest.pallets;
                //console.log('Pallet@53 =', pallets[idx].name.toString());

                const dispatchError = dryRunCall.asOk.executionResult.asErr;
                //const meta = polkadotJs.registry.findMetaError({ error: dispatchError.asModule.error, index: dispatchError.asModule.index });
                const meta = polkadotJs.registry.findMetaError({ error: hexToU8a('0x1c000800'), index: new BN(53) });
                //const meta = polkadotJs.registry.findMetaError('0x1c000800');
                console.log(`${meta.section}.${meta.name}`, meta.docs?.toString());

                expect(dryRunCall.isOk).to.be.true;
                expect(dryRunCall.asOk.executionResult.isOk).be.true;
            },
        });

        it({
            id: "T02",
            title: "Should succeed calling dryRunXcm",
            test: async () => {
                const metadata = await polkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();
                const random = chain === "frontier-template" ? generateKeyringPair() : generateKeyringPair("sr25519");

                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: {
                                    X1: { PalletInstance: Number(balancesPalletIndex) },
                                },
                            },
                            fungible: 1_000_000_000_000_000n,
                        },
                    ],
                    beneficiary: u8aToHex(random.addressRaw),
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                const dryRunXcm = await polkadotJs.call.dryRunApi.dryRunXcm(
                    {
                        V3: {
                            Concrete: { parent: 1, interior: { Here: null } },
                        },
                    },
                    xcmMessage
                );
                console.log("dryRunXcm:", dryRunXcm.toJSON());

                expect(dryRunXcm.isOk).to.be.true;
                expect(dryRunXcm.asOk.executionResult.isComplete).be.true;
            },
        });
    },
});
