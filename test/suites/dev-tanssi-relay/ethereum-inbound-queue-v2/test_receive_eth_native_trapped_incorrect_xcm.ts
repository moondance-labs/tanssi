// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    FOREIGN_ASSET_ID,
    ETHEREUM_NETWORK_TESTNET,
    generateOutboundMessageAcceptedLog,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    ETHEREUM_NETWORK_MAINNET,
} from "utils";
import type { KeyringPair } from "@moonwall/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "../../../helpers";

describeSuite({
    id: "ETHINBV2ETHTRAP",
    title: "ETH native tokens trapped, incorrect XCM",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let ethNetworkId: number;
        let shouldSkipStarlightSnV2TT: boolean;
        let specVersion: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            ethNetworkId = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;

            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSnV2TT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);

            if (shouldSkipStarlightSnV2TT) {
                console.log("Skipping test for Starlight runtime");
                return;
            }

            const ethTokenLocation = {
                parents: 1,
                interior: {
                    X1: [
                        {
                            GlobalConsensus: ethNetworkId,
                        },
                    ],
                },
            };
            const createForeignAssetTx = await polkadotJs.tx.sudo
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

            await context.createBlock([createForeignAssetTx], { allowFailures: false });

            const tokenLocation = {
                parents: 0,
                interior: "Here",
            };
            const versionedLocation = {
                V3: tokenLocation,
            };

            const metadata = {
                name: "relay",
                symbol: "relay",
                decimals: 12,
            };
            const registerTokenTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.ethereumSystemV2.registerToken(versionedLocation, versionedLocation, metadata, 1)
            );
            await context.createBlock(await registerTokenTx.signAsync(alice), {
                allowFailures: false,
            });

            const transferFeesAccountTx = await polkadotJs.tx.sudo
                .sudo(
                    polkadotJs.tx.balances.forceSetBalance(
                        SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
                        10_000_000_000_000_000_000n
                    )
                )
                .signAsync(alice);
            await context.createBlock([transferFeesAccountTx], { allowFailures: false });
        });

        it({
            id: "E01",
            title: "Receive ETH native token from Ethereum in Tanssi chain",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const transferAmount = BigInt(12345n);

                // Let's use empty XCM to tokens should be trapped
                const instructions = [];

                const log = await generateOutboundMessageAcceptedLog(polkadotJs, 1, transferAmount, instructions);

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();
                const xcmErrorEvent = events.find(
                    (event) =>
                        event.toHuman().event.method === "AssetsTrapped" &&
                        BigInt(event.toHuman().event.data.assets.V5[0].fun.Fungible.replace(/,/g, "")) ===
                            transferAmount
                );
                expect(!!xcmErrorEvent).to.equal(true);
            },
        });
    },
});
