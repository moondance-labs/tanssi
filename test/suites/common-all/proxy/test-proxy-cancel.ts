import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "C0301",
    title: "Proxy test suite - ProxyType::CancelProxy",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let chain: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightProxy: boolean;

        beforeAll(() => {
            initializeCustomCreateBlock(context);
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            isStarlight = chain === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightProxy = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY.includes(specVersion);
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
            title: "Add proxy Any",
            test: async () => {
                const delegate = bob.address;
                const delay = 3;
                const tx = polkadotJs.tx.proxy.addProxy(delegate, "Any", delay);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock();
                await context.createBlock([await tx.signAsync(alice)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate,
                        proxyType: "Any",
                        delay,
                    },
                ]);
            },
        });

        it({
            id: "E03",
            title: "Add proxy CancelProxy",
            test: async () => {
                const delegate = charlie.address;
                const cancelProxy = ["frontier-template", "container-chain-template"].includes(chain) ? 3 : 4;
                const delay = 0;
                const tx = polkadotJs.tx.proxy.addProxy(delegate, cancelProxy, delay);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate: bob.address,
                        proxyType: "Any",
                        delay: 3,
                    },
                    {
                        delegate: charlie.address,
                        proxyType: "CancelProxy",
                        delay: 0,
                    },
                ]);
            },
        });

        it({
            id: "E04",
            title: "Delegate account can call proxy.rejectAnnouncement",
            test: async () => {
                await context.createBlock();

                // Bob announces a transfer call
                const balanceCall = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const callHash = balanceCall.method.hash.toString();
                const tx1 = polkadotJs.tx.proxy.announce(alice.address, callHash);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx1.signAsync(bob));
                    return;
                }

                await context.createBlock([await tx1.signAsync(bob)]);
                let events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "Announced";
                });
                expect(ev1.length).to.be.equal(1);

                // Charlie can reject the announcement
                const tx2 = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.proxy.rejectAnnouncement(bob.address, callHash)
                );
                await context.createBlock([await tx2.signAsync(charlie)]);
                events = await polkadotJs.query.system.events();
                const ev2 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev2.length).to.be.equal(1);
                expect(ev2[0].event.data[0].toString()).to.be.eq("Ok");

                // Wait for the proxy delay
                await context.createBlock();
                await context.createBlock();
                await context.createBlock();
                await context.createBlock();

                // Anyone can try to execute the announced call, but it will fail since it has been rejected
                const tx3 = polkadotJs.tx.proxy.proxyAnnounced(bob.address, alice.address, null, balanceCall);
                await context.createBlock([await tx3.signAsync(dave)]);

                events = await polkadotJs.query.system.events();
                const ev3 = events.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev3.length).to.be.equal(1);
            },
        });

        it({
            id: "E05",
            title: "Unauthorized account cannot reject announcement",
            test: async () => {
                // Bob announces a transfer call
                const balanceCall = polkadotJs.tx.balances.transferAllowDeath(bob.address, 200_000);
                const callHash = balanceCall.method.hash.toString();
                const tx1 = polkadotJs.tx.proxy.announce(alice.address, callHash);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E05 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx1.signAsync(bob));
                    return;
                }

                await context.createBlock([await tx1.signAsync(bob)]);
                let events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "Announced";
                });
                expect(ev1.length).to.be.equal(1);

                // Dave cannot reject the announcement
                const tx2 = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.proxy.rejectAnnouncement(bob.address, callHash)
                );
                await context.createBlock([await tx2.signAsync(dave)]);
                events = await polkadotJs.query.system.events();
                const ev2 = events.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev2.length).to.be.equal(1);

                // Wait for the proxy delay
                await context.createBlock();
                await context.createBlock();
                await context.createBlock();
                await context.createBlock();

                // Anyone can try to execute the announced call
                const tx3 = polkadotJs.tx.proxy.proxyAnnounced(bob.address, alice.address, null, balanceCall);
                await context.createBlock([await tx3.signAsync(dave)]);

                events = await polkadotJs.query.system.events();
                const ev3 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev3.length).to.be.equal(1);
                expect(ev3[0].event.data[0].toString()).to.be.eq("Ok");
            },
        });

        it({
            id: "E06",
            title: "Delegate account cannot call balance.transfer",
            test: async () => {
                if (!chain.includes("light")) {
                    await context.createBlock();
                }

                const tx = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.balances.transferAllowDeath(charlie.address, 200_000)
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E06 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(charlie));
                    return;
                }

                await context.createBlock([await tx.signAsync(charlie)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.not.be.eq("Ok");
            },
        });
    },
});
