import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { MultiLocation, extractPaidDeliveryFees, getLastSentUmpMessageFee } from "../../../util/xcm";
import { ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
    id: "CX0206",
    title: "XCM - Succeeds sending XCM reserve transfer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain;
        const txByteFee = 1n;
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
            baseDelivery = chain == "frontier-template" ? 100_000_000_000_000n : 100_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed sending a reserve transfer upward",
            test: async function () {
                // Get pallet indices
                const metadata = await context.polkadotJs().rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() == "Balances")!
                    .index.toNumber();

                const destMultilocation: MultiLocation = {
                    parents: 1,
                    interior: { Here: null },
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
                                interior: {
                                    X1: { PalletInstance: Number(balancesPalletIndex) },
                                },
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
                const tx = polkadotJs.tx.polkadotXcm.transferAssets(
                    dest,
                    versionedBeneficiary,
                    versionedAssets,
                    0,
                    "Unlimited"
                );

                await context.createBlock(await tx.signAsync(alice), { allowFailures: false });

                const fee = await getLastSentUmpMessageFee(context, baseDelivery, txByteFee);
                const paid = await extractPaidDeliveryFees(context);
                expect(paid).to.be.equal(fee);
            },
        });
    },
});
