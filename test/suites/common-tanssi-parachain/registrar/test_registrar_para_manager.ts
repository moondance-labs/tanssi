import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { generateEmptyGenesisData } from "utils";

describeSuite({
    id: "COMMO1102",
    title: "Registrar para manager",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        const paraId = 2002;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Para manager can be set and is recognized as ManagerOrigin",
            test: async () => {
                await context.createBlock();
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                await context.createBlock([
                    await polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null).signAsync(alice),
                ]);

                // Bob still not a manager, extrinsic requiring ManagerOrigin should fail
                const { result } = await context.createBlock(
                    await polkadotJs.tx.servicesPayment.setRefundAddress(paraId, bob.address).signAsync(bob)
                );
                expect(result.successful).to.be.false;

                // Set bob as manager
                await context.createBlock([
                    await polkadotJs.tx.registrar.setParaManager(paraId, bob.address).signAsync(alice),
                ]);

                // Extrinsic should succeed now
                const { result: result2 } = await context.createBlock(
                    await polkadotJs.tx.servicesPayment.setRefundAddress(paraId, bob.address).signAsync(bob)
                );

                expect(result2.successful).to.be.true;
            },
        });
    },
});
