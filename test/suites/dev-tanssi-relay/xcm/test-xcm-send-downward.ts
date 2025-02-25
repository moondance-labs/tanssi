import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { type MultiLocation, XcmFragment, extractPaidDeliveryFeesDancelight, getLastSentDmpMessageFee } from "utils";

describeSuite({
    id: "DEVT1902",
    title: "XCM - Succeeds sending XCM",
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
        });

        it({
            id: "T01",
            title: "Should succeed sending a XCM downward",
            test: async () => {
                const xcmMessage = new XcmFragment({
                    assets: [],
                })
                    .clear_origin()
                    .as_v3();

                const destMultilocation: MultiLocation = {
                    parents: 0,
                    interior: { X1: { Parachain: 1001 } },
                };

                const dest = {
                    V3: destMultilocation,
                };
                const txRoot = polkadotJs.tx.xcmPallet.send(dest, xcmMessage);

                await context.createBlock(await txRoot.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentDmpMessageFee(context, baseDelivery, txByteFee, 1001);
                const paid = await extractPaidDeliveryFeesDancelight(context);
                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(paid).toBeGreaterThanOrEqual(fee - 1n);
                expect(paid).toBeLessThanOrEqual(fee + 1n);
            },
        });
    },
});
