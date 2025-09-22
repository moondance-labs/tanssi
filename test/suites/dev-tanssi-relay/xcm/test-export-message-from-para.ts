import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair, filterAndApply } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    type RawXcmMessage,
    XcmFragment,
    injectUmpMessageAndSeal,
    isStarlightRuntime,
    jumpToSession,
    TESTNET_ETHEREUM_NETWORK_ID,
} from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS } from "helpers";
import type { EventRecord } from "@polkadot/types/interfaces";

describeSuite({
    id: "DEVT1904",
    title: "XCM - Succeeds sending XCM",
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
                                ByGenesis: "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
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
            title: "Should succeed exporting the message",
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
                    .withdraw_asset()
                    .buy_execution()
                    .export_message(xcmToExport.instructions, ethereumNetwork, "Here")
                    .as_v3();

                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                const tokenTransferNonceBefore =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                // Wait until message is processed
                await jumpToSession(context, 3);
                await context.createBlock();
                const tokenTransferNonceAfter =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                expect(tokenTransferNonceAfter.toBigInt()).toBe(tokenTransferNonceBefore.toBigInt() + 1n);
            },
        });

        it({
            id: "T02",
            title: "Incorrect parachain should fail",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                const incorrectContainerAsset: any = {
                    parents: 1,
                    interior: {
                        X3: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
                                },
                            },
                            {
                                Parachain: 2005, // Incorrect parachain ID
                            },
                            {
                                PalletInstance: 5,
                            },
                        ],
                    },
                };
                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: incorrectContainerAsset,
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
                    .withdraw_asset()
                    .buy_execution()
                    .export_message(xcmToExport.instructions, ethereumNetwork, "Here")
                    .as_v3();

                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                const tokenTransferNonceBefore =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                await context.createBlock();
                const errorEvents = filterAndApply(
                    await context.polkadotJs().query.system.events(),
                    "xcmPallet",
                    ["ProcessXcmError"],
                    ({ event }: EventRecord) => event.data.toHuman() as unknown as { error: string }
                );
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].error, "The error message should be 'Unroutable'").toBe("Unroutable");

                // Wait until message is processed
                await jumpToSession(context, 6);
                await context.createBlock();
                const tokenTransferNonceAfter =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                expect(tokenTransferNonceAfter.toBigInt()).toBe(tokenTransferNonceBefore.toBigInt()); // No change in nonce
            },
        });

        it({
            id: "T03",
            title: "Export message should show error because of incorrect asset",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

                const incorrectContainerAsset: any = {
                    parents: 1,
                    interior: {
                        X3: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
                                },
                            },
                            {
                                Parachain: 2000,
                            },
                            {
                                PalletInstance: 5, // Incorrect pallet instance
                            },
                        ],
                    },
                };
                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: incorrectContainerAsset,
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
                    .withdraw_asset()
                    .buy_execution()
                    .export_message(xcmToExport.instructions, ethereumNetwork, "Here")
                    .as_v3();

                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                const tokenTransferNonceBefore =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                await context.createBlock();
                const errorEvents = filterAndApply(
                    await context.polkadotJs().query.system.events(),
                    "xcmPallet",
                    ["ProcessXcmError"],
                    ({ event }: EventRecord) => event.data.toHuman() as unknown as { error: string }
                );
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].error, "The error message should be 'Unroutable'").toBe("Unroutable");

                // Wait until message is processed
                await jumpToSession(context, 9);
                await context.createBlock();
                const tokenTransferNonceAfter =
                    await polkadotJs.query.ethereumOutboundQueue.nonce(tokenTransferChannel);

                expect(tokenTransferNonceAfter.toBigInt()).toBe(tokenTransferNonceBefore.toBigInt()); // No change in nonce, because pallet instance in incorrect
            },
        });
    },
});
