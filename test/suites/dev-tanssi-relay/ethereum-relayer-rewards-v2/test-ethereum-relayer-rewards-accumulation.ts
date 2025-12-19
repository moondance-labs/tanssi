// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateOutboundEventLogV2,
    generateUpdate,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    type MultiLocation,
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
} from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "helpers";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR2102",
    title: "EthereumTokenTransfersV2 tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;
        let sovereignAccount: string;
        let relayerReward: bigint;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);
            sovereignAccount = isStarlight
                ? ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS
                : SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;

            const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
            const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
            const newParaId = 500;

            // Set channel info on EthereumTokenTransfers pallet.
            const tx1 = polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                newChannelId,
                newAgentId,
                newParaId
            );

            const tokenLocation: MultiLocation = {
                parents: 0,
                interior: "Here",
            };
            const versionedLocation = {
                V3: tokenLocation,
            };

            const metadata = {
                name: "dance",
                symbol: "dance",
                decimals: 12,
            };

            const registrarLocation = {
                V3: {
                    parents: 0,
                    interior: "Here",
                },
            };

            if (shouldSkipStarlightETT) {
                console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                return;
            }

            const sudoSignedTx1 = await polkadotJs.tx.sudo.sudo(tx1).signAsync(alice);
            await context.createBlock([sudoSignedTx1], { allowFailures: false });

            // Register token on EthereumSystemV2.
            const tx2 = await polkadotJs.tx.sudo
                .sudo(polkadotJs.tx.ethereumSystemV2.registerToken(registrarLocation, versionedLocation, metadata, 0))
                .signAsync(alice);

            await context.createBlock([tx2], { allowFailures: false });
            // 1 TANSSI
            relayerReward = 1_000_000_000_000n;
            const recipient = "0x0000000000000000000000000000000000000007";
            const amount = 1000n;
            // Finally call transferNativeToken extrinsic.
            // Put 2, we will demonstrate rewards are accumulated
            const tx3 = await polkadotJs.tx.ethereumTokenTransfers
                .transferNativeTokenV2(amount, recipient, relayerReward)
                .signAsync(alice);
            await context.createBlock([tx3], { allowFailures: false });

            const tx4 = await polkadotJs.tx.ethereumTokenTransfers
                .transferNativeTokenV2(amount, recipient, relayerReward)
                .signAsync(alice);
            await context.createBlock([tx4], { allowFailures: false });
        });

        it({
            id: "E01",
            title: "Relayer should be able to accumulate rewards",
            test: async () => {
                if (shouldSkipStarlightETT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Snowbridge v2 not available yet`
                    );
                    return;
                }
                // Use random account instead of alice because alice is getting block rewards
                const randomAccount = generateKeyringPair("sr25519");

                const nonceToProveFirst = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const nonceToProveSecond = nonceToProveFirst.toNumber() - 1;

                const eventFirst = await generateOutboundEventLogV2(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    nonceToProveFirst.toNumber(),
                    true,
                    randomAccount.addressRaw
                );

                const eventSecond = await generateOutboundEventLogV2(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    nonceToProveSecond,
                    true,
                    randomAccount.addressRaw
                );

                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [
                    eventFirst,
                    eventSecond,
                ]);
                const firstUpdate = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate))
                    .signAsync(alice, { nonce: aliceNonce++ });
                const fisrtClaim = await polkadotJs.tx.ethereumOutboundQueueV2
                    .submitDeliveryReceipt(messageExtrinsics[0])
                    .signAsync(alice, { nonce: aliceNonce++ });
                await context.createBlock([firstUpdate, fisrtClaim], { allowFailures: false });

                const secondClaim = await polkadotJs.tx.ethereumOutboundQueueV2
                    .submitDeliveryReceipt(messageExtrinsics[1])
                    .signAsync(alice, { nonce: aliceNonce++ });
                await context.createBlock([secondClaim], { allowFailures: false });

                // now we simply claim as relayer
                // we are going to do it with sudo to have the relayer Reward as is (as sudo pays for tx fees)
                const claimTx = await polkadotJs.tx.sudo
                    .sudoAs(
                        randomAccount.address,
                        polkadotJs.tx.bridgeRelayers.claimRewards("SnowbridgeRewardOutbound")
                    )
                    .signAsync(alice);
                await context.createBlock([claimTx], { allowFailures: false });
                const balanceRandom = (await polkadotJs.query.system.account(randomAccount.address)).data.free;
                // We should have claimed twice
                expect(balanceRandom.toBigInt()).to.equal(relayerReward * 2n);
            },
        });
    },
});
