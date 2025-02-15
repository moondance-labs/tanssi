import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type MultiLocation, extractPaidDeliveryFees, getLastSentUmpMessageFee, XcmFragment } from "../../../util/xcm";
import { type ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
    id: "COMMON0309",
    title: "XCM - Succeeds sending XCM",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain: any;
        const txByteFee = 1n;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            baseDelivery = chain === "frontier-template" ? 100_000_000_000_000n : 100_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed sending a XCM upward",
            test: async () => {
                const xcmMessage = new XcmFragment({
                    assets: [],
                })
                    .clear_origin()
                    .as_v3();

                const destMultilocation: MultiLocation = {
                    parents: 1,
                    interior: { Here: null },
                };

                const dest = {
                    V3: destMultilocation,
                };
                const txRoot = polkadotJs.tx.polkadotXcm.send(dest, xcmMessage);

                await context.createBlock(await txRoot.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentUmpMessageFee(context, baseDelivery, txByteFee);
                const paid = await extractPaidDeliveryFees(context);
                expect(paid).to.be.equal(fee);
            },
        });
    },
});
