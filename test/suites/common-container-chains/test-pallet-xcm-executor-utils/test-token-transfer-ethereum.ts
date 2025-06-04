import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { type RawXcmMessage, XcmFragment, injectUmpMessageAndSeal, ETHEREUM_NETWORK_ID } from "utils";

describeSuite({
    id: "COM0103",
    title: "XCM transfer to Ethereum",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: string;
        let transferredBalance: bigint;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;
        });

        it({
            id: "T01",
            title: "Should allow sending asset to Ethereum",
            test: async () => {
                const assetId = 1;
                // Random ETH destination that we send asset to
                const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
                const ethereumMultilocation = {
                    parents: 1,
                    interior: {
                        X1: [
                            {
                                GlobalConsensus: { Ethereum: { chainId: ETHEREUM_NETWORK_ID } },
                            },
                        ],
                    },
                };

                // Let's create an asset and register it
                const result = await context.createBlock(
                    await polkadotJs.tx.sudo
                        .sudo(
                            polkadotJs.tx.utility.batch([
                                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    ethereumMultilocation,
                                    assetId,
                                    alice.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(assetId, 2_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alice),
                    {
                        allowFailures: false,
                    }
                );

                // Check balance before transfer
                const balanceBefore = (await polkadotJs.query.foreignAssets.account(assetId, alice.address))
                    .unwrap()
                    .balance.toBigInt();

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
                    beneficiary: {
                        parents: 1,
                        interior: {
                            X2: [
                                {
                                    GlobalConsensus: { Ethereum: { chainId: ETHEREUM_NETWORK_ID } },
                                },
                                {
                                    AccountKey20: {
                                        network: null,
                                        key: destinationAddress,
                                    },
                                },
                            ],
                        },
                    },
                })
                    .withdraw_asset()
                    .buy_execution()
                    .export_message()
                    .as_v3();

                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();

                const balanceAfter = (await polkadotJs.query.foreignAssets.account(assetId, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(balanceAfter < balanceBefore).to.be.true;
            },
        });
    },
});
