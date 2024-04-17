import "@polkadot/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "../../../util/block";

describeSuite({
    id: "C0104",
    title: "Proxy test suite - create_pure",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let proxyAddress;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "No proxies at genesis",
            test: async function () {
                await context.createBlock();
                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([]);
            },
        });

        it({
            id: "E02",
            title: "Add pure proxy",
            test: async function () {
                const delay = 0;
                const index = 0;
                const tx = polkadotJs.tx.proxy.createPure("Any", delay, index);
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "PureCreated";
                });
                expect(ev1.length).to.be.equal(1);
                proxyAddress = ev1[0].event.toJSON().data[0];
            },
        });

        it({
            id: "E03",
            title: "Pure proxy account can call balance.transfer",
            test: async function () {
                await context.createBlock();

                // Send some initial balance to pure proxy account
                const existentialDeposit = polkadotJs.consts.balances.existentialDeposit.toBigInt();
                const tx1 = polkadotJs.tx.balances.transferAllowDeath(proxyAddress, existentialDeposit + 200_000n);
                await context.createBlock([await tx1.signAsync(alice)]);

                // Transfer from pure proxy to charlie
                const tx = polkadotJs.tx.proxy.proxy(
                    proxyAddress,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(charlie.address, 100_000n)
                );
                await context.createBlock([await tx.signAsync(alice)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");
            },
        });

        it({
            id: "E04",
            title: "Pure proxy account can call system.remark",
            test: async function () {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.proxy(
                    proxyAddress,
                    null,
                    polkadotJs.tx.system.remarkWithEvent("I was called through using proxy.proxy")
                );
                await context.createBlock([await tx.signAsync(alice)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");

                const ev2 = events.filter((a) => {
                    return a.event.method == "Remarked";
                });
                expect(ev2.length).to.be.equal(1);
            },
        });
    },
});
