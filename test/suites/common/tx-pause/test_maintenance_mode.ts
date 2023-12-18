import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { DANCE } from "util/constants";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Result } from "@polkadot/types-codec";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";

describeSuite({
    id: "C0301",
    title: "Pausing is compatible with maintenance mode",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let chain: string;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            chain = polkadotJs.consts.system.version.specName.toString();
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

                const signedTx = polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice);

                // transfer_allow_death should fail
                if (chain == "frontier-template") {
                    expect(await context.createBlock(signedTx).catch((e) => e.toString())).to.equal(
                        "RpcError: 1010: Invalid Transaction: Transaction call is not expected"
                    );
                } else {
                    const { result: resultTransfer } = await context.createBlock(signedTx);

                    expect(resultTransfer.successful).to.be.false;
                    expect(resultTransfer.error.name).to.eq("CallFiltered");
                }
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

                const signedTx = polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice);

                // transfer_allow_death should fail
                if (chain == "frontier-template") {
                    expect(await context.createBlock(signedTx).catch((e) => e.toString())).to.equal(
                        "RpcError: 1010: Invalid Transaction: Transaction call is not expected"
                    );
                } else {
                    const { result: resultTransfer } = await context.createBlock(signedTx);

                    expect(resultTransfer.successful).to.be.false;
                    expect(resultTransfer.error.name).to.eq("CallFiltered");
                }
            },
        });
    },
});
