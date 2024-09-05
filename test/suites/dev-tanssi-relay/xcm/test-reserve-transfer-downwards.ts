import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { MultiLocation, extractPaidDeliveryFeesStarlight, getLastSentDmpMessageFee } from "../../../util/xcm";
import { ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
    id: "CX0204",
    title: "XCM - Succeeds sending XCM reserve transfer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain;
        const UNITS = 1_000_000_000_000n;
        const CENTS = UNITS / 30_000n;
        const MILICENTS = CENTS / 1000n;
        const txByteFee = 10n * MILICENTS;

        const randomReceiver = "0x1111111111111111111111111111111111111111111111111111111111111111";

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            baseDelivery = 100_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed sending a reserve transfer downward",
            test: async function () {
                const destMultilocation: MultiLocation = {
                    parents: 0,
                    interior: { X1: { Parachain: 1001 } },
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

                await context.createBlock(await tx.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentDmpMessageFee(context, baseDelivery, txByteFee, 1001);
                const paid = await extractPaidDeliveryFeesStarlight(context);
                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(paid).toBeGreaterThanOrEqual(fee - 1n);
                expect(paid).toBeLessThanOrEqual(fee + 1n);
            },
        });
    },
});
