import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { type MultiLocation, XcmFragment, extractPaidDeliveryFeesDancelight, getLastSentDmpMessageFee } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_XCM, checkCallIsFiltered } from "helpers";

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
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightXCM: boolean;

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
            title: "Should succeed sending a XCM downward",
            test: async () => {
                const xcmMessage = new XcmFragment({
                    assets: [],
                })
                    .clear_origin()
                    .as_v3();

                const destMultilocation: MultiLocation = {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                };

                const dest = {
                    V3: destMultilocation,
                };
                const txRoot = polkadotJs.tx.xcmPallet.send(dest, xcmMessage);

                if (shouldSkipStarlightXCM) {
                    console.log(`Skipping T01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await txRoot.signAsync(alice));
                    return;
                }

                await context.createBlock(await txRoot.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentDmpMessageFee(context, baseDelivery, txByteFee, 2000);
                const paid = await extractPaidDeliveryFeesDancelight(context);
                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(paid).toBeGreaterThanOrEqual(fee - 1n);
                expect(paid).toBeLessThanOrEqual(fee + 1n);
            },
        });
    },
});
