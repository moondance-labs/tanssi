import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "COMM0301",
    title: "Registrar extrinsics permissions",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        const paraId = 2001;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightCR: boolean;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            api = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Para manager can execute registrar pallet extrinsics",
            test: async () => {
                const runtimeName = api.runtimeVersion.specName.toString();

                isStarlight = runtimeName === "starlight";
                specVersion = api.consts.system.version.specVersion.toNumber();
                shouldSkipStarlightCR = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR.includes(specVersion);

                const registerAlias = runtimeName.includes("light") ? api.tx.containerRegistrar : api.tx.registrar;

                const pausedSignedTx = await registerAlias.pauseContainerChain(paraId).signAsync(bob);

                if (shouldSkipStarlightCR) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, api, pausedSignedTx);

                    // SetParaManager (without sudo) should be also filtered
                    const signedTx = await registerAlias.setParaManager(paraId, bob.address).signAsync(alice);
                    await checkCallIsFiltered(context, api, signedTx);
                    return;
                }

                // Bob is not a manager, extrinsic requiring RegistrarOrigin should fail with BadOrigin error
                const { result: pauseContainerResultAttempt1 } = await context.createBlock(
                    pausedSignedTx
                );
                expect(pauseContainerResultAttempt1.successful).toEqual(false);
                expect(pauseContainerResultAttempt1.error.name).toEqual("BadOrigin");

                // Set bob as manager
                const { result: sudoResult } = await context.createBlock(
                    await api.tx.sudo.sudo(registerAlias.setParaManager(paraId, bob.address)).signAsync(alice)
                );

                expect(sudoResult.successful).toEqual(true);

                // Now it should succeed
                const { result: pauseContainerResultAttempt2 } = await context.createBlock(
                    await registerAlias.pauseContainerChain(paraId).signAsync(bob)
                );

                expect(pauseContainerResultAttempt2.successful).toEqual(true);
            },
        });
    },
});
