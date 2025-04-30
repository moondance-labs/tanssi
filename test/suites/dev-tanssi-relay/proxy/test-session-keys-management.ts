import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1503",
    title: "Proxy test suite - ProxyType::SessionKeyManagement",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        const sessionKeysManagementProxy = 9;
        const someKeys = "0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF";
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightProxy: boolean;

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightProxy = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Delegate account can manage keys",
            test: async () => {
                const delegator_alice = context.keyring.alice;
                const delegate_charlie = context.keyring.charlie;

                let tx = polkadotJs.tx.proxy.addProxy(delegate_charlie.address, sessionKeysManagementProxy, 0);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(delegator_alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(delegator_alice)]);

                let events = await polkadotJs.query.system.events();
                let ev1 = events.filter((a) => {
                    return a.event.method === "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                await context.createBlock();

                tx = polkadotJs.tx.proxy.proxy(
                    delegator_alice.address,
                    null,
                    polkadotJs.tx.session.setKeys(someKeys, "0x")
                );
                await context.createBlock([await tx.signAsync(delegate_charlie)]);
                events = await polkadotJs.query.system.events();
                ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");
            },
        });

        it({
            id: "E02",
            title: "Non-Delegate account fails to manage other account's keys",
            test: async () => {
                const alice = context.keyring.alice;
                const non_delegate_dave = context.keyring.dave;

                await context.createBlock();

                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.session.setKeys(
                        "0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF",
                        "0x"
                    )
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(non_delegate_dave));
                    return;
                }

                await context.createBlock([await tx.signAsync(non_delegate_dave)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(0);
            },
        });
    },
});
