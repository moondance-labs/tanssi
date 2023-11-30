import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { DANCE } from "util/constants";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Result } from "@polkadot/types-codec";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";

describeSuite({
    id: "DT0802",
    title: "Txs can be paused and unpaused",
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
            title: "transfer should fail after pausing it",
            test: async function () {
                // Pause Balances.transfer
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.txPause.pause(["Balances", "transfer_allow_death"])).signAsync(alice)
                );

                expect(result.successful).to.be.true;
                // Check sudo was successful
                const sudoEvents = result.events.filter(({ event: { method } }) => method === "Sudid");
                expect(sudoEvents.length).toBe(1);
                expect((sudoEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;

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
            title: "transfer should succeed after unpausing it",
            test: async function () {
                // Unpause Balances.transferAllowDeath
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.txPause.unpause(["Balances", "transfer_allow_death"])).signAsync(alice),
                    {
                        // allowFailures: true,
                        expectEvents: [context.polkadotJs().events.sudo.Sudid],
                    }
                );
                expect(result.successful).to.be.true;

                // Check sudo was successful
                const sudoEvents = result.events.filter(({ event: { method } }) => method === "Sudid");
                expect(sudoEvents.length).toBe(1);
                expect((sudoEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;

                // transfer_allow_death should fail
                const { result: resultTransfer } = await context.createBlock(
                    polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice)
                );

                expect(resultTransfer.successful).to.be.true;
            },
        });
    },
});
