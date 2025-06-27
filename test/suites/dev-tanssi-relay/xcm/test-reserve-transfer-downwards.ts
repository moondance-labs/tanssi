import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { type MultiLocation, extractPaidDeliveryFeesDancelight, getLastSentDmpMessageFee } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_XCM, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1901",
    title: "XCM - Succeeds sending XCM reserve transfer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain: any;
        const UNITS = 1_000_000_000_000n;
        const CENTS = UNITS / 30_000n;
        const MILICENTS = CENTS / 1000n;
        const txByteFee = 10n * MILICENTS;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightXCM: boolean;

        const randomReceiver = "0x1111111111111111111111111111111111111111111111111111111111111111";

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            baseDelivery = 100_000_000n;

            isStarlight = chain === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightXCM = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_XCM.includes(specVersion);
        });

        it({
            id: "T01",
            title: "Should succeed sending a reserve transfer downward",
            test: async () => {
                const destMultilocation: MultiLocation = {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                };

                const beneficiary: MultiLocation = {
                    parents: 0,
                    interior: {
                        X1: {
                            AccountId32: {
                                network: null,
                                id: randomReceiver,
                            },
                        },
                    },
                };

                const versionedBeneficiary = {
                    V3: beneficiary,
                };

                const assets = [
                    {
                        id: {
                            Concrete: {
                                parents: 0,
                                interior: "Here",
                            },
                        },
                        fun: {
                            Fungible: 1_000_000_000_000_000n,
                        },
                    },
                ];
                const versionedAssets = {
                    V3: assets,
                };
                const dest = {
                    V3: destMultilocation,
                };
                const tx = polkadotJs.tx.xcmPallet.transferAssets(
                    dest,
                    versionedBeneficiary,
                    versionedAssets,
                    0,
                    "Unlimited"
                );

                if (shouldSkipStarlightXCM) {
                    console.log(`Skipping T01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock(await tx.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentDmpMessageFee(context, baseDelivery, txByteFee, 2000);
                const paid = await extractPaidDeliveryFeesDancelight(context);
                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(paid).toBeGreaterThanOrEqual(fee - 1n);
                expect(paid).toBeLessThanOrEqual(fee + 1n);
            },
        });
    },
});
