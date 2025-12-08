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
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "ETHINBV2TANS",
    title: "Receive Tanssi Native Token from Ethereum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let ethNetworkId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            ethNetworkId = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;

            if (isStarlight) {
                console.log("Skipping test for Starlight runtime");
                return;
            }

            const ethTokenLocation = {
                parents: 1,
                interior: {
                    X1: [
                        {
                            GlobalConsensus: ethNetworkId,
                        },
                    ],
                },
            };
            const createForeignAssetTx = await polkadotJs.tx.sudo
                .sudo(
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethTokenLocation,
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
            title: "Receive Tanssi token from Ethereum in Tanssi chain",
            test: async () => {
                if (isStarlight) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const allEntries = await polkadotJs.query.ethereumSystem.nativeToForeignId.entries();
                const tokenIds = allEntries.map(([, id]) => id.toHuman());

                const tokenId = tokenIds[0];

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
                    [],
                    [
                        {
                            value: transferAmount,
                            tokenId,
                        },
                    ]
                );

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const nativeBalanceBefore = (await polkadotJs.query.system.account(tokenReceiver)).data.free.toBigInt();

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const nativeBalanceAfter = (await polkadotJs.query.system.account(tokenReceiver)).data.free.toBigInt();
                expect(nativeBalanceAfter - nativeBalanceBefore).to.be.eq(transferAmount);
            },
        });
    },
});
