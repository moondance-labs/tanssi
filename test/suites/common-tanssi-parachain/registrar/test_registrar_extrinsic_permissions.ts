import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "COMMO1107",
    title: "Registrar extrinsics permissions",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        const paraId = 2001;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Para manager can execute registrar pallet extrinsics",
            test: async () => {
                await context.createBlock();

                // Bob is not a manager, extrinsic requiring RegistrarOrigin should fail with BadOrigin error
                const { result: pauseContainerResultAttempt1 } = await context.createBlock(
                    await polkadotJs.tx.registrar.pauseContainerChain(paraId).signAsync(bob)
                );
                expect(pauseContainerResultAttempt1.successful).toEqual(false);
                expect(pauseContainerResultAttempt1.error.name).toEqual("BadOrigin");

                // Set bob as manager
                const tx = polkadotJs.tx.registrar.setParaManager(paraId, bob.address);

                const { result: sudoResult } = await context.createBlock(
                    await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)
                );

                expect(sudoResult.successful).toEqual(true);

                // Now it should show ParaIdNotRegistered error but not the BadOrigin
                const { result: pauseContainerResultAttempt2 } = await context.createBlock(
                    await polkadotJs.tx.registrar.pauseContainerChain(paraId).signAsync(bob)
                );

                expect(pauseContainerResultAttempt2.successful).toEqual(true);
            },
        });
    },
});
