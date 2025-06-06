import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { signAndSendAndInclude, TESTNET_ETHEREUM_NETWORK_ID } from "utils";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "ZOMBIETANSS02",
    title: "XCM transfer to Ethereum",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceAccount32: KeyringPair;
        let chain: string;

        beforeAll(async () => {
            containerChainPolkadotJs = context.polkadotJs("Container2001");
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            chain = containerChainPolkadotJs.consts.system.version.specName.toString();
            aliceAccount32 = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
            alice = chain === "frontier-template" ? alith : aliceAccount32;
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
                await signAndSendAndInclude(
                    containerChainPolkadotJs.tx.sudo.sudo(
                        containerChainPolkadotJs.tx.utility.batch([
                            containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                ethereumTokenLocation,
                                assetId,
                                alice.address,
                                true,
                                1
                            ),
                            containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                { parents: 1, interior: "Here" },
                                tanssiAssetId,
                                alice.address,
                                true,
                                1
                            ),
                            containerChainPolkadotJs.tx.assetRate.create(assetId, 2_000_000_000_000_000_000n),
                            containerChainPolkadotJs.tx.assetRate.create(tanssiAssetId, 1_000_000_000_000_000_000n),
                        ])
                    ),
                    alice
                );

                await signAndSendAndInclude(
                    relayChainPolkadotJs.tx.sudo.sudo(
                        relayChainPolkadotJs.tx.utility.batch([
                            relayChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                { ...ethereumTokenLocation, parents: 1 }, // We decrease "parents" for the relay chain
                                assetId,
                                aliceAccount32.address,
                                true,
                                1
                            ),
                        ])
                    ),
                    aliceAccount32
                );

                const tanssiNativeTokenAmount = 1000n;

                await signAndSendAndInclude(
                    containerChainPolkadotJs.tx.foreignAssets.mint(assetId, alice.address, tanssiNativeTokenAmount),
                    alice
                );

                // Check balance before transfer
                const balanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId, alice.address)
                )
                    .unwrap()
                    .balance.toBigInt();

                expect(balanceBefore).toEqual(tanssiNativeTokenAmount);

                const metadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const foreignCreatorPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "ForeignAssets")
                    .index.toNumber();

                const dest = {
                    V3: {
                        parents: 1,
                        interior: "Here",
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

                const txRoot = containerChainPolkadotJs.tx.polkadotXcm.send(dest, {
                    V3: [
                        {
                            WithdrawAsset: [assetToTransfer],
                        },
                        {
                            DepositAsset: {
                                assets: { Wild: "All" }, // TODO: Try to be more specific here, instead of "All"
                                maxAssets: 1,
                                beneficiary: {
                                    parents: 1,
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

                const result = await signAndSendAndInclude(txRoot, alice);

                console.log(result);

                const balanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId, alice.address)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(balanceAfter < balanceBefore).to.be.true;
            },
        });
    },
});
