import { describeSuite, expect, beforeEach } from "@moonwall/cli";
import { alith } from "@moonwall/util";
import { MultiLocation } from "../../../util/xcm";

// Here we will test that NormalFilter of MaintenanceMode pallet works fine.
// This means that only one XCM call can be done -> 'forceDefaultXcmVersion',
// and only root can do it (that's how it's defined inside the extrinsic).
describeSuite({
    id: "DF0801",
    title: "XCM - maintenance <> NormalFilter",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeEach(async () => {
            // Check maintenance mode is disabled
            const enabled = (await context.polkadotJs().query.maintenanceMode.maintenanceMode()).toJSON();
            expect(enabled).to.be.false;
        });

        it({
            id: "T01",
            title: "should fail to call forceDefaultXcmVersion without sudo",
            test: async function () {
                const tx = context.polkadotJs().tx.polkadotXcm.forceDefaultXcmVersion(3);
                const { result } = await context.createBlock(tx.signAsync(alith));
                expect(result.successful).to.be.false;
                expect(result.error.name).to.equal("BadOrigin");
            },
        });

        it({
            id: "T02",
            title: "should succeed to call forceDefaultXcmVersion with sudo",
            test: async function () {
                const tx = context.polkadotJs().tx.polkadotXcm.forceDefaultXcmVersion(3);
                const { result } = await context.createBlock(context.polkadotJs().tx.sudo.sudo(tx));
                expect(result.successful).to.be.true;
                expect(result.error).to.be.undefined;
            },
        });

        it({
            id: "T03",
            title: "should fail to call another extrinsic different from forceDefaultXcmVersion",
            test: async function () {
                // Try with polkadotXcm 'forceSuspension' extrinsic
                const forceSuspensionTx = context.polkadotJs().tx.polkadotXcm.forceSuspension(true);

                // Should fail because the only allowed call is 'forceDefaultXcmVersion'
                expect(async () => await context.createBlock(forceSuspensionTx.signAsync(alith))).rejects.toThrowError(
                    "1010: Invalid Transaction: Transaction call is not expected"
                );

                // Now let's try with polkadotXcm 'send' extrinsic
                const destMultilocation: MultiLocation = {
                    parents: 0,
                    interior: { Here: null },
                };

                const dest = {
                    V3: destMultilocation,
                };

                const message = {
                    V3: [{ ClearOrigin: null }],
                };

                const sendTx = context.polkadotJs().tx.polkadotXcm.send(dest, message);

                // Should fail because the only allowed call is 'forceDefaultXcmVersion'
                expect(async () => await context.createBlock(sendTx.signAsync(alith))).rejects.toThrowError(
                    "1010: Invalid Transaction: Transaction call is not expected"
                );
            },
        });
    },
});
