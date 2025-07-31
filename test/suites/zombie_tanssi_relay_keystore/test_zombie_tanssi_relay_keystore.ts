import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import {
    checkLogsNotExist,
    getCommonTests,
    getKeyringNimbusIdHex,
    getTmpZombiePath,
    signAndSendAndInclude,
    waitSessions,
} from "utils";
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

// probably should be moved to utils
export async function countFilesInKeystore(path: string): Promise<number> {
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
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let collator01RelayApi: ApiPromise;
        let collator02RelayApi: ApiPromise;
        let newKeys1: Bytes;
        let newKeys2: Bytes;
        let oldAssignment: any;
        let oldKeys: any;
        let collator01KeystorePath: string;
        let collator02KeystorePath: string;
        let collator01KeystoreLength: number;
        let collator02KeystoreLength: number;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            container2002Api = context.polkadotJs("Container2002");
            ethersSigner = context.ethers();

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
            expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

            const container2002Network = container2002Api.consts.system.version.specName.toString();
            const paraId2002 = (await container2002Api.query.parachainInfo.parachainId()).toString();
            expect(container2002Network, "Container2002 API incorrect").to.contain("container-chain-template");
            expect(paraId2002, "Container2002 API incorrect").to.be.equal("2002");

            const wsProvider1 = new WsProvider("ws://127.0.0.1:9961");
            collator01RelayApi = await ApiPromise.create({ provider: wsProvider1 });
            const wsProvider2 = new WsProvider("ws://127.0.0.1:9962");
            collator02RelayApi = await ApiPromise.create({ provider: wsProvider2 });

            collator01KeystorePath = `${getTmpZombiePath()}/Collator-01/relay-data/chains/dancelight_local_testnet/keystore/`;
            // Collator-02 keystore is in a different path because we have added a custom `--keystore-path` arg
            collator02KeystorePath = `${getTmpZombiePath()}/Collator-02/relay-data/tmp_keystore_zombie_test/`;
        }, 120000);

        const commonTests = getCommonTests(
            context,
            relayApi,
            [2000, 2001, 2002],
            [container2000Api, container2001Api, container2002Api]
        );

        for (const test of commonTests) {
            it(test);
        }

        it({
            id: "T11",
            title: "Transactions can be made with ethers",
            timeout: 30000,
            test: async () => {
                const randomAccount = generateKeyringPair();
                const tx = await context.ethers().sendTransaction({
                    to: randomAccount.address,
                    value: 1_000_000_000_000n,
                });
                await tx.wait();
                expect(await context.ethers().provider.getBalance(randomAccount.address)).to.be.greaterThan(0);
            },
        });

        it({
            id: "T12",
            title: "Test rotateKeys for Collator-01 and Collator02",
            timeout: 60000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const collator01 = keyring.addFromUri("//Collator-01", { name: "Collator-01 default" });
                const collator02 = keyring.addFromUri("//Collator-02", { name: "Collator-02 default" });

                const session = await relayApi.query.session.currentIndex();
                oldKeys = (await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(session)).toJSON();
                oldAssignment = (await relayApi.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();

                console.log("session", session.toJSON());
                console.log("oldKeys", oldKeys);
                console.log("oldAssignment", oldAssignment);

                newKeys1 = await collator01RelayApi.rpc.author.rotateKeys();
                newKeys2 = await collator02RelayApi.rpc.author.rotateKeys();

                // New keys are added to the same keystore, so the total number of keys increases
                const newCollator01KeystoreLength = await countFilesInKeystore(collator01KeystorePath);
                expect(newCollator01KeystoreLength).toBeGreaterThan(collator01KeystoreLength);
                const newCollator02KeystoreLength = await countFilesInKeystore(collator02KeystorePath);
                expect(newCollator02KeystoreLength).toBeGreaterThan(collator02KeystoreLength);

                await Promise.all([
                    signAndSendAndInclude(relayApi.tx.session.setKeys(newKeys1, []), collator01),
                    signAndSendAndInclude(relayApi.tx.session.setKeys(newKeys2, []), collator02),
                ]);
            },
        });

        it({
            id: "T13",
            title: "Test rotateKeys - Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T14",
            title: "Check that keys have changed and collators keep producing blocks",
            timeout: 60000,
            test: async () => {
                const session = await relayApi.query.session.currentIndex();
                const newKeys = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(session)
                ).toJSON();
                const newAssignment = (await relayApi.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();

                console.log("session", session.toJSON());
                console.log("newKeys", newKeys);
                console.log("newAssignment", newAssignment);

                // Since we disabled rotation, collator assignment did not change
                expect(newAssignment).to.deep.equal(oldAssignment);
                // But we changed collator keys, so some of them witll not be equal
                expect(newKeys).to.not.deep.equal(oldKeys);

                // Compare on chain key with response of rpc.rotateKeys
                const hexAddress1 = getKeyringNimbusIdHex("Collator-01");
                const collatorName1 = relayApi.createType("AccountId", hexAddress1).toString();
                const key1 = findHexKeyForAccount(newAssignment, newKeys, collatorName1);
                const decodedKeys1 = relayApi.createType("DancelightRuntimeSessionKeys", newKeys1);
                expect(key1).to.equal(decodedKeys1.nimbus.toJSON());

                const hexAddress2 = getKeyringNimbusIdHex("Collator-02");
                const collatorName2 = relayApi.createType("AccountId", hexAddress2).toString();
                const key2 = findHexKeyForAccount(newAssignment, newKeys, collatorName2);
                const decodedKeys2 = relayApi.createType("DancelightRuntimeSessionKeys", newKeys2);
                expect(key2).to.equal(decodedKeys2.nimbus.toJSON());
            },
        });

        it({
            id: "T15",
            title: "Blocks are being produced on container 2000",
            test: async () => {
                await context.waitBlock(3, "Container2000");
            },
        });

        it({
            id: "T16",
            title: "Blocks are being produced on container 2001",
            test: async () => {
                await context.waitBlock(3, "Container2001");
            },
        });

        it({
            id: "T20",
            title: "Check collator logs to ensure common errors are fixed",
            timeout: 300000,
            test: async () => {
                const logs = ["/Collator-01.log", "/Collator-02.log", "/Collator-03.log", "/Collator-04.log"];
                for (const log of logs) {
                    const logFilePath = getTmpZombiePath() + log;
                    await checkLogsNotExist(logFilePath, [
                        "Shutdown error",
                        "Timeout when waiting for paritydb lock",
                        "Error waiting for chain",
                        "Failed to start container chain",
                        "Shutting down container chain service",
                        "Entering off-chain worker.",
                        "Overweight para inherent data after enacting the candidates",
                    ]);
                }
            },
        });

        it({
            id: "T21",
            title: "Check reward points for validators are distributed",
            test: async () => {
                const keys = await relayApi.query.externalValidatorsRewards.rewardPointsForEra.keys();
                expect(keys.length).to.be.greaterThan(0);
            },
        });
    },
});
