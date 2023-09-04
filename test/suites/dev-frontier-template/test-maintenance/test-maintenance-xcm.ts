import { describeSuite, expect, beforeAll, beforeEach } from "@moonwall/cli";
import { alith } from "@moonwall/util";
import { MultiLocation } from "../../../util/xcm";

// When in maintenance mode:
// A call from root (sudo) can make a transfer directly to polkadotXcm pallet
// A signed call cannot make a call directly in polkadotXcm pallet
describeSuite({
    id: "DF0802",
    title: "XCM - maintenance <> MaintenanceFilter",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
            // Enter maintenance mode
            const tx = context.polkadotJs().tx.maintenanceMode.enterMaintenanceMode();
            await context.createBlock([await context.polkadotJs().tx.sudo.sudo(tx).signAsync(alith)]);

            // Search for EnteredMaintenanceMode event
            const events = (await context.polkadotJs().query.system.events()).filter(({ event }) =>
                context.polkadotJs().events.maintenanceMode.EnteredMaintenanceMode.is(event)
            );
            expect(events).to.have.lengthOf(1);
        });

        beforeEach(async () => {
            // Check maintenance mode is enabled
            const enabled = (await context.polkadotJs().query.maintenanceMode.maintenanceMode()).toJSON();
            expect(enabled).to.be.true;
        });

        it({
            id: "T01",
            title: "should fail to call polkadotXcm without sudo",
            test: async function () {
                const destMultilocation: MultiLocation = {
                    parents: 1,
                    interior: { Here: null },
                };

                const dest = {
                    V3: destMultilocation,
                };

                const message = {
                    V3: [{ ClearOrigin: null }],
                };

                const tx = context.polkadotJs().tx.polkadotXcm.send(dest, message);
                expect(async () => await context.createBlock(tx.signAsync(alith))).rejects.toThrowError(
                    "1010: Invalid Transaction: Transaction call is not expected"
                );
            },
        });

        it({
            id: "T02",
            title: "should succeed to call polkadotXcm with sudo",
            test: async function () {
                const destMultiLocation: MultiLocation = {
                    parents: 1,
                    interior: { Here: null },
                };

                const dest = {
                    V3: destMultiLocation,
                };

                const message = {
                    V3: [{ ClearOrigin: null }],
                };

                await context.createBlock(
                    context.polkadotJs().tx.sudo.sudo(context.polkadotJs().tx.polkadotXcm.send(dest, message))
                );

                // Search for ExtrinsicSuccess event
                const events = (await context.polkadotJs().query.system.events()).filter(({ event }) =>
                    context.polkadotJs().events.system.ExtrinsicSuccess.is(event)
                );
                expect(events.length).toBeGreaterThanOrEqual(1);
            },
        });
    },
});
