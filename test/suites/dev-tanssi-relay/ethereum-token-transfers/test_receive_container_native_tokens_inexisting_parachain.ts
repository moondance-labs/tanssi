import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateEventLog,
    generateUpdate,
    makeSendTokenMessageFrontierTemplateInexistingParachain,
    SNOWBRIDGE_FEES_ACCOUNT,
} from "utils";
import { expectEventCount, STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS } from "helpers";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1804",
    title: "NativeContainerTokensProcessor: reception of container native tokens (parachain not found)",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightCTT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightCTT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Receive Container native token from Ethereum in Tanssi chain and forward to container",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Container native token transfers not available yet`
                    );
                    return;
                }
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
                    await makeSendTokenMessageFrontierTemplateInexistingParachain(isStarlight)
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

                const tokenLocation = {
                    parents: 0,
                    interior: {
                        X2: [
                            {
                                Parachain: 5000,
                            },
                            {
                                PalletInstance: 10,
                            },
                        ],
                    },
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "para5000",
                    symbol: "para5000",
                    decimals: 12,
                };

                // Register token on EthereumSystem.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                // Add funds to snowbridge fees account
                const transferFeesAccountTx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.balances.forceSetBalance(SNOWBRIDGE_FEES_ACCOUNT, 500_000_000_000_000_000n))
                    .signAsync(alice);
                await context.createBlock([transferFeesAccountTx], { allowFailures: false });

                // We DON'T mock the para head data, so the parachain is unreachable.
                //const paraId = polkadotJs.createType("ParaId", 5000);
                //await mockAndInsertHeadData(context, paraId, 2, 2, alice);

                const nonceBefore = await polkadotJs.query.ethereumInboundQueue.nonce(newChannelId);

                // Submit the message
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);

                // Execution should succeed regardless of the parachain being unreachable.
                await context.createBlock([tx3], { allowFailures: false });

                const nonceAfter = await polkadotJs.query.ethereumInboundQueue.nonce(newChannelId);

                // Nonce should increase
                expect(nonceAfter.toNumber()).to.be.equal(nonceBefore.toNumber() + 1);

                // XCM Sent event should not be emitted
                await expectEventCount(polkadotJs, {
                    Sent: 0,
                });
            },
        });
    },
});
