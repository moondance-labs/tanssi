import { expect, describeSuite } from "@moonwall/cli";
import { STATEMINT_LOCATION_EXAMPLE } from "../../../util/constants.ts";
import { alith } from "@moonwall/util";

describeSuite({
    id: "DF0201",
    title: "Ethereum asset dummy precompile address creation",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "dummy precompile address is created when creating the asset and removed when destroyed",
            test: async function () {
                const assetId = 5;
                const assetIdAddress = new Uint8Array([
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 5,
                ]);
                const revertBytecode = "0x60006000fd";
                const addressInHex = "0x" + Buffer.from(assetIdAddress).toString("hex");

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(
                            context
                                .polkadotJs()
                                .tx.foreignAssetsCreator.createForeignAsset(
                                    STATEMINT_LOCATION_EXAMPLE,
                                    assetId,
                                    alith.address,
                                    true,
                                    1
                                )
                        )
                );

                // After the foreign asset creation, the address should contain revert byte code.
                expect(await context.web3().eth.getCode(addressInHex)).to.equal(revertBytecode);

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(context.polkadotJs().tx.foreignAssetsCreator.destroyForeignAsset(assetId))
                );

                // After the foreign asset destruction, the revert bytecode from that address should be removed.
                expect(await context.web3().eth.getCode(addressInHex)).to.equal("0x");
            },
        });
    },
});
