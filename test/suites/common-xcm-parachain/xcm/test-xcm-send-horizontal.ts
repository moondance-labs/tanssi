import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    type MultiLocation,
    XcmFragment,
    extractPaidDeliveryFees,
    getLastSentHrmpMessageFee,
    mockHrmpChannelExistanceTx,
} from "utils";

describeSuite({
    id: "COMMON0308",
    title: "XCM - Succeeds sending XCM",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain: any;
        const destinationPara = 3000;
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
            title: "Should succeed sending a XCM horizontal",
            test: async () => {
                // We need to first mock the existence of the channel
                const mockHrmp3000Tx = polkadotJs.tx.sudo.sudo(
                    mockHrmpChannelExistanceTx(context, destinationPara, 1000, 102400, 102400)
                );
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

                const xcmMessage = new XcmFragment({
                    assets: [],
                })
                    .clear_origin()
                    .as_v3();

                const destMultilocation: MultiLocation = {
                    parents: 1,
                    interior: {
                        X1: {
                            Parachain: destinationPara,
                        },
                    },
                };

                const dest = {
                    V3: destMultilocation,
                };
                const tx = polkadotJs.tx.polkadotXcm.send(dest, xcmMessage);

                await context.createBlock(
                    [
                        await mockHrmp3000Tx.signAsync(alice, { nonce: aliceNonce++ }),
                        await tx.signAsync(alice, { nonce: aliceNonce++ }),
                    ],
                    { allowFailures: true }
                );

                const fee = await getLastSentHrmpMessageFee(context, destinationPara, baseDelivery, txByteFee);
                const paid = await extractPaidDeliveryFees(context);
                expect(paid).to.be.equal(fee);
            },
        });
    },
});
