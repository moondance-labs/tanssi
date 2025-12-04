// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { isStarlightRuntime, XcmFragment, TESTNET_ETHEREUM_NETWORK_ID, DANCELIGHT_GENESIS_HASH } from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS,
    retrieveDispatchErrors,
    retrieveSudoDispatchErrors,
    retrieveBatchDispatchErrors,
} from "helpers";

describeSuite({
    id: "DEVT1906",
    title: "XCM - Export errors",
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

            // Send some tokens to the sovereign account of para 2000
            const txSigned = polkadotJs.tx.balances.transferAllowDeath(convertedAddress, transferredBalance);
            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(convertedAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);

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
            title: "Should fail exporting from a user-level call",
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });

                await context.createBlock(executeMessageTx.signAsync(alice));

                const errorEvents = await retrieveDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0]).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T02",
            title: "Should fail exporting if clear origin message is not there",
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });

                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));

                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T03",
            title: "Should fail exporting if buy_execution is not present in the message",
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });

                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));
                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T04",
            title: "Should fail exporting if exporting to something not ethereum",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                // random chain id
                const noneEthereumNetwork = { Ethereum: { chainId: 5 } };

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
                    .export_message(xcmToExport.instructions, noneEthereumNetwork, "Here")
                    .as_v3();

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });

                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));
                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T05",
            title: "Should fail exporting to a 32 byte address in ethereum",
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
                    beneficiary: u8aToHex(random.addressRaw),
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });
                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));

                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T06",
            title: "Should fail exporting a message without appropriate instructions",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                // only a set topic inst
                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: containerAsset,
                            fungible: transferredBalance / 10n,
                        },
                    ],
                    beneficiary: "0x983a1a72503d6cc3636776747ec627172b51272b",
                }).set_topic();

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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });
                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));

                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T07",
            title: "Should fail exporting a message that contains several assets",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
                // pallet instance 11 in this case
                const containerAsset2 = {
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
                                PalletInstance: 11,
                            },
                        ],
                    },
                };

                // many assets
                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: containerAsset,
                            fungible: transferredBalance / 10n,
                        },
                        {
                            multilocation: containerAsset2,
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });

                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));

                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T08",
            title: "Should fail exporting an asset for which we are not reserve",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                // We first need to register an asset for 2001, otherwise we dont know if that is the failure reason

                const containerAsset2001 = {
                    parents: 1,
                    interior: {
                        X3: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: DANCELIGHT_GENESIS_HASH,
                                },
                            },
                            {
                                Parachain: 2001,
                            },
                            {
                                PalletInstance: 10,
                            },
                        ],
                    },
                };

                const containerAsset2001ParentView = {
                    parents: 0,
                    interior: {
                        X2: [
                            {
                                Parachain: 2001,
                            },
                            {
                                PalletInstance: 10,
                            },
                        ],
                    },
                };

                // Register the token of the container-chain
                const versionedLocation = {
                    V3: containerAsset2001ParentView,
                };

                const metadata = {
                    name: "container2001",
                    symbol: "cont2001",
                    decimals: 12,
                };
                const registerTokenTx = polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata);

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                // many assets
                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: containerAsset2001,
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
                    // we still descend to 2000
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });
                // session change, no txs
                await context.createBlock();

                await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.utility.batch([registerTokenTx, executeMessageTx]))
                        .signAsync(alice)
                );

                const errorEvents = await retrieveBatchDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of retrieveBatchDispatchErrorsf error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });

        it({
            id: "T09",
            title: "Should fail reserve asset deposited is different to depositAsset",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const containerAsset2 = {
                    parents: 1,
                    interior: {
                        X3: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: DANCELIGHT_GENESIS_HASH,
                                },
                            },
                            {
                                Parachain: 20010,
                            },
                            {
                                PalletInstance: 11,
                            },
                        ],
                    },
                };

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                // here we simply substitute the depositAsset to instead of being wild be definite
                // we additionally put a different asset
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
                    .deposit_asset_definite(
                        containerAsset2,
                        transferredBalance / 10n,
                        "0x983a1a72503d6cc3636776747ec627172b51272b"
                    )
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
                    // we still descend to 2000
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

                const executeMessageTx = polkadotJs.tx.xcmPallet.execute(xcmMessage, {
                    refTime: 10000000000,
                    proofSize: 1000000,
                });
                // session change, no txs

                await context.createBlock(polkadotJs.tx.sudo.sudo(executeMessageTx).signAsync(alice));

                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("LocalExecutionIncompleteWithError");
            },
        });
    },
});
