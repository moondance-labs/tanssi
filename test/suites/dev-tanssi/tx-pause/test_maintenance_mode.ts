import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { DANCE } from "util/constants";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Result } from "@polkadot/types-codec";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";

describeSuite({
    id: "DT0801",
    title: "Pausing is compatible with maintenance mode",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
        });

        it({
            id: "E01",
            title: "a paused tx should still fail during maintenance mode",
            test: async function () {
                // Pause Balances.transfer_allow_death
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.txPause.pause(["Balances", "transfer_allow_death"]))
                        .signAsync(alice)
                );
                expect(result.successful).to.be.true;

                // Check sudo was successful
                const sudoEvents = result.events.filter(({ event: { method } }) => method === "Sudid");
                expect(sudoEvents.length).toBe(1);
                expect((sudoEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;

                // Enable maintenance mode
                await context.createBlock(
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.maintenanceMode.enterMaintenanceMode()).signAsync(alice)
                );
                expect((await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON()).to.be.true;

                // transfer_allow_death should fail
                const { result: resultTransfer } = await context.createBlock(
                    polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice)
                );

                expect(resultTransfer.successful).to.be.false;
                expect(resultTransfer.error.name).to.eq("CallFiltered");
            },
        });

        it({
            id: "E02",
            title: "a paused tx should still fail after maintenance mode",
            test: async function () {
                // Disable maintenance mode
                await context.createBlock(
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.maintenanceMode.resumeNormalOperation()).signAsync(alice)
                );
                expect((await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON()).to.be.false;

                await context.createBlock();

                // transfer should still fail
                const { result } = await context.createBlock(
                    polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice)
                );

                expect(result.successful).to.be.false;
                expect(result.error.name).to.eq("CallFiltered");
            },
        });
    },
});
