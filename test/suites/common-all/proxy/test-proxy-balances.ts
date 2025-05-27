import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "utils";

describeSuite({
    id: "C0304",
    title: "Proxy test suite - ProxyType::Balances",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let chain: string;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
        });

        it({
            id: "E01",
            title: "No proxies at genesis",
            test: async () => {
                await context.createBlock();
                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([]);
            },
        });

        it({
            id: "E02",
            title: "Add proxy Balances",
            test: async () => {
                const delegate = charlie.address;
                const delay = 0;
                const tx = polkadotJs.tx.proxy.addProxy(delegate, "Balances", delay);
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate: charlie.address,
                        proxyType: "Balances",
                        delay: 0,
                    },
                ]);
            },
        });

        it({
            id: "E03",
            title: "Delegate account can call balance.transfer",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(charlie.address, 200_000)
                );
                await context.createBlock([await tx.signAsync(charlie)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");
            },
        });

        it({
            id: "E04",
            title: "Delegate account cannot call system.remark",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.system.remarkWithEvent("I was called through using proxy.proxy")
                );
                await context.createBlock([await tx.signAsync(charlie)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.not.be.eq("Ok");

                const ev2 = events.filter((a) => {
                    return a.event.method === "Remarked";
                });
                expect(ev2.length).to.be.equal(0);
            },
        });
    },
});
