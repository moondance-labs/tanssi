// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { encodeAddress, decodeAddress } from "@polkadot/util-crypto";
import {
    generateUpdate,
    FOREIGN_ASSET_ID,
    ETHEREUM_NETWORK_TESTNET,
    generateOutboundMessageAcceptedLog,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    ETHEREUM_NETWORK_MAINNET,
} from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR,
} from "helpers";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "ETHINBV2ERC20",
    title: "Receive ERC20 Token from Ethereum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;
        let shouldSkipStarlightForeignAssetsCreator: boolean;
        let tokenAddress: string;
        let ethNetworkId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS.includes(specVersion);
            shouldSkipStarlightForeignAssetsCreator =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR.includes(specVersion);

            ethNetworkId = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;

            tokenAddress = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
            const erc20TokenLocation = {
                parents: 1,
                interior: {
                    X2: [
                        {
                            GlobalConsensus: ethNetworkId,
                        },
                        {
                            AccountKey20: {
                                network: ethNetworkId,
                                key: tokenAddress,
                            },
                        },
                    ],
                },
            };
            const createForeignAssetTx = await polkadotJs.tx.sudo
                .sudo(
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        erc20TokenLocation,
                        FOREIGN_ASSET_ID,
                        alice.address,
                        true,
                        1
                    )
                )
                .signAsync(alice);

            await context.createBlock([createForeignAssetTx], { allowFailures: false });

            const tokenLocation = {
                parents: 0,
                interior: "Here",
            };
            const versionedLocation = {
                V3: tokenLocation,
            };

            const metadata = {
                name: "relay",
                symbol: "relay",
                decimals: 12,
            };
            const registerTokenTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.ethereumSystemV2.registerToken(versionedLocation, versionedLocation, metadata, 1)
            );
            await context.createBlock(await registerTokenTx.signAsync(alice), {
                allowFailures: false,
            });

            const transferFeesAccountTx = await polkadotJs.tx.sudo
                .sudo(
                    polkadotJs.tx.balances.forceSetBalance(
                        SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
                        10_000_000_000_000_000_000n
                    )
                )
                .signAsync(alice);
            await context.createBlock([transferFeesAccountTx], { allowFailures: false });
        });

        it({
            id: "E01",
            title: "Receive ERC20 token from Ethereum in Tanssi chain",
            test: async () => {
                if (isStarlight) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const transferAmount = 123_456_789_000n;

                // Create token receiver account
                const tokenReceiver = encodeAddress(
                    "0x0909090909090909090909090909090909090909090909090909090909090909"
                );

                const instructions = [
                    {
                        DepositAsset: {
                            assets: {
                                Wild: {
                                    AllCounted: 1,
                                },
                            },
                            beneficiary: {
                                parents: 0,
                                interior: {
                                    X1: [
                                        {
                                            AccountId32: {
                                                network: null,
                                                id: decodeAddress(tokenReceiver),
                                            },
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ];

                const log = await generateOutboundMessageAcceptedLog(
                    polkadotJs,
                    1,
                    0,
                    instructions,
                    [
                        {
                            tokenAddress,
                            value: 123_456_000n,
                        },
                    ],
                    []
                );

                await new Promise((res) => {
                    setTimeout(() => {
                        res();
                    }, 120000);
                });

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const assetAccountDetailsBefore = await context
                    .polkadotJs()
                    .query.foreignAssets.account(FOREIGN_ASSET_ID, tokenReceiver);
                expect(assetAccountDetailsBefore.toJSON()).to.be.null;

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const assetAccountDetailsAfter = await context
                    .polkadotJs()
                    .query.foreignAssets.account(FOREIGN_ASSET_ID, tokenReceiver);
                expect(assetAccountDetailsAfter.toJSON()).to.not.be.null;
                expect(BigInt(assetAccountDetailsAfter.toJSON().balance)).to.be.eq(transferAmount);
            },
        });
    },
});
