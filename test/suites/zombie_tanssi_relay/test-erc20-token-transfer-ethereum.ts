import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { sleep, TESTNET_ETHEREUM_NETWORK_ID, waitEventUntilTimeout } from "utils";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "ZOMBIETANSS04",
    title: "XCM transfer ERC20 tokens back to to Ethereum",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceAccount32: KeyringPair;
        let chain: string;

        // Random ETH destination that we send asset to
        const destinationAddress = generateKeyringPair("ethereum").address;
        const tokenToTransfer = 123_321_000_000_000_000n;

        const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newParaId = 0;

        const ASSET_ID = 123;

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

                const containerSovereignAccountInRelayRaw =
                    await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                        V3: { parents: 0, interior: { X1: { Parachain: 2001 } } },
                    });
                const containerSovereignAccountInRelay = containerSovereignAccountInRelayRaw.asOk.toHuman();

                console.log("ContainerSovereignAccountInRelay:", containerSovereignAccountInRelay);

                const tokenAddress = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

                const ethTokenLocationContainer = {
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

                const txHash = await relayChainPolkadotJs.tx.utility
                    .batch([
                        relayChainPolkadotJs.tx.balances.transferKeepAlive(
                            containerSovereignAccountInRelay,
                            10_000_000_000_000_000n
                        ),
                        relayChainPolkadotJs.tx.sudo.sudo(
                            relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                                newChannelId,
                                newAgentId,
                                newParaId
                            )
                        ),
                    ])
                    .signAndSend(aliceAccount32);

                console.log("Tx hash:", txHash.toHuman());

                expect(!!txHash.toHuman()).to.be.true;

                await sleep(12000);

                const assetId = context.polkadotJs().createType("u16", ASSET_ID);
                const txHash1 = await containerChainPolkadotJs.tx.utility
                    .batch([
                        containerChainPolkadotJs.tx.sudo.sudo(
                            containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                ethTokenLocationContainer,
                                ASSET_ID,
                                alice.address,
                                true,
                                1
                            )
                        ),
                        containerChainPolkadotJs.tx.foreignAssets.mint(assetId.toU8a(), alice.address, tokenToTransfer),
                    ])
                    .signAndSend(alice);

                console.log("Tx1 hash:", txHash1.toHuman());

                expect(!!txHash1.toHuman()).to.be.true;

                await sleep(12000);

                // Check balance before transfer
                const balanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId.toU8a(), alice.address)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(balanceBefore).to.eq(tokenToTransfer);

                const versionedBeneficiary = {
                    V3: {
                        parents: 0,
                        interior: {
                            X1: {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: destinationAddress,
                                },
                            },
                        },
                    },
                };
                const assetToTransferNative = {
                    id: {
                        Concrete: ethTokenLocationContainer,
                    },
                    fun: { Fungible: tokenToTransfer },
                };
                const versionedAssets = {
                    V3: [assetToTransferNative],
                };

                // Specify ethereum destination with global consensus
                const dest = {
                    V3: {
                        parents: 2,
                        interior: {
                            X1: {
                                GlobalConsensus: ethereumNetwork,
                            },
                        },
                    },
                };

                const channelNonceBefore = await relayChainPolkadotJs.query.ethereumOutboundQueue.nonce(newChannelId);

                await containerChainPolkadotJs.tx.polkadotXcm
                    .transferAssets(dest, versionedBeneficiary, versionedAssets, 0, "Unlimited")
                    .signAndSend(alice);

                await waitEventUntilTimeout(relayChainPolkadotJs, "ethereumOutboundQueue.MessageAccepted", 90000);

                await sleep(24000);

                // It should be empty after the transfer
                const isBalanceEmpty = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId.toU8a(), alice.address)
                ).isNone;
                expect(isBalanceEmpty).to.eq(true);

                const channelNonceAfter = await relayChainPolkadotJs.query.ethereumOutboundQueue.nonce(newChannelId);

                // Check that nonce has changed
                expect(channelNonceAfter.toNumber() - channelNonceBefore.toNumber()).toEqual(1);
            },
        });
    },
});
