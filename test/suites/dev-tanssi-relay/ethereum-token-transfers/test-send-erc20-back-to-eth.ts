import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";
import { generateEventLog, generateUpdate } from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR,
    checkCallIsFiltered,
} from "helpers";
import type { KeyringPair } from "@moonwall/util";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "DTR1705",
    title: "EthProcessor: send eth ERC20 tokens back to etherum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;
        let shouldSkipStarlightForeignAssetsCreator: boolean;
        let assetId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            assetId = 42;

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS.includes(specVersion);
            shouldSkipStarlightForeignAssetsCreator =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Send ERC20 back to ethereum",
            test: async () => {
                if (shouldSkipStarlightForeignAssetsCreator) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: ForeignAssetsCreator pallet not available yet`
                    );
                    return;
                }

                if (shouldSkipStarlightETT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);

                    // Check that inboundQueue.submit is filtered
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.ethereumInboundQueue.submit("0x").signAsync(alice)
                    );
                    return;
                }
                const initialBalance = BigInt(100_000);

                const transferAmount = BigInt(10_000);

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                const ethereumNetwork = isStarlight
                    ? { Ethereum: { chainId: 1 } }
                    : { Ethereum: { chainId: 11155111 } };
                const tokenAddress = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

                // Create token on ForeignAssetsCreator to be validated when receiving the tokens.
                const ethTokenLocation = {
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

                // Register token on ForeignAssetsCreator.
                const tx2 = await polkadotJs.tx.sudo
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

                await context.createBlock([tx2], { allowFailures: false });

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.foreignAssets.mint(assetId, alice.addressRaw, initialBalance)
                        .signAsync(alice)
                );
            },
        });
    },
});
