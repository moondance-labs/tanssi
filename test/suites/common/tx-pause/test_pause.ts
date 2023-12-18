import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { DANCE } from "util/constants";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Result } from "@polkadot/types-codec";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";

describeSuite({
    id: "C0302",
    title: "Txs can be paused and unpaused",
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
            title: "transfer should fail after pausing it",
            test: async function () {
                await context.createBlock();
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
            title: "transfer should succeed after unpausing it",
            test: async function () {
                // Unpause Balances.transferAllowDeath
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.txPause.unpause(["Balances", "transfer_allow_death"]))
                        .signAsync(alice)
                );
                expect(result.successful).to.be.true;

                // Check sudo was successful
                const sudoEvents = result.events.filter(({ event: { method } }) => method === "Sudid");
                expect(sudoEvents.length).toBe(1);
                expect((sudoEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;

                // transfer_allow_death should succeed
                const { result: resultTransfer } = await context.createBlock(
                    polkadotJs.tx.balances.transferAllowDeath(bob.address, DANCE).signAsync(alice)
                );

                expect(resultTransfer.successful).to.be.true;
            },
        });

        it({
            id: "E03",
            title: "sudo shoudn't be affected by a pause",
            test: async function () {
                await context.createBlock();

                // Pause Balances.transfer
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.txPause.pause(["Balances", "force_transfer"]))
                        .signAsync(alice)
                );

                expect(result.successful).to.be.true;
                // Check sudo was successful
                const sudoEvents = result.events.filter(({ event: { method } }) => method === "Sudid");
                expect(sudoEvents.length).toBe(1);
                expect((sudoEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;

                // force_transfer should succeed
                const { result: resultTransfer } = await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.balances.forceTransfer(alice.address, bob.address, DANCE))
                        .signAsync(alice)
                );

                expect(resultTransfer.successful).to.be.true;
                // Check sudo was successful
                const transferEvents = resultTransfer.events.filter(({ event: { method } }) => method === "Sudid");
                expect(transferEvents.length).toBe(1);
                expect((transferEvents[0].event.data[0] as Result<any, SpRuntimeDispatchError>).isOk).to.be.true;
            },
        });
    },
});
