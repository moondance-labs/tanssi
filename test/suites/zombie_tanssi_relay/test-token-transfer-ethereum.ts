import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, sleep, TESTNET_ETHEREUM_NETWORK_ID, waitEventUntilTimeout, SNOWBRIDGE_FEES_ACCOUNT } from "utils";

describeSuite({
    id: "ZOMBIETANSS02",
    title: "XCM transfer to Ethereum",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceAccount32: KeyringPair;
        let chain: string;

        // Random ETH destination that we send asset to
        const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
        const holdingAccount = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        const tokenToTransfer = 123_321_000_000_000_000n;

        const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newParaId = 0;

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

                const convertLocation = await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: { parents: 0, interior: { X1: { Parachain: 2001 } } },
                });
                const convertedAddress = convertLocation.asOk.toHuman();

                console.log("Converted address:", convertedAddress);

                const versionedLocation = {
                    V3: {
                        parents: 1,
                        interior: {
                            X3: [
                                {
                                    GlobalConsensus: {
                                        ByGenesis: "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
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
                    },
                };

                const containerTokenMetadata = {
                    name: "para2001",
                    symbol: "para2001",
                    decimals: 12,
                };

                const txHash = await relayChainPolkadotJs.tx.utility
                    .batch([
                        relayChainPolkadotJs.tx.balances.transferKeepAlive(convertedAddress, 100_000_000_000_000n),
                        relayChainPolkadotJs.tx.sudo.sudo(
                            relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                                newChannelId,
                                newAgentId,
                                newParaId
                            )
                        ),
                        relayChainPolkadotJs.tx.sudo.sudo(
                            relayChainPolkadotJs.tx.ethereumSystem.registerToken(
                                versionedLocation,
                                containerTokenMetadata
                            )
                        ),
                    ])
                    .signAndSend(aliceAccount32);

                expect(!!txHash.toHuman()).to.be.true;

                // Check balance before transfer
                const balanceBefore = (
                    await containerChainPolkadotJs.query.system.account(holdingAccount)
                ).data.free.toBigInt();
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
                const metadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();

                const assetToTransferNative = {
                    id: {
                        Concrete: {
                            parents: 0,
                            interior: {
                                X1: { PalletInstance: Number(balancesPalletIndex) },
                            },
                        },
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

                // Fees account (on Tanssi) only has the existential deposit
                const existentialDeposit = relayChainPolkadotJs.consts.balances.existentialDeposit.toBigInt();
                const feesAccountBalanceBefore = (await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)).data.free.toBigInt();
                expect(feesAccountBalanceBefore).to.be.eq(existentialDeposit);

                await containerChainPolkadotJs.tx.polkadotXcm
                    .transferAssets(dest, versionedBeneficiary, versionedAssets, 0, "Unlimited")
                    .signAndSend(alice);

                await waitEventUntilTimeout(relayChainPolkadotJs, "ethereumOutboundQueue.MessageAccepted", 90000);

                // Wait 2 blocks until nonce changed
                await sleep(24000);

                const balanceAfter = (
                    await containerChainPolkadotJs.query.system.account(holdingAccount)
                ).data.free.toBigInt();

                expect(balanceAfter - balanceBefore).toEqual(tokenToTransfer);

                const channelNonceAfter = await relayChainPolkadotJs.query.ethereumOutboundQueue.nonce(newChannelId);

                // Fees are collected on Tanssi
                const feesAccountBalanceAfter = (await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)).data.free.toBigInt();
                expect(feesAccountBalanceAfter).toBeGreaterThan(feesAccountBalanceBefore);

                // Check that nonce has changed
                expect(channelNonceAfter.toNumber() - channelNonceBefore.toNumber()).toEqual(1);
            },
        });
    },
});
