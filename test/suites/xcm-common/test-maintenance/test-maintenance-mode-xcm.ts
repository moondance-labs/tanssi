import "@polkadot/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "../../../util/block";
import { MultiLocation } from "../../../util/xcm";

describeSuite({
    id: "CX0102",
    title: "XCM in maintenance mode",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: string;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            polkadotJs = context.pjsApi;
            chain = polkadotJs.consts.system.version.specName.toString();
            alice = context.keyring.alice;
        });

        it({
            id: "E01",
            title: "polkadotXcm calls disabled in maintenance mode",
            test: async function () {
                await context.createBlock();
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

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

                const polkadotXcmSend = context.polkadotJs().tx.polkadotXcm.send(dest, message);

                if (chain == "frontier-template") {
                    expect(
                        async () => await context.createBlock(polkadotXcmSend.signAsync(alice))
                    ).rejects.toThrowError("1010: Invalid Transaction: Transaction call is not expected");
                } else {
                    const { result } = await context.createBlock([await polkadotXcmSend.signAsync(alice)]);
                    expect(result[0].successful).to.be.false;
                    expect(result[0].error.name).to.eq("CallFiltered");
                }
            },
        });

        it({
            id: "E02",
            title: "polkadotXcm calls enabled with sudo in maintenance mode",
            test: async function () {
                await context.createBlock();
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

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

                const polkadotXcmSend = context.polkadotJs().tx.polkadotXcm.send(dest, message);

                const { result } = await context.createBlock([
                    await polkadotJs.tx.sudo.sudo(polkadotXcmSend).signAsync(alice),
                ]);

                // Search for ExtrinsicSuccess event
                const events = (await context.polkadotJs().query.system.events()).filter(({ event }) =>
                    context.polkadotJs().events.system.ExtrinsicSuccess.is(event)
                );
                expect(events.length).toBeGreaterThanOrEqual(1);
                expect(result[0].successful).to.be.true;
                expect(result[0].error).to.be.undefined;
            },
        });

        it({
            id: "E03",
            title: "polkadotXcm calls allowed again after disabling maintenance mode",
            test: async function () {
                await context.createBlock();
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.resumeNormalOperation();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.false;

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

                const polkadotXcmSend = context.polkadotJs().tx.polkadotXcm.send(dest, message);
                const { result } = await context.createBlock([await polkadotXcmSend.signAsync(alice)]);

                // Search for ExtrinsicSuccess event
                const events = (await context.polkadotJs().query.system.events()).filter(({ event }) =>
                    context.polkadotJs().events.system.ExtrinsicSuccess.is(event)
                );
                expect(events.length).toBeGreaterThanOrEqual(1);
                expect(result[0].successful).to.be.true;
                expect(result[0].error).to.be.undefined;
            },
        });
    },
});
