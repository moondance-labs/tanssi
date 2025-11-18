// @ts-nocheck

import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    type RawXcmMessage,
    injectUmpMessageAndSeal,
    isStarlightRuntime,
    jumpToSession,
    SNOWBRIDGE_FEES_ACCOUNT,
} from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2,
} from "helpers";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "DTR2003",
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
        let ethTokenLocation;
        let ethLocation;
        let ethereumNetwork;
        let tokenAddress;
        let sovAddress;
        let assetId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

            random = generateKeyringPair("sr25519");

            isStarlight = isStarlightRuntime(polkadotJs);
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightContainerExport =
                isStarlight &&
                (STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS.includes(specVersion) ||
                    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2);

            if (shouldSkipStarlightContainerExport) {
                console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                return;
            }

            assetId = 42;

            transferredBalance = 100_000_000_000_000_000n;

            const location = {
                V3: {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                },
            };

            const locationToAccountResult = await polkadotJs.call.locationToAccountApi.convertLocation(location);
            expect(locationToAccountResult.isOk);

            sovAddress = locationToAccountResult.asOk.toJSON();

            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

            // Send some tokens to the sovereign account of para 2000
            const txSigned = polkadotJs.tx.balances.transferAllowDeath(sovAddress, transferredBalance);
            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(sovAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);
            ethereumNetwork = isStarlight ? { Ethereum: { chainId: 1 } } : { Ethereum: { chainId: 11155111 } };
            tokenAddress = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

            // Specify ethereum destination with global consensus
            ethLocation = {
                parents: 1,
                interior: {
                    X1: [
                        {
                            GlobalConsensus: ethereumNetwork,
                        },
                    ],
                },
            };
            // Create token on ForeignAssetsCreator to be validated when receiving the tokens.
            ethTokenLocation = {
                parents: 1,
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
            const initialBalance = BigInt(100_000);

            // Register token on ForeignAssetsCreator.
            const createAssetTx = await polkadotJs.tx.sudo
                .sudo(
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethTokenLocation,
                        assetId,
                        alice.address,
                        true,
                        1
                    )
                )
                .signAsync(alice);

            await context.createBlock([createAssetTx], { allowFailures: false });

            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(assetId, sovAddress, initialBalance).signAsync(alice)
            );

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
            await context.createBlock(await setChannelTx.signAsync(alice), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should succeed sending back eth token from container v2",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const transferAmount = BigInt(10_000);
                const feeAmount = BigInt(1_000_000_000_000);
                const beneficiaryOnDest = {
                    parents: 0,
                    interior: {
                        X1: [
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: "0x1111111111111111111111111111111111111111",
                                },
                            },
                        ],
                    },
                };
                const feeTokenLocation = {
                    parents: 0,
                    interior: { Here: null },
                };
                const feeAssetToWithdraw = {
                    id: feeTokenLocation,
                    fun: { Fungible: feeAmount * 2n },
                };
                const feeAssetToBuyExec = {
                    id: feeTokenLocation,
                    fun: { Fungible: feeAmount },
                };

                const ethAssetToWithdraw = {
                    id: ethTokenLocation,
                    fun: { Fungible: transferAmount },
                };

                const xcmMessage = {
                    V5: [
                        {
                            WithdrawAsset: [feeAssetToWithdraw, ethAssetToWithdraw],
                        },
                        {
                            BuyExecution: {
                                fees: feeAssetToBuyExec,
                                weight_limit: "Unlimited",
                            },
                        },
                        {
                            InitiateTransfer: {
                                destination: ethLocation,
                                remoteFees: {
                                    ReserveDeposit: {
                                        Definite: [
                                            {
                                                id: feeTokenLocation,
                                                fun: { Fungible: feeAmount },
                                            },
                                        ],
                                    },
                                },
                                preserveOrigin: true,
                                assets: [
                                    {
                                        ReserveWithdraw: {
                                            Definite: [ethAssetToWithdraw],
                                        },
                                    },
                                ],
                                remoteXcm: [
                                    {
                                        DepositAsset: {
                                            assets: { Wild: { AllCounted: 1 } },
                                            beneficiary: beneficiaryOnDest,
                                        },
                                    },
                                ],
                            },
                        },
                    ],
                };

                await context.createBlock();
                // Send RPC call to enable para inherent candidate generation

                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                const tokenTransferNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const sovEthBalanceBefore = (await polkadotJs.query.foreignAssets.account(assetId, sovAddress))
                    .unwrap()
                    .balance.toBigInt();

                // Wait until message is processed
                // we need to wait until session 3 for sure so that paras produce blocks
                await jumpToSession(context, 3);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();
                await context.createBlock();

                // Things to verify:
                // 1. ethereumOutboundQueueV2 increases the nonce
                // 2. reward goes to snowbridge fees account
                // 3. sent tokens are burnt from alice
                // 4. a pending order exists for such nonce, with the fee=reward
                const tokenTransferNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const sovEthBalanceAfter = (await polkadotJs.query.foreignAssets.account(assetId, sovAddress))
                    .unwrap()
                    .balance.toBigInt();
                const pendingOrder =
                    await polkadotJs.query.ethereumOutboundQueueV2.pendingOrders(tokenTransferNonceAfter);

                expect(tokenTransferNonceAfter.toNumber()).to.be.equal(tokenTransferNonceBefore.toNumber() + 1);
                expect(snowbridgeFeesAccountBalanceAfter).to.be.eq(snowbridgeFeesAccountBalanceBefore + feeAmount);
                expect(sovEthBalanceAfter).to.be.eq(sovEthBalanceBefore - transferAmount);
                expect(pendingOrder.unwrap().fee.toBigInt()).to.be.equal(feeAmount);
                expect(tokenTransferNonceAfter.toBigInt()).toBe(tokenTransferNonceBefore.toBigInt() + 1n);
            },
        });
    },
});
