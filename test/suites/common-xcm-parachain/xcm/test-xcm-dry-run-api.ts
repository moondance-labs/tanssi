// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { XcmFragment } from "utils";

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

                /*
                const tx4 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(tx4, aliceRelay);
                 */

                const versionedBeneficiary = {
                    V3: {
                        parents: 0,
                        interior: {
                            X1: {
                                AccountId32: {
                                    network: null,
                                    id: randomReceiver,
                                },
                            },
                        },
                    },
                };

                const versionedAssets = {
                    V3: [
                        {
                            id: {
                                Concrete: {
                                    parents: 0,
                                    interior: {
                                        X1: { PalletInstance: Number(balancesPalletIndex) },
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
                    V3: {
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

                // Sending the tx works
                /*
                const result = await context.createBlock(await tx.signAsync(alice));
                console.log(result);
                return;
                 */
                // The dry run call is fixed if we create a block
                // Why? ¯\_(ツ)_/¯
                await context.createBlock();

                const XCM_VERSION = 3;
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

dryRunCall.asOk.executionResult.err.error.module { index: 53, error: '0x01000000' }

2025-10-02 16:38:03 XCM validate_send failed with error error=Transport("Other") dest=Location { parents: 1, interior: Here } remote_xcm=Xcm([ReserveAssetDeposited(Assets([Asset { id: AssetId(Location { parents: 0, interior: X2([Parachain(1000), PalletInstance(10)]) }), fun: Fungible(1000000000000000) }])), ClearOrigin, BuyExecution { fees: Asset { id: AssetId(Location { parents: 0, interior: X2([Parachain(1000), PalletInstance(10)]) }), fun: Fungible(1000000000000000) }, weight_limit: Unlimited }, DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountId32 { network: None, id: [17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17] }]) } }])
*/
                console.log(
                    "dryRunCall.asOk.executionResult.err.error.module",
                    dryRunCall.asOk.executionResult.toJSON().err.error.module
                );

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
