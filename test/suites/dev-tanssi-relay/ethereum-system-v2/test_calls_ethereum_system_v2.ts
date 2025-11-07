import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "helpers";

describeSuite({
    id: "DTR1901",
    title: "EthereumSystemV2 tests",
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
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Root is able to register tokens",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Snowbridge v2 not available yet`
                    );
                    return;
                }

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

                const registrarLocaiton = {
                    V3: {
                        parents: 0,
                        interior: "Here",
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
                const tx = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumSystemV2.registerToken(registrarLocaiton, versionedLocation, metadata, 0)
                    )
                    .signAsync(alice);

                const outboundNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                await context.createBlock([tx], { allowFailures: false });

                await context.createBlock();

                const outboundNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                // Nonce should increase
                expect(outboundNonceAfter.toNumber()).to.be.equal(outboundNonceBefore.toNumber() + 1);
            },
        });

        it({
            id: "E02",
            title: "Root is able to set operating mode",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Snowbridge v2 not available yet`
                    );
                    return;
                }

                // Register token on EthereumSystem.
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystemV2.setOperatingMode("RejectingOutboundMessages"))
                    .signAsync(alice);

                const outboundNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                await context.createBlock([tx], { allowFailures: false });

                await context.createBlock();

                const outboundNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                // Nonce should increase
                expect(outboundNonceAfter.toNumber()).to.be.equal(outboundNonceBefore.toNumber() + 1);
            },
        });

        it({
            id: "E03",
            title: "User is not able to add tip",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Snowbridge v2 not available yet`
                    );
                    return;
                }

                // Register token on EthereumSystem.
                const tx = await polkadotJs.tx.ethereumSystemV2
                    .addTip(
                        alice.address,
                        {
                            Outbound: 0,
                        },
                        100
                    )
                    .signAsync(alice);

                const {
                    result: [addTipAttempt],
                } = await context.createBlock([tx]);

                expect(addTipAttempt.successful).toEqual(false);
                expect(addTipAttempt.error.name).toEqual("BadOrigin");
            },
        });
    },
});
