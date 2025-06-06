import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { TESTNET_ETHEREUM_NETWORK_ID } from "utils";
import { hexToU8a } from "@polkadot/util";

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
                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
                const assetId = 42;
                const tanssiAssetId = 2;
                // Random ETH destination that we send asset to
                const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
                const tokenAddress = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");
                const ethereumTokenLocation = {
                    parents: 2,
                    interior: {
                        X2: [
                            {
                                GlobalConsensus: ethereumNetwork,
                            },
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: tokenAddress,
                                },
                            },
                        ],
                    },
                };

                // Let's create an asset and register it
                await context.createBlock(
                    await polkadotJs.tx.sudo
                        .sudo(
                            polkadotJs.tx.utility.batch([
                                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    ethereumTokenLocation,
                                    assetId,
                                    alice.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    { parents: 1, interior: "Here" },
                                    tanssiAssetId,
                                    alice.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(assetId, 2_000_000_000_000_000_000n),
                                polkadotJs.tx.assetRate.create(tanssiAssetId, 1_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alice),
                    {
                        allowFailures: false,
                    }
                );

                const tanssiNativeTokenAmount = 1000n;

                await context.createBlock(
                    await polkadotJs.tx.foreignAssets
                        .mint(assetId, alice.address, tanssiNativeTokenAmount)
                        .signAsync(alice),
                    {
                        allowFailures: false,
                    }
                );

                // Check balance before transfer
                const balanceBefore = (await polkadotJs.query.foreignAssets.account(assetId, alice.address))
                    .unwrap()
                    .balance.toBigInt();

                expect(balanceBefore).toEqual(tanssiNativeTokenAmount);

                const metadata = await polkadotJs.rpc.state.getMetadata();
                const foreignCreatorPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "ForeignAssets")
                    .index.toNumber();

                const dest = {
                    V3: {
                        parents: 2,
                        interior: {
                            X1: {
                                GlobalConsensus: ethereumNetwork,
                            },
                        },
                    },
                };

                const assetToTransfer = {
                    id: {
                        Concrete: {
                            parents: 0,
                            interior: {
                                X2: [{ PalletInstance: foreignCreatorPalletIndex }, { GeneralIndex: assetId }],
                            },
                        },
                    },
                    fun: { Fungible: 2500_000_000_000_000_000n },
                };

                const txRoot = polkadotJs.tx.polkadotXcm.send(dest, {
                    V3: [
                        {
                            WithdrawAsset: [assetToTransfer],
                        },
                        {
                            DepositAsset: {
                                assets: { Wild: "All" }, // TODO: Try to be more specific here, instead of "All"
                                maxAssets: 1,
                                beneficiary: {
                                    parents: 2,
                                    interior: {
                                        X2: [
                                            {
                                                GlobalConsensus: ethereumNetwork,
                                            },
                                            {
                                                AccountKey20: {
                                                    network: ethereumNetwork,
                                                    key: destinationAddress,
                                                },
                                            },
                                        ],
                                    },
                                },
                            },
                        },
                    ],
                });

                const result = await context.createBlock(await txRoot.signAsync(alice), { allowFailures: true }); // TODO: revert allow failures

                console.log("result", result);

                const balanceAfter = (await polkadotJs.query.foreignAssets.account(assetId, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(balanceAfter < balanceBefore).to.be.true;
            },
        });
    },
});
