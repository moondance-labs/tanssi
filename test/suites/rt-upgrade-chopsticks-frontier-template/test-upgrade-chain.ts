import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { alith, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { chopsticksWaitTillIncluded } from "utils";

const MAX_BALANCE_TRANSFER_TRIES = 5;
describeSuite({
    id: "R01",
    title: "Chopsticks Frontier Template Upgrade Test",
    foundationMethods: "chopsticks",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let xcmQueryToAnalyze: number;

        beforeAll(async () => {
            api = context.polkadotJs();

            const rtBefore = api.consts.system.version.specVersion.toNumber();
            log("About to upgrade to runtime at:");
            log((await MoonwallContext.getContext()).rtUpgradePath);

            await context.upgradeRuntime();

            const rtafter = api.consts.system.version.specVersion.toNumber();

            if (rtBefore === rtafter) {
                throw new Error("Runtime upgrade failed");
            }

            log(`RT upgrade has increased specVersion from ${rtBefore} to ${rtafter}`);

            const specName = api.consts.system.version.specName.toString();
            log(`Currently connected to chain: ${specName}`);

            // Inject a forceSubscribe version to create a query to check migration to xcm
            const queryLocation = {
                parents: 1,
                interior: "Here",
            };
            // fetch on-chain later
            // TODO:Once we update the next runtime, remove this and take it form onchain
            const previousXcmVersion = 5;
            const latestVersion = "V" + previousXcmVersion.toString();

            const versionedLocation = {
                [latestVersion]: queryLocation,
            };
            xcmQueryToAnalyze = await api.query.polkadotXcm.queryCounter();
            const finalTx = api.tx.sudo.sudo(api.tx.polkadotXcm.forceSubscribeVersionNotify(versionedLocation));
            await chopsticksWaitTillIncluded(context, api, alith, finalTx);
        });

        it({
            id: "T1",
            timeout: 60000,
            title: "Can create new blocks",
            test: async () => {
                const currentHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock({ count: 2 });
                const newHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(newHeight - currentHeight).to.be.equal(2);
            },
        });
        it({
            id: "T2",
            timeout: 60000,
            title: "Can send balance transfers",
            test: async () => {
                const randomAccount = generateKeyringPair("ethereum");

                const balanceBefore = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();

                const balanceTransferTx = await api.tx.balances.transferAllowDeath(
                    randomAccount.address,
                    1_000_000_000
                );

                await chopsticksWaitTillIncluded(context, api, alith, balanceTransferTx);

                const balanceAfter = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });
        it({
            id: "T3",
            title: "Xcm migrations have runned, if any",
            test: async () => {
                const currentXcmVersion = await api.consts.polkadotXcm.advertisedXcmVersion.toNumber();
                const query = await api.query.polkadotXcm.queries(xcmQueryToAnalyze);
                const version = Object.keys(query.toJSON()["versionNotifier"]["origin"])[0];
                expect(version).to.be.equal("v" + currentXcmVersion.toString());
            },
        });
    },
});
