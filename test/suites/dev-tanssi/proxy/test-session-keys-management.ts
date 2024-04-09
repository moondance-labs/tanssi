import "@polkadot/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DT0601",
    title: "Proxy test suite - ProxyType::SessionKeyManagement",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        const sessionKeysManagementProxy = 8;
        const someKeys = "0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF";

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Delegate account can manage keys",
            test: async function () {
                const delegator_alice = context.keyring.alice;
                const delegate_charlie = context.keyring.charlie;

                let tx = polkadotJs.tx.proxy.addProxy(delegate_charlie.address, sessionKeysManagementProxy, 0);
                await context.createBlock([await tx.signAsync(delegator_alice)]);

                let events = await polkadotJs.query.system.events();
                let ev1 = events.filter((a) => {
                    return a.event.method == "ProxyAdded";
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
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");
            },
        });

        it({
            id: "E02",
            title: "Non-Delegate account fails to manage other account's keys",
            test: async function () {
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
                await context.createBlock([await tx.signAsync(non_delegate_dave)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(0);
            },
        });
    },
});
