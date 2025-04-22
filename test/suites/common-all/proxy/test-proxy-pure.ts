import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "utils";
import {STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY, checkCallIsFiltered} from "helpers"

describeSuite({
    id: "C0302",
    title: "Proxy test suite - create_pure",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let proxyAddress: string;
        let chain: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightProxy: boolean;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
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
            title: "Add pure proxy",
            test: async () => {
                const delay = 0;
                const index = 0;
                const tx = polkadotJs.tx.proxy.createPure("Any", delay, index);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "PureCreated";
                });
                expect(ev1.length).to.be.equal(1);
                proxyAddress = ev1[0].event.toJSON().data[0];
            },
        });

        it({
            id: "E03",
            title: "Pure proxy account can call balance.transfer",
            test: async () => {
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

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);
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
            title: "Pure proxy account can call system.remark",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.proxy(
                    proxyAddress,
                    null,
                    polkadotJs.tx.system.remarkWithEvent("I was called through using proxy.proxy")
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await context.createBlock([await tx.signAsync(alice)]);
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ProxyExecuted";
                });
                expect(ev1.length).to.be.equal(1);
                expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");

                const ev2 = events.filter((a) => {
                    return a.event.method === "Remarked";
                });
                expect(ev2.length).to.be.equal(1);
            },
        });
    },
});
