import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { jumpToSession } from "util/block";
import { RawXcmMessage, XcmFragment, injectUmpMessageAndSeal } from "../../../util/xcm";


describeSuite({
    id: "DTR1003",
    title: "XCM - Succeeds sending XCM",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let transferredBalance;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
        
            transferredBalance = 10_000_000_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed receiving tokens",
            test: async function () {
                // XCM message sending reserved assets to alice
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: { Here: null },
                            },
                            fungible: transferredBalance,
                        },
                    ],
                    beneficiary: u8aToHex(alice.addressRaw),
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Send an XCM and create block to execute it
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                 await jumpToSession(context, 3);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                // Make sure the state has alice's to DOT tokens
                const alice_dot_balance = (await context.polkadotJs().query.foreignAssets.account(1, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alice_dot_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alice_dot_balance < transferredBalance).to.be.true;
            },
        });
    },
});
