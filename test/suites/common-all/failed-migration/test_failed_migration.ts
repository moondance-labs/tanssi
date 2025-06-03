import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "C0206",
    title: "Test failed multiblock migration",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice_or_alith: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            // This test will be run against frontier & substrate chains, hence the accounts used
            alice_or_alith = context.isEthereumChain ? alith : context.keyring.alice;
        });

        it({
            id: "E01",
            title: "Migrate runtime to same runtime and trigger stuck migrations error handler",
            test: async () => {
                const wasmCode = await polkadotJs.rpc.state.getStorage(":code");
                const wasmCodeHex = wasmCode.toHex();
                console.log(`Current runtime code prefix: ${wasmCodeHex.slice(0, 20)}...`);

                // Remove from storage the info about the current runtime version.
                // This ensures that the new runtime is considered new, because `runtime_upgraded()` returns true.
                // Without this on_runtime_upgrade will not be called.
                // https://github.com/paritytech/polkadot-sdk/blob/26afcd65438f5518bb6a0c281cfcc6159a60587e/substrate/frame/executive/src/lib.rs#L602
                const tx1 = polkadotJs.tx.system.killStorage([polkadotJs.query.system.lastRuntimeUpgrade.key()]);

                // Upgrade runtime without checks, to skip the check that "new version must be greater than current version"
                const tx2 = polkadotJs.tx.system.setCodeWithoutChecks(wasmCodeHex);

                // Mock migration cursor to be "Stuck".
                // Setting this value prevents including extrinsics in future blocks, so it must be set in the same block
                // as the runtime upgrade
                const tx3 = polkadotJs.tx.multiBlockMigrations.forceSetCursor("Stuck");

                await context.createBlock(
                    await polkadotJs.tx.utility
                        .batchAll([
                            polkadotJs.tx.sudo.sudo(tx1),
                            // Must use sudoUncheckedWeight because setCode takes 100% of block weight,
                            // so without this the node will not include the batch tx
                            polkadotJs.tx.sudo.sudoUncheckedWeight(tx2, { refTime: 1, proofSize: 1 }),
                            polkadotJs.tx.sudo.sudo(tx3),
                        ])
                        .signAsync(alice_or_alith)
                );

                // Cursor is stuck
                const cursor = await polkadotJs.query.multiBlockMigrations.cursor();
                console.log(cursor.toJSON());
                expect(cursor.unwrap().isStuck).to.be.true;

                // No maintenance mode yet
                const maintenanceMode = await polkadotJs.query.maintenanceMode.maintenanceMode();
                expect(maintenanceMode.toJSON()).to.be.false;

                // Create block to trigger on_runtime_upgrade
                await context.createBlock();

                // Cursor has been deleted
                const cursor2 = await polkadotJs.query.multiBlockMigrations.cursor();
                console.log(cursor2.toJSON());
                expect(cursor2.isNone).to.be.true;

                // Chain has entered maintenance mode
                const maintenanceMode2 = await polkadotJs.query.maintenanceMode.maintenanceMode();
                expect(maintenanceMode2.toJSON()).to.be.true;
            },
        });
    },
});
