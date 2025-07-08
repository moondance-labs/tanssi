import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";
import {
    generateEventLog,
    generateUpdate,
    FOREIGN_ASSET_ID,
    ETHEREUM_NETWORK_MAINNET,
    ETHEREUM_NETWORK_TESTNET,
} from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR,
    checkCallIsFiltered,
} from "helpers";
import type { KeyringPair } from "@moonwall/util";
import { hexToU8a } from "@polkadot/util";

describeSuite({
    id: "DTR1704",
    title: "EthTokensLocalProcessor: receive inbound ETH tokens from Ethereum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;
        let shouldSkipStarlightForeignAssetsCreator: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

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
            title: "Receive ERC20 token from Ethereum in Tanssi chain",
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
                const transferAmount = BigInt(10_000);

                // Create token receiver account
                const tokenReceiver = encodeAddress(
                    "0x0505050505050505050505050505050505050505050505050505050505050505"
                );

                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    new Uint8Array([
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 222, 173, 190, 239, 222, 173, 190, 239, 222, 173, 190, 239, 222,
                        173, 190, 239, 222, 173, 190, 239, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 3,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

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

                const ethereumNetwork = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;
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
                            FOREIGN_ASSET_ID,
                            alice.address,
                            true,
                            1
                        )
                    )
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                // Check account balance before submitting the message
                const assetAccountDetailsBefore = await context
                    .polkadotJs()
                    .query.foreignAssets.account(FOREIGN_ASSET_ID, tokenReceiver);
                expect(assetAccountDetailsBefore.toJSON()).to.be.null;

                // Submit the message
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check the ERC20 token was received correctly
                const assetAccountDetailsAfter = await context
                    .polkadotJs()
                    .query.foreignAssets.account(FOREIGN_ASSET_ID, tokenReceiver);
                expect(assetAccountDetailsAfter.toJSON()).to.not.be.null;
                expect(BigInt(assetAccountDetailsAfter.toJSON().balance)).to.be.eq(transferAmount);
            },
        });
    },
});
