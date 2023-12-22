import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock, extractFeeAuthor, filterRewardFromContainer } from "../../../util/block";

describeSuite({
    id: "C0103",
    title: "Proxy test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let chain: string;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;

            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
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

        it({
            id: "E05",
            title: "Can add multiple proxy types to the same delegator",
            test: async function () {
                await context.createBlock();

                const delegate = dave.address;
                const txs = [];

                // All proxy types that do not allow balance transfer
                // Frontier chains -> NonTransfer = 1, Governance = 2, CancelProxy = 3
                // Other chains -> NonTransfer = 1, Governance = 2, Staking = 3, CancelProxy = 4
                const proxyTypes = chain == "frontier-template" ? [1, 2, 3] : [1, 2, 3, 4];
                const nonce =
                    chain == "frontier-template"
                        ? (await polkadotJs.query.system.account(alice.address)).nonce
                        : await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);

                for (const [i, proxyType] of proxyTypes.entries()) {
                    const tx = polkadotJs.tx.proxy.addProxy(delegate, proxyType, 0);
                    txs.push(await tx.signAsync(alice, { nonce: nonce.addn(i) }));
                }
                await context.createBlock(txs);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(proxyTypes.length);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0].length).to.be.equal(proxyTypes.length + 1);
            },
        });

        it({
            id: "E06",
            title: "Account with no balance proxy cannot call balances.transfer",
            test: async function () {
                // Dave has multiple proxy types, but none of them allows to call balances.transfer
                const balanceBefore = (await polkadotJs.query.system.account(dave.address)).data.free.toBigInt();
                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(dave.address, 200_000)
                );
                await context.createBlock([await tx.signAsync(dave)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.not.be.eq("Ok");

                const fee = extractFeeAuthor(events, dave.address).amount.toBigInt();
                const balanceAfter = (await polkadotJs.query.system.account(dave.address)).data.free.toBigInt();

                // Balance of Dave account must be the same (minus fee)
                expect(balanceBefore - fee).to.equal(balanceAfter);
            },
        });

        it({
            id: "E07",
            title: "Account with non transfer proxy can call system.remark",
            test: async function () {
                await context.createBlock();

                // Dave has NonTransfer proxy, that allows to call system.remark
                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.system.remarkWithEvent("I was called through using proxy.proxy")
                );
                await context.createBlock([await tx.signAsync(dave)]);

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
