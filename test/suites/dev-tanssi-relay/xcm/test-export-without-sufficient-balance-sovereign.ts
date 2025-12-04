// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { isStarlightRuntime, XcmFragment, TESTNET_ETHEREUM_NETWORK_ID, DANCELIGHT_GENESIS_HASH } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS } from "helpers";

describeSuite({
    id: "DEVT1909",
    title: "XCM - Succeeds error exporting xcm",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let random: KeyringPair;
        let transferredBalance: bigint;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightContainerExport: boolean;
        let containerAsset: any;
        let tokenTransferChannel: any;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

            random = generateKeyringPair("sr25519");

            isStarlight = isStarlightRuntime(polkadotJs);
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightContainerExport =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS.includes(specVersion);

            if (shouldSkipStarlightContainerExport) {
                console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                return;
            }

            transferredBalance = 100_000_000_000_000_000n;

            const location = {
                V3: {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                },
            };

            const locationToAccountResult = await polkadotJs.call.locationToAccountApi.convertLocation(location);
            expect(locationToAccountResult.isOk);

            const convertedAddress = locationToAccountResult.asOk.toJSON();

            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

            // DO NOT SEND TOKENS TO SOVEREIGN
            const balanceSigned = (await polkadotJs.query.system.account(convertedAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(0n);

            containerAsset = {
                parents: 1,
                interior: {
                    X3: [
                        {
                            GlobalConsensus: {
                                ByGenesis: DANCELIGHT_GENESIS_HASH,
                            },
                        },
                        {
                            Parachain: 2000,
                        },
                        {
                            PalletInstance: 10,
                        },
                    ],
                },
            };

            const containerAssetParentView = {
                parents: 0,
                interior: {
                    X2: [
                        {
                            Parachain: 2000,
                        },
                        {
                            PalletInstance: 10,
                        },
                    ],
                },
            };

            // Register the token of the container-chain
            const versionedLocation = {
                V3: containerAssetParentView,
            };

            const metadata = {
                name: "container",
                symbol: "cont",
                decimals: 12,
            };
            const registerTokenTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata)
            );

            await context.createBlock(await registerTokenTx.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            // Create EthereumTokenTransfers channel to validate when receiving the tokens.
            tokenTransferChannel = "0x0000000000000000000000000000000000000000000000000000000000000004";
            const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
            const newParaId = 500;

            const setChannelTx = await polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                    tokenTransferChannel,
                    newAgentId,
                    newParaId
                )
            );
            await context.createBlock(await setChannelTx.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should fail because the sovereign account does not have enough tokens to pay for the export",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: containerAsset,
                            fungible: transferredBalance / 10n,
                        },
                    ],
                    beneficiary: "0x983a1a72503d6cc3636776747ec627172b51272b",
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset_v3()
                    .set_topic();

                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: { Here: null },
                            },
                            fungible: transferredBalance / 10n,
                        },
                    ],
                })
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                Parachain: 2000,
                            },
                        },
                    })
                    .withdraw_asset()
                    .buy_execution()
                    .export_message(xcmToExport.instructions, ethereumNetwork, "Here")
                    .as_v3();

                const tokenTransferNonceBefore =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                const executeMessageTx = await polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                        refTime: 10000000000,
                        proofSize: 1000000,
                    })
                );
                const { result } = await context.createBlock(executeMessageTx.signAsync(alice));

                // sudo calls are always true
                expect(result.successful).to.be.true;

                // we need one more for the nonce to be updated
                await context.createBlock();

                const tokenTransferNonceAfter =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                // but the nonce never increases
                expect(tokenTransferNonceAfter.toBigInt()).toBe(tokenTransferNonceBefore.toBigInt());
            },
        });
    },
});
