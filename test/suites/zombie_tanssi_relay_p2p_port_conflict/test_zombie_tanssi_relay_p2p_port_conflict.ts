// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import fs from "node:fs";

/**
 * Find the hex key corresponding to a given SS58 account.
 *
 * @param keys    – object mapping chains to hex keys
 * @param assign  – object mapping chains to SS58 accounts
 * @param account – the SS58 account you’re looking up
 * @returns the matching hex key, or undefined if not found
 */
function findHexKeyForAccount(assign: any, keys: any, account: string): string | undefined {
    // 1) check orchestratorChain
    const orchIndex = assign.orchestratorChain.indexOf(account);
    if (orchIndex !== -1) {
        return keys.orchestratorChain[orchIndex];
    }

    // 2) check each containerChains group
    for (const chainId of Object.keys(assign.containerChains)) {
        const accounts = assign.containerChains[chainId];
        const idx = accounts.indexOf(account);
        if (idx !== -1) {
            // guard: same chainId must exist in keys
            const keyList = keys.containerChains[chainId];
            if (!keyList) {
                throw new Error(`No keys found for chain ${chainId} (found assignment but missing keys)`);
            }
            return keyList[idx];
        }
    }

    // 3) not found anywhere
    return undefined;
}

async function countFilesInKeystore(path: string): Promise<number> {
    // Check that the directory exists and is accessible
    await fs.promises.access(path, fs.constants.F_OK);

    // Read all filenames in the directory
    const filenames: string[] = await fs.promises.readdir(path);

    // Assert that there is at least one file
    if (filenames.length === 0) {
        throw new Error(`Expected at least one file in ${path}, but found none.`);
    }

    return filenames.length;
}

describeSuite({
    id: "ZOMBIETANSS01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
        });

        it({
            id: "T01",
            title: "Test block numbers in relay are 0 yet",
            test: async () => {
                const peers = (await relayApi.rpc.system.peers()).toJSON();
                console.log(peers);
                expect(peers.length, "validators cannot connect to each other").toBeGreaterThan(0);
            },
        });
    },
});
