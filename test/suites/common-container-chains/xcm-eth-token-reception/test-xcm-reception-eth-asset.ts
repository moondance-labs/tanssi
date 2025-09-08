import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import {
    type RawXcmMessage,
    XcmFragment,
    injectDmpMessageAndSeal,
    SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_FRONTIER,
    SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_SUBSTRATE,
} from "utils";

describeSuite({
    id: "COM0202",
    title: "Mock XCM - Succeeds receiving container-chain token back from ethereum",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let transferredBalance: bigint;
        let alice: KeyringPair;
        let chain: any;
        let ethereumSovereignAddress: any;
        let balancesPalletIndex: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();

            // Get Pallet balances index
            const metadata = await context.polkadotJs().rpc.state.getMetadata();
            const foundPallet = metadata.asLatest.pallets.find((pallet) => pallet.name.toString() === "Balances");

            if (!foundPallet || !foundPallet.index) {
                throw new Error("Balances pallet or its index not found");
            }

            balancesPalletIndex = foundPallet.index.toNumber();

            // since in the future is likely that we are going to add this to containers, I leave it here
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });

            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;

            ethereumSovereignAddress = context.isEthereumChain
                ? SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_FRONTIER
                : SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_SUBSTRATE;

            // We transfer double to the sovereign account just in case
            // this is to avoid ED errors
            const txSigned = polkadotJs.tx.balances.transferKeepAlive(
                ethereumSovereignAddress,
                transferredBalance * 2n
            );

            await context.createBlock(await txSigned.signAsync(alice), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should succeed receiving tokens",
            test: async () => {
                const aliceBalanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free;
                const ethSovereignBalanceBefore = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                    UniversalOrigin(GlobalConsensus(network)),
                    WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                    BuyExecution {
                        fees: container_asset_fee,
                        weight_limit: Unlimited,
                    },
                    DepositAsset {
                        assets: Definite(container_asset_to_deposit.into()),
                        beneficiary,
                    },
                */
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: {
                                    X1: { PalletInstance: balancesPalletIndex },
                                },
                            },
                            fungible: transferredBalance,
                        },
                    ],
                    beneficiary: u8aToHex(alice.addressRaw),
                })
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                PalletInstance: 24,
                            },
                        },
                    })
                    .push_any({
                        UniversalOrigin: {
                            GlobalConsensus: {
                                Ethereum: {
                                    chainId: 11155111,
                                },
                            },
                        },
                    })
                    .withdraw_asset()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const aliceBalanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free;

                const ethSovereignBalanceAfter = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // it's not clear how much alice should receive since it will depend on fee, but it has to be greater
                expect(aliceBalanceAfter.toBigInt()).to.be.greaterThan(aliceBalanceBefore.toBigInt());

                // what it's clear is that the sovereign will have lost ths
                expect(ethSovereignBalanceBefore.toBigInt()).to.be.eq(
                    ethSovereignBalanceAfter.toBigInt() + transferredBalance
                );
            },
        });
    },
});
