import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import type { PalletProxyProxyDefinition } from "@polkadot/types/lookup";
import chalk from "chalk";

describeSuite({
    id: "S05",
    title: "Verify account proxies created",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        const proxiesPerAccount: Map<string, { deposit: bigint; definitions: PalletProxyProxyDefinition[] }> =
            new Map();
        let atBlockNumber = 0;
        let apiAt: ApiDecoration<"promise">;
        let paraApi: ApiPromise;

        beforeAll(async () => {
            paraApi = context.polkadotJs("para");
            const limit = 1000;
            let last_key = "";
            let count = 0;

            // Configure the api at a specific block
            // (to avoid inconsistency querying over multiple block when the test takes a long time to
            // query data and blocks are being produced)
            atBlockNumber = process.env.BLOCK_NUMBER
                ? Number.parseInt(process.env.BLOCK_NUMBER)
                : (await paraApi.rpc.chain.getHeader()).number.toNumber();
            apiAt = await paraApi.at(await paraApi.rpc.chain.getBlockHash(atBlockNumber));

            for (;;) {
                const query = await apiAt.query.proxy.proxies.entriesPaged({
                    args: [],
                    pageSize: limit,
                    startKey: last_key,
                });

                if (query.length === 0) {
                    break;
                }
                count += query.length;

                // TEMPLATE: convert the data into the format you want (usually a dictionary per account)
                for (const proxyData of query) {
                    last_key = proxyData[0].toString();
                    proxiesPerAccount.set(proxyData[0].args[0].toString(), {
                        definitions: proxyData[1][0].toArray(),
                        deposit: proxyData[1][1].toBigInt(),
                    });
                }

                // log logs to make sure it keeps progressing
                // TEMPLATE: Adapt log line
                if (count % (10 * limit) === 0) {
                    log(`Retrieved ${count} proxies`);
                }
            }

            // TEMPLATE: Adapt proxies
            log(`Retrieved ${count} total proxies`);
        }, 30_000);

        it({
            id: "C100",
            title: "should have no more than the maximum allowed proxies",
            timeout: 240000,
            test: async () => {
                const maxProxies = paraApi.consts.proxy.maxProxies.toNumber();
                const failedProxies: { accountId: string; proxiesCount: number }[] = [];

                for (const accountId of proxiesPerAccount.keys()) {
                    const proxiesCount = proxiesPerAccount.get(accountId).definitions.length;
                    if (proxiesCount > maxProxies) {
                        failedProxies.push({ accountId, proxiesCount });
                    }
                }

                if (failedProxies.length > 0) {
                    log("Failed accounts with too many proxies:");
                    log(
                        failedProxies
                            .map(({ accountId, proxiesCount }) => {
                                return `accountId: ${accountId} - ${chalk.red(
                                    proxiesCount.toString().padStart(4, " ")
                                )} proxies (expected max: ${maxProxies})`;
                            })
                            .join("\n")
                    );
                }

                expect(failedProxies.length, "Failed max proxies").to.equal(0);

                log(`Verified ${proxiesPerAccount.size} total accounts (at #${atBlockNumber})`);
            },
        });

        it({
            id: "C200",
            title: "should have a maximum allowed proxies of 32",
            test: async () => {
                const runtimeName = paraApi.runtimeVersion.specName.toString();
                const maxProxies = (await paraApi.consts.proxy.maxProxies).toNumber();

                switch (runtimeName) {
                    case "dancebox":
                        expect(maxProxies).to.equal(32);
                        break;
                    case "flashbox":
                        expect(maxProxies).to.equal(32);
                        break;
                    case "dancelight":
                        expect(maxProxies).to.equal(32);
                        break;
                }

                log("Verified maximum allowed proxies constant");
            },
        });

        it({
            id: "C300",
            title: "check that the account has at least the deposit needed to cover proxy",
            test: async () => {
                for (const accountId of proxiesPerAccount.keys()) {
                    const accountData = await apiAt.query.system.account(accountId);
                    const reserved = accountData.data.reserved.toBigInt();

                    if (reserved === 0n && accountData.nonce.toBigInt() === 0n) {
                        log(
                            `Account ${accountId} has no reserved balance and no nonce, looks like this is a pure proxy. Skipping it. `
                        );
                        continue;
                    }

                    expect(
                        reserved,
                        `Reserved balance: ${reserved} for account ${accountId} should be more or equal to deposit: ${proxiesPerAccount.get(accountId).deposit}`
                    ).toBeGreaterThanOrEqual(proxiesPerAccount.get(accountId).deposit);
                }
            },
        });
    },
});
