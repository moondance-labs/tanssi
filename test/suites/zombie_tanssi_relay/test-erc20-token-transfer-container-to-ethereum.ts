import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { sleep, TESTNET_ETHEREUM_NETWORK_ID, waitEventUntilTimeout } from "utils";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "ZOMBIETANSS04",
    title: "XCM transfer ERC20 tokens to Ethereum",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceAccount32: KeyringPair;
        let chain: string;

        const ERC20_ASSET_AMOUNT = 123_321_000_000_000_000n;
        const RELAY_ASSET_FEE_AMOUNT = 3_500_000_000_000n; // Actual fee: 3480020020291, the rest will be trapped in HR

        const NEW_CHANNEL_ID = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const NEW_AGENT_ID = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const CHANNEL_PARA_ID = 0;

        const ERC20_FOREIGN_ASSET_ID = 123;
        const RELAY_FOREIGN_ASSET_ID = 124;
        const PARA_ID = 2001;

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
                // Random ETH destination that we send asset to
                const erc20AssetReceiverAddress = generateKeyringPair("ethereum").address;

                const erc20AssetIdTyped = context.polkadotJs().createType("u16", ERC20_FOREIGN_ASSET_ID);
                const relayAssetIdTyped = context.polkadotJs().createType("u16", RELAY_FOREIGN_ASSET_ID);

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
                const globalConsensusEthereumInterior = { GlobalConsensus: ethereumNetwork };

                const containerSovereignAccountInRelayRaw =
                    await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                        V3: { parents: 0, interior: { X1: { Parachain: PARA_ID } } },
                    });
                const containerSovereignAccountInRelay = containerSovereignAccountInRelayRaw.asOk.toHuman();

                const erc20AssetId = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

                const accountKey20Interior = {
                    AccountKey20: {
                        network: ethereumNetwork,
                        key: erc20AssetId,
                    },
                };
                const erc20AssetIdForContainerContext = {
                    parents: 2,
                    interior: { X2: [globalConsensusEthereumInterior, accountKey20Interior] },
                };
                const erc20AssetIdForRelayContext = {
                    parents: 1,
                    interior: { X2: [globalConsensusEthereumInterior, accountKey20Interior] },
                };
                const relayAssetIdForContainerContext = {
                    parents: 1,
                    interior: "Here",
                };

                await prepareForeignAssetsAndDistributeBalances();
                await checkBalancesBeforeExecution();

                const dest = {
                    V3: {
                        parents: 1,
                        interior: "Here",
                    },
                };

                const channelNonceBefore = await relayChainPolkadotJs.query.ethereumOutboundQueue.nonce(NEW_CHANNEL_ID);

                // Execute the XCM Transfer
                await containerChainPolkadotJs.tx.polkadotXcm
                    .transferAssetsUsingTypeAndThen(
                        dest,
                        prepareAssets(),
                        "DestinationReserve",
                        {
                            V3: { Concrete: relayAssetIdForContainerContext },
                        },
                        "DestinationReserve",
                        prepareCustomXcmOnDest(),
                        "Unlimited"
                    )
                    .signAndSend(alice);

                await waitEventUntilTimeout(relayChainPolkadotJs, "ethereumOutboundQueue.MessageAccepted", 90000);

                await sleep(24000);

                await checkBalancesAfterExecution();

                const channelNonceAfter = await relayChainPolkadotJs.query.ethereumOutboundQueue.nonce(NEW_CHANNEL_ID);
                expect(
                    channelNonceAfter.toNumber() - channelNonceBefore.toNumber(),
                    "Nonce should be increased by 1"
                ).toEqual(1);

                // Helpers
                async function prepareForeignAssetsAndDistributeBalances() {
                    const txHash = await relayChainPolkadotJs.tx.utility
                        .batch([
                            relayChainPolkadotJs.tx.sudo.sudo(
                                relayChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    erc20AssetIdForRelayContext,
                                    ERC20_FOREIGN_ASSET_ID,
                                    aliceAccount32.address,
                                    true,
                                    1
                                )
                            ),
                            relayChainPolkadotJs.tx.sudo.sudo(
                                relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                                    NEW_CHANNEL_ID,
                                    NEW_AGENT_ID,
                                    CHANNEL_PARA_ID
                                )
                            ),
                            relayChainPolkadotJs.tx.foreignAssets.mint(
                                erc20AssetIdTyped.toU8a(),
                                containerSovereignAccountInRelay,
                                ERC20_ASSET_AMOUNT
                            ),
                            relayChainPolkadotJs.tx.balances.transferKeepAlive(
                                containerSovereignAccountInRelay,
                                RELAY_ASSET_FEE_AMOUNT
                            ),
                        ])
                        .signAndSend(aliceAccount32);
                    expect(!!txHash.toHuman()).to.be.true;

                    const txHash1 = await containerChainPolkadotJs.tx.utility
                        .batch([
                            containerChainPolkadotJs.tx.sudo.sudo(
                                containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    erc20AssetIdForContainerContext,
                                    ERC20_FOREIGN_ASSET_ID,
                                    alice.address,
                                    true,
                                    1
                                )
                            ),
                            containerChainPolkadotJs.tx.sudo.sudo(
                                containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    relayAssetIdForContainerContext,
                                    RELAY_FOREIGN_ASSET_ID,
                                    alice.address,
                                    true,
                                    1
                                )
                            ),
                            containerChainPolkadotJs.tx.foreignAssets.mint(
                                erc20AssetIdTyped.toU8a(),
                                alice.address,
                                ERC20_ASSET_AMOUNT
                            ),
                            containerChainPolkadotJs.tx.foreignAssets.mint(
                                relayAssetIdTyped.toU8a(),
                                alice.address,
                                RELAY_ASSET_FEE_AMOUNT
                            ),
                        ])
                        .signAndSend(alice);
                    expect(!!txHash1.toHuman()).to.be.true;

                    await sleep(30000);
                }

                async function checkBalancesAfterExecution() {
                    // Container chain: check relay native (for fees) and ERC20 balances
                    const isErc20BalanceEmpty = (
                        await containerChainPolkadotJs.query.foreignAssets.account(
                            erc20AssetIdTyped.toU8a(),
                            alice.address
                        )
                    ).isNone;
                    expect(isErc20BalanceEmpty).to.eq(true);

                    const isRelayNativeBalanceEmpty = (
                        await containerChainPolkadotJs.query.foreignAssets.account(
                            relayAssetIdTyped.toU8a(),
                            alice.address
                        )
                    ).isNone;
                    expect(isRelayNativeBalanceEmpty).to.eq(true);

                    // Relay chain: check relay native (for fees) and ERC20 balances
                    const containerChainSovereignAccountSystemBalanceAfter = (
                        await relayChainPolkadotJs.query.system.account(containerSovereignAccountInRelay)
                    ).data.free.toBigInt();
                    expect(containerChainSovereignAccountSystemBalanceAfter).to.eq(0n);

                    const isContainerChainSovereignAccountErc20BalanceEmpty = (
                        await relayChainPolkadotJs.query.foreignAssets.account(erc20AssetIdTyped.toU8a(), alice.address)
                    ).isNone;
                    expect(isContainerChainSovereignAccountErc20BalanceEmpty).to.eq(true);
                }

                async function checkBalancesBeforeExecution() {
                    // Container chain: check relay native (for fees) and ERC20 balances
                    const erc20AssetBalanceBefore = (
                        await containerChainPolkadotJs.query.foreignAssets.account(
                            erc20AssetIdTyped.toU8a(),
                            alice.address
                        )
                    )
                        .unwrap()
                        .balance.toBigInt();
                    expect(erc20AssetBalanceBefore).to.eq(ERC20_ASSET_AMOUNT);

                    const relayNativeAssetBalanceBefore = (
                        await containerChainPolkadotJs.query.foreignAssets.account(
                            relayAssetIdTyped.toU8a(),
                            alice.address
                        )
                    )
                        .unwrap()
                        .balance.toBigInt();
                    expect(relayNativeAssetBalanceBefore).to.eq(RELAY_ASSET_FEE_AMOUNT);

                    // Relay chain: check native and ERC20 balances in container sovereign account
                    const containerChainSovereignAccountSystemBalanceBefore = (
                        await relayChainPolkadotJs.query.system.account(containerSovereignAccountInRelay)
                    ).data.free.toBigInt();
                    expect(containerChainSovereignAccountSystemBalanceBefore).to.eq(RELAY_ASSET_FEE_AMOUNT);

                    const containerChainSovereignAccountErc20BalanceBefore = (
                        await relayChainPolkadotJs.query.foreignAssets.account(erc20AssetIdTyped.toU8a(), alice.address)
                    )
                        .unwrap()
                        .balance.toBigInt();
                    expect(containerChainSovereignAccountErc20BalanceBefore).to.eq(ERC20_ASSET_AMOUNT);
                }

                function prepareAssets() {
                    const erc20Assets = {
                        id: {
                            Concrete: erc20AssetIdForContainerContext,
                        },
                        fun: { Fungible: ERC20_ASSET_AMOUNT },
                    };
                    const feeAssets = {
                        id: {
                            Concrete: relayAssetIdForContainerContext,
                        },
                        fun: { Fungible: RELAY_ASSET_FEE_AMOUNT },
                    };

                    return {
                        V3: [feeAssets, erc20Assets],
                    };
                }

                function prepareCustomXcmOnDest() {
                    const erc20AssetIdForEthereumContext = {
                        parents: 0,
                        interior: {
                            X1: accountKey20Interior,
                        },
                    };

                    const beneficiary = {
                        parents: 0,
                        interior: {
                            X1: {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: erc20AssetReceiverAddress,
                                },
                            },
                        },
                    };

                    return {
                        V3: [
                            {
                                InitiateReserveWithdraw: {
                                    assets: {
                                        Definite: [
                                            {
                                                id: {
                                                    Concrete: erc20AssetIdForRelayContext,
                                                },
                                                fun: { Fungible: ERC20_ASSET_AMOUNT },
                                            },
                                        ],
                                    },
                                    reserve: {
                                        parents: 1,
                                        interior: { X1: globalConsensusEthereumInterior },
                                    },
                                    xcm: [
                                        {
                                            DepositAsset: {
                                                assets: {
                                                    Definite: [
                                                        {
                                                            id: {
                                                                Concrete: erc20AssetIdForEthereumContext,
                                                            },
                                                            fun: { Fungible: ERC20_ASSET_AMOUNT },
                                                        },
                                                    ],
                                                },
                                                beneficiary,
                                            },
                                        },
                                    ],
                                },
                            },
                        ],
                    };
                }
            },
        });
    },
});
