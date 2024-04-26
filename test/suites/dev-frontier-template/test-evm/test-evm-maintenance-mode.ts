import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ALITH_ADDRESS, DEFAULT_GENESIS_BALANCE, BALTATHAR_ADDRESS, baltathar, alith } from "@moonwall/util";

// When in maintenance mode:
// A call from root (sudo) can make a transfer directly in pallet_evm
// A signed call cannot make a transfer directly in pallet_evm
describeSuite({
    id: "DF0801",
    title: "Pallet EVM - maintenance mode",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
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

                const tx = context
                    .polkadotJs()
                    .tx.evm.call(
                        ALITH_ADDRESS,
                        BALTATHAR_ADDRESS,
                        "0x0",
                        100_000_000_000_000_000_000n,
                        12_000_000n,
                        1_000_000_000n,
                        "0",
                        null,
                        []
                    );
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

                const { result } = await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(
                            context
                                .polkadotJs()
                                .tx.evm.call(
                                    ALITH_ADDRESS,
                                    baltathar.address,
                                    "0x0",
                                    100_000_000_000_000_000_000n,
                                    12_000_000n,
                                    100_000_000_000_000n,
                                    "0",
                                    null,
                                    []
                                )
                        )
                );

                expect(
                    result?.events.find(
                        ({ event: { section, method } }) => section == "system" && method == "ExtrinsicSuccess"
                    )
                ).to.exist;
                expect(await context.viem("public").getBalance({ address: baltathar.address })).to.equal(
                    DEFAULT_GENESIS_BALANCE + 100_000_000_000_000_000_000n
                );
            },
        });
    },
});
