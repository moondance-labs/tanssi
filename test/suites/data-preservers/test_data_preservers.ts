import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { signAndSendAndInclude } from "../../util/block";
import { getHeaderFromRelay } from "../../util/relayInterface";
import fs from "fs/promises";
import ethers from "ethers";
import { baltathar, BALTATHAR_PRIVATE_KEY, CHARLETH_ADDRESS, KeyringPair } from "@moonwall/util";
import { BN, u8aToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";

describeSuite({
    id: "DP01",
    title: "Data Preservers Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;

        let dataProvider2000Api: ApiPromise;
        let dataProvider2001Api: ApiPromise;
        let dataProvider2000BApi: ApiPromise;

        let keyring: Keyring;
        let alice: KeyringPair;
        let bob: KeyringPair;

        let profile1;
        let profile2;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");

            keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            bob = keyring.addFromUri("//Bob", { name: "Bob default" });

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

            const paraNetwork = paraApi.consts.system.version.specName.toString();
            const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
            expect(paraNetwork, "Para API incorrect").to.contain("dancebox");
            expect(paraId1000, "Para API incorrect").to.be.equal("1000");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
            expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);

            expect(header2000.number.toNumber()).to.be.equal(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async function () {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Data preservers 2000 watcher properly starts",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver-2000.log";
                await waitForLogs(logFilePath, 300, ["Starting data preserver assignment watcher"]);
            },
        });

        it({
            id: "T03",
            title: "Change assignment 2000",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver-2000.log";
                
                const profile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: false } },
                    assignmentRequest: "Free",
                };

                profile1 = Number(await paraApi.query.dataPreservers.nextProfileId());
                expect(profile1).to.be.eq(2); // 0 and 1 are auto assigned for bootnodes

                {
                    const tx = paraApi.tx.dataPreservers.forceCreateProfile(profile, bob.address);
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                {
                    const tx = paraApi.tx.dataPreservers.forceStartAssignment(profile1, 2000, "Free");
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                let onChainProfile = (await paraApi.query.dataPreservers.profiles(profile1)).unwrap();
                let onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                let bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman().toString()).to.be.eq(["2,000", "Free"].toString());

                await waitForLogs(logFilePath, 300, ["NotAssigned => Active(Id(2000))"]);
            },
        });

        it({
            id: "T04",
            title: "RPC endpoint 2000 is properly started",
            test: async function () {
                const wsProvider = new WsProvider('ws://127.0.0.1:9950');
                dataProvider2000Api = await ApiPromise.create({ provider: wsProvider });

                const container2000Network = dataProvider2000Api.consts.system.version.specName.toString();
                const paraId2000 = (await dataProvider2000Api.query.parachainInfo.parachainId()).toString();
                expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
                expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");
            },
        });

        it({
            id: "T05",
            title: "Data preservers 2001 watcher properly starts",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver-2001.log";
                await waitForLogs(logFilePath, 300, ["Starting data preserver assignment watcher"]);
            },
        });

        it({
            id: "T06",
            title: "Change assignment 2001",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver-2001.log";

                const profile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: true } },
                    assignmentRequest: "Free",
                };

                profile2 = Number(await paraApi.query.dataPreservers.nextProfileId());
                expect(profile2).to.be.eq(3);

                {
                    const tx = paraApi.tx.dataPreservers.forceCreateProfile(profile, bob.address);
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                {
                    const tx = paraApi.tx.dataPreservers.forceStartAssignment(profile2, 2001, "Free");
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                let onChainProfile = (await paraApi.query.dataPreservers.profiles(profile2)).unwrap();
                let onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                let bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman().toString()).to.be.eq(["2,001", "Free"].toString());

                await waitForLogs(logFilePath, 300, ["NotAssigned => Active(Id(2001))"]);
            },
        });

        it({
            id: "T07",
            title: "RPC endpoint 2001 is properly started",
            test: async function () {
                const wsProvider = new WsProvider('ws://127.0.0.1:9952');
                dataProvider2001Api = await ApiPromise.create({ provider: wsProvider });

                const container2001Network = dataProvider2001Api.consts.system.version.specName.toString();
                const paraId2001 = (await dataProvider2001Api.query.parachainInfo.parachainId()).toString();
                expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
                expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");
            },
        });

        it({
            id: "T08",
            title: "RPC endpoint 2001 is Ethereum compatible",
            test: async function () {
                const url = "ws://127.0.0.1:9952";
                const customHttpProvider = new ethers.providers.WebSocketProvider(url);
                console.log((await customHttpProvider.getNetwork()).chainId);

                const signer = new ethers.Wallet(BALTATHAR_PRIVATE_KEY, customHttpProvider);
                const tx = await signer.sendTransaction({
                    to: CHARLETH_ADDRESS,
                    value: ethers.utils.parseUnits("0.001", "ether"),
                });

                await customHttpProvider.waitForTransaction(tx.hash);
                expect(Number(await customHttpProvider.getBalance(CHARLETH_ADDRESS))).to.be.greaterThan(0);
            },
        });

        it({
            id: "T09",
            title: "Stop assignement 2001",
            test: async function () {
                {
                    const tx = paraApi.tx.dataPreservers.stopAssignment(profile2, 2001);
                    await signAndSendAndInclude(tx, bob);
                    await context.waitBlock(1, "Tanssi");
                }

                let onChainProfile = (await paraApi.query.dataPreservers.profiles(profile2)).unwrap();
                let onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                let bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman()).to.be.eq(null);

                const logFilePath = getTmpZombiePath() + "/DataPreserver-2001.log";
                await waitForLogs(logFilePath, 300, ["Active(Id(2001)) => NotAssigned"]);
            },
        });

        it({
            id: "T10",
            title: "Update profile to Stream Payment",
            test: async function () {
                const newProfile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: true } },
                    assignmentRequest: { "StreamPayment": {
                        "config": {
                            timeUnit: "BlockNumber",
                            assetId: "Native",
                            rate: '1000000',
                        } 
                    }},
                };

                {
                    const tx = paraApi.tx.dataPreservers.updateProfile(profile2, newProfile);
                    console.log(`tx: ${tx.toHuman().toString()}`);
                    await signAndSendAndInclude(tx, bob);
                    await context.waitBlock(1, "Tanssi");
                }

                let onChainProfile = (await paraApi.query.dataPreservers.profiles(profile2)).unwrap();
                let onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                let bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman()).to.be.eq(null);
                expect(JSON.stringify(onChainProfile.profile.assignmentRequest.toHuman())).to.be.eq(JSON.stringify({ "StreamPayment": {
                    "config": {
                        timeUnit: "BlockNumber",
                        assetId: "Native",
                        rate: '1,000,000',
                    } 
                }}));
            },
        });
    },
});

// Checks every second the log file to find the watcher best block notification until it is found or
// timeout is reached.
async function waitForLogs(logFilePath: string, timeout: number, logs: string[]): Promise<void> {
    for (let i = 0; i < timeout; i++) {
        if (checkLogsNoFail(logFilePath, logs)) {
            return;
        }

        await delay(1000);
    }

    expect.fail(`RPC Assignment Watch log was not found after ${timeout} seconds.`);
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
async function checkLogsNoFail(logFilePath: string, logs: string[]): Promise<boolean> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    let logIndex = 0;

    for (let i = 0; i < lines.length; i++) {
        if (logIndex < logs.length && lines[i].includes(logs[logIndex])) {
            logIndex++;
        }

        if (logIndex === logs.length) {
            break;
        }
    }

    return logIndex === logs.length;
}

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

const delay = (ms) => new Promise((res) => setTimeout(res, ms));
