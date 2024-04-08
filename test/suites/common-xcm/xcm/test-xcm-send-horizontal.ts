import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import {
    MultiLocation,
    extractPaidDeliveryFees,
    getLastSentHrmpMessageFee,
    XcmFragment,
    mockHrmpChannelExistanceTx,
} from "../../../util/xcm";
import { ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
    id: "CX0204",
    title: "XCM - Succeeds sending XCM",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain;
        const destinationPara = 3000;
        const txByteFee = 1n;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            baseDelivery = chain == "frontier-template" ? 100_000_000_000_000n : 100_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed sending a XCM horizontal",
            test: async function () {
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
