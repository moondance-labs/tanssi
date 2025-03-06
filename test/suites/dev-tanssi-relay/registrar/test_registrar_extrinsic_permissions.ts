import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT1601",
    title: "Registrar extrinsics permissions",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        const paraId = 2001;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            api = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Deregister through extrinsic should fail",
            test: async () => {
                const runtimeName = api.runtimeVersion.specName.toString();
                console.log("runtimeName", runtimeName);

                const { result: deregisterResult } = await context.createBlock(
                    await api.tx.registrar.deregister(paraId).signAsync(bob)
                );
                expect(deregisterResult.successful).toEqual(false);
                expect(deregisterResult.error.name).toEqual("CallFiltered");
            },
        });
    },
});
