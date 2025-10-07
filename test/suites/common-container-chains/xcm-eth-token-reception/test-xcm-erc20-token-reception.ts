// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { ETHEREUM_NETWORK_TESTNET } from "utils";

import { type RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "utils";

describeSuite({
    id: "COM0203",
    title: "Mock XCM - Succeeds receiving erc20 token from ethereum",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: any;
        let tokenAddrHex: string;
        let relayNativeTokenAssetId: number;
        let erc20AssetId: number;
        let ethTokenLocationFromContainerViewpoint: any;
        let beneficiary: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
          
            beneficiary = chain === "frontier-template" ? generateKeyringPair() : generateKeyringPair("sr25519");

            // since in the future is likely that we are going to add this to containers, I leave it here
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });

            relayNativeTokenAssetId = 42;
            erc20AssetId = 24;
            tokenAddrHex = "0x1111111111111111111111111111111111111111";

            const relayNativeTokenLocation = {
                parents: 1,
                interior: "Here",
            };

            ethTokenLocationFromContainerViewpoint = {
                parents: 2,
                interior: {
                    X2: [
                        {
                            GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                        },
                        {
                            AccountKey20: {
                                network: ETHEREUM_NETWORK_TESTNET,
                                key: tokenAddrHex,
                            },
                        },
                    ],
                },
            };

            // Create relay native token as foreign in container
            const registerRelayNativeTokenLocationTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                    relayNativeTokenLocation,
                    relayNativeTokenAssetId,
                    alice.address,
                    true,
                    1
                )
            );
            await context.createBlock(await registerRelayNativeTokenLocationTx.signAsync(alice), {
                allowFailures: false,
            });

            // Create relay native token rate in container
            const assetRateTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.assetRate.create(relayNativeTokenAssetId, 50_000_000_000_000_000_000n)
            );
            await context.createBlock(await assetRateTx.signAsync(alice), {
                allowFailures: false,
            });

            // Create erc20 token as foreign in container
            const registerErc20ContainerViewpointAssetTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                    ethTokenLocationFromContainerViewpoint,
                    erc20AssetId,
                    alice.address,
                    true,
                    1
                )
            );
            await context.createBlock(await registerErc20ContainerViewpointAssetTx.signAsync(alice), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should succeed receiving tokens",
            test: async () => {
                const feeLocation = { parents: 1, interior: "Here" };
                const feeAmount = 2_000_000_000_000_000n;
                const depositAmount = 100_000_000n;

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                    ReserveAssetDeposited(
                        vec![asset_fee_container.clone(), asset_to_deposit.clone()].into(),
                    ),
                    BuyExecution {
                        fees: asset_fee_container.clone(),
                        weight_limit: Unlimited,
                    },
                    DepositAsset {
                        assets: Definite(vec![asset_to_deposit].into()),
                        beneficiary,
                    },
                */

                const xcmMessage = new XcmFragment({
                    assets: [
                        { multilocation: feeLocation, fungible: feeAmount },
                        { multilocation: ethTokenLocationFromContainerViewpoint, fungible: depositAmount },
                    ],
                    beneficiary: u8aToHex(beneficiary.addressRaw),
                })
                    .reserve_asset_deposited()
                    .buy_execution(0)
                    .deposit_asset_definite(ethTokenLocationFromContainerViewpoint, depositAmount, u8aToHex(beneficiary.addressRaw))
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const beneficiaryBalanceAfter = (
                    await polkadotJs.query.foreignAssets.account(erc20AssetId, u8aToHex(beneficiary.addressRaw))
                )
                    .unwrap()
                    .balance.toBigInt();

                // Tokens are minted to the beneficiary account
                expect(beneficiaryBalanceAfter).to.be.eq(depositAmount);
            },
        });
    },
});
