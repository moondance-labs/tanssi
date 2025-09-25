import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    type RawXcmMessage,
    XcmFragment,
    injectUmpMessageAndSeal,
    jumpToSession,
    TESTNET_ETHEREUM_NETWORK_ID,
} from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS } from "helpers";
import { isStarlightRuntime } from "../../../utils/runtime.ts";

describeSuite({
    id: "DEVT1910",
    title: "XCM - Succeeds sending XCM for ERC20 transfer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let transferredBalance: bigint;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightContainerExport: boolean;
        let containerERC20Asset: any;
        let tokenTransferChannel: any;
        const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

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

            const txSigned = polkadotJs.tx.balances.transferAllowDeath(convertedAddress, transferredBalance);
            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(convertedAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);

            const tokenAddress = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

            containerERC20Asset = {
                parents: 0,
                interior: {
                    X1: {
                        AccountKey20: {
                            network: ethereumNetwork,
                            key: tokenAddress,
                        },
                    },
                },
            };

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
            title: "Should succeed exporting the message for ERC20",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const xcmToExport = new XcmFragment({
                    assets: [
                        {
                            multilocation: containerERC20Asset,
                            fungible: transferredBalance / 10n,
                        },
                    ],
                    beneficiary: "0x983a1a72503d6cc3636776747ec627172b51272b",
                })
                    .withdraw_asset()
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
    },
});
