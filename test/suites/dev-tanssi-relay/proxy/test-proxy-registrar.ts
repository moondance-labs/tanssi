import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock, extractFeeAuthor, filterRewardFromContainer } from "../../../util/block";

describeSuite({
    id: "DTR1201",
    title: "Proxy test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            alice = context.keyring.alice;
            bob = context.keyring.bob;
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
            title: "Add proxy",
            test: async function () {
                await context.createBlock();

                const delegate = bob.address;
                const tx = polkadotJs.tx.proxy.addProxy(delegate, "Any", 0);
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate,
                        proxyType: "Any",
                        delay: 0,
                    },
                ]);
            },
        });

        it({
            id: "E03",
            title: "Delegate account can call proxy.proxy",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000)
                );
                await context.createBlock([await tx.signAsync(bob)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");

                const fee = extractFeeAuthor(events, bob.address).amount.toBigInt();
                const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free.toBigInt();

                // Balance of Bob account increased
                // (balanceBefore - fee) is the balance that the account would have if the extrinsic failed
                expect(balanceAfter > balanceBefore - fee).to.be.true;
            },
        });

        it({
            id: "E04",
            title: "Unauthorized account cannot call proxy.proxy",
            test: async function () {
                await context.createBlock();

                const balanceBefore = (await polkadotJs.query.system.account(charlie.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(charlie.address, 200_000)
                );
                await context.createBlock([await tx.signAsync(charlie)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ExtrinsicFailed";
                });
                expect(ev1.length).to.be.equal(1);

                // Charlie receives rewards for authoring container, we should take this into account
                const fee = extractFeeAuthor(events, charlie.address).amount.toBigInt();
                const receivedReward = filterRewardFromContainer(events, charlie.address, 2000);

                const balanceAfter = (await polkadotJs.query.system.account(charlie.address)).data.free.toBigInt();

                // Balance of Charlie account must be the same (minus fee)
                expect(balanceBefore + receivedReward - fee).to.equal(balanceAfter);
            },
        });
    },
});
