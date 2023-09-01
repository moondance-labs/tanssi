import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ALITH_ADDRESS, DEFAULT_GENESIS_BALANCE, BALTATHAR_ADDRESS, baltathar, alith } from "@moonwall/util";
import {} from "../../../util/xcm"

// When in maintenance mode:
// A call from root (sudo) can make a transfer directly to polkadotXcm pallet
// A signed call cannot make a call directly in polkadotXcm pallet
describeSuite({
    id: "DF0801",
    title: "XCM - maintenance mode",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
            // Enter maintenance mode
            const tx = context.polkadotJs().tx.maintenanceMode.enterMaintenanceMode();
            await context.createBlock([await context.polkadotJs().tx.sudo.sudo(tx).signAsync(alith)]);

            const events = await context.polkadotJs().query.system.events();
            const ev1 = events.filter((a) => {
                return a.event.method == "EnteredMaintenanceMode";
            });
            expect(ev1.length).to.be.equal(1);

            const enabled = (await context.polkadotJs().query.maintenanceMode.maintenanceMode()).toJSON();
            expect(enabled).to.be.true;
        });

        it({
            id: "T01",
            title: "should fail without sudo",
            test: async function () {
                const enabled = (await context.polkadotJs().query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

                const dest = {
                    V3: {
                        parents: 0,
                        interior: "Here"
                    }
                };

                const message = {
                    V3: [{ClearOrigin: null}]
                }

                const tx = context.polkadotJs().tx.polkadotXcm.send(dest, message);

                expect(await context.createBlock([await tx.signAsync(alith)]).catch((e) => e.toString())).to.equal(
                    "RpcError: 1010: Invalid Transaction: Transaction call is not expected"
                );

                expect(await context.viem("public").getBalance({ address: baltathar.address })).to.equal(
                    DEFAULT_GENESIS_BALANCE
                );
            },
        });

        it({
            id: "T02",
            title: "should succeed with sudo",
            test: async function () {
                const enabled = (await context.polkadotJs().query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

                const dest = {
                    V3: {
                        parents: 0,
                        interior: "Here"
                    }
                };

                const message = {
                    V3: [{ClearOrigin: null}]
                }

                const { result } = await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(
                            context.polkadotJs().tx.polkadotXcm.send(dest, message)
                        )
                );

                expect(
                    result?.events.find(
                        ({ event: { section, method } }) => section == "system" && method == "ExtrinsicSuccess"
                    )
                ).to.exist;
            },
        });
    },
});
