import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { BALTATHAR_ADDRESS, BALTATHAR_PRIVATE_KEY, CHARLETH_ADDRESS, type KeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import { WebSocketProvider, ethers, parseUnits } from "ethers";
import { getHeaderFromRelay, getTmpZombiePath, signAndSendAndInclude, waitForLogs } from "utils";

// Checks every second the log file to find the watcher best block notification until it is found or
// timeout is reached. If timeout is reached, throws an error.
export async function expectLogs(logFilePath: string, timeout: number, logs: string[]): Promise<void> {
    const logsFound = await waitForLogs(logFilePath, timeout, logs);
    if (!logsFound) {
        expect.fail(`RPC Assignment Watch log was not found after ${timeout} seconds.`);
    }
}

describeSuite({
    id: "ZOM01",
    title: "Data Preservers Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;

        let dataProvider2000Api: ApiPromise;
        let dataProvider2001Api: ApiPromise;
        let dataProvider2000BApi: ApiPromise;

        let keyring: Keyring;
        let alice: KeyringPair;
        let bob: KeyringPair;

        let profile1: number;
        let profile2: number;

        let balanceBeforeAssignment: bigint;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");

            keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            bob = keyring.addFromUri("//Bob", { name: "Bob default" });

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

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);

            expect(header2000.number.toNumber()).to.be.equal(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on relay",
            test: async () => {
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Data preservers 2000 watcher properly starts",
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2000.log`;
                await expectLogs(logFilePath, 300, ["Starting Data Preserver Assignment Watcher"]);
            },
        });

        it({
            id: "T03",
            title: "Change assignment 2000",
            timeout: 180000,
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2000.log`;

                const profile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: false } },
                    assignmentRequest: "Free",
                };

                profile1 = Number(await relayApi.query.dataPreservers.nextProfileId());
                expect(profile1).to.be.eq(2); // 0 and 1 are auto assigned for bootnodes

                {
                    const tx = relayApi.tx.dataPreservers.forceCreateProfile(profile, bob.address);
                    await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Relay");

                    const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                }

                {
                    const tx = relayApi.tx.dataPreservers.forceStartAssignment(profile1, 2000, "Free");
                    await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Relay");
                }

                const onChainProfile = (await relayApi.query.dataPreservers.profiles(profile1)).unwrap();
                const onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                const bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman().toString()).to.be.eq(["2,000", "Free"].toString());

                await expectLogs(logFilePath, 300, ["NotAssigned => Active(Id(2000))"]);
            },
        });

        it({
            id: "T04",
            title: "RPC endpoint 2000 is properly started",
            test: async () => {
                const wsProvider = new WsProvider("ws://127.0.0.1:9950");
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
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2001.log`;
                await expectLogs(logFilePath, 300, ["Starting Data Preserver Assignment Watcher"]);
            },
        });

        it({
            id: "T06",
            title: "Change assignment 2001",
            timeout: 180000,
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2001.log`;

                const profile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: true } },
                    assignmentRequest: "Free",
                };

                profile2 = Number(await relayApi.query.dataPreservers.nextProfileId());
                expect(profile2).to.be.eq(3);

                {
                    const tx = relayApi.tx.dataPreservers.forceCreateProfile(profile, bob.address);
                    await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Relay");
                }

                {
                    const tx = relayApi.tx.dataPreservers.forceStartAssignment(profile2, 2001, "Free");
                    await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Relay");
                }

                const onChainProfile = (await relayApi.query.dataPreservers.profiles(profile2)).unwrap();
                const onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                const bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman().toString()).to.be.eq(["2,001", "Free"].toString());

                await expectLogs(logFilePath, 300, ["NotAssigned => Active(Id(2001))"]);
            },
        });

        it({
            id: "T07",
            title: "RPC endpoint 2001 is properly started",
            test: async () => {
                const wsProvider = new WsProvider("ws://127.0.0.1:9952");
                dataProvider2001Api = await ApiPromise.create({ provider: wsProvider });

                const container2001Network = dataProvider2001Api.consts.system.version.specName.toString();
                const paraId2001 = (await dataProvider2001Api.query.parachainInfo.parachainId()).toString();
                expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
                expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");
            },
        });

        it({
            id: "T07b",
            title: "RPC endpoint 2001 is synced to latest block",
            test: async () => {
                const wsProvider = new WsProvider("ws://127.0.0.1:9952");
                dataProvider2001Api = await ApiPromise.create({ provider: wsProvider });

                while (true) {
                    const blockNum = (await dataProvider2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
                    if (blockNum > 0) {
                        break;
                    }
                    // TODO: we want to wait for 1 container block, not 1 tanssi block, but this also works
                    await context.waitBlock(1, "Relay");
                }
            },
        });

        it({
            id: "T08",
            title: "RPC endpoint 2001 is Ethereum compatible",
            test: async () => {
                const url = "ws://127.0.0.1:9952";
                const customHttpProvider = new WebSocketProvider(url);
                console.log((await customHttpProvider.getNetwork()).chainId);

                const signer = new ethers.Wallet(BALTATHAR_PRIVATE_KEY, customHttpProvider);

                // Try to send a test transaction.
                const nonce = await customHttpProvider.getTransactionCount(BALTATHAR_ADDRESS);
                const tx = await signer.sendTransaction({
                    to: CHARLETH_ADDRESS,
                    value: parseUnits("0.001", "ether"),
                    nonce,
                });
                let blockNumber = await customHttpProvider.getBlockNumber();
                console.log("frontier template block number sent: ", blockNumber);
                console.log("frontier tx: ", tx);
                const now = new Date();
                const pad = (n) => String(n).padStart(2, "0");
                console.log(
                    `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())} ` +
                        `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`
                );
                await customHttpProvider.waitForTransaction(tx.hash, 1, 300_000);
                blockNumber = await customHttpProvider.getBlockNumber();
                console.log("frontier template block number included: ", blockNumber);
                await customHttpProvider.waitForTransaction(tx.hash, 1, 300_000);
                expect(Number(await customHttpProvider.getBalance(CHARLETH_ADDRESS))).to.be.greaterThan(0);
            },
        });

        it({
            id: "T09",
            title: "Stop assignement 2001",
            timeout: 180000,
            test: async () => {
                {
                    const tx = relayApi.tx.dataPreservers.stopAssignment(profile2, 2001);
                    await signAndSendAndInclude(tx, bob);
                    await context.waitBlock(1, "Relay");
                }

                const onChainProfile = (await relayApi.query.dataPreservers.profiles(profile2)).unwrap();
                const onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                const bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman()).to.be.eq(null);

                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2001.log`;
                await expectLogs(logFilePath, 300, ["Active(Id(2001)) => NotAssigned"]);
            },
        });

        it({
            id: "T10",
            title: "Update profile to Stream Payment",
            timeout: 180000,
            test: async () => {
                const newProfile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: true } },
                    assignmentRequest: {
                        StreamPayment: {
                            config: {
                                timeUnit: "BlockNumber",
                                assetId: "Native",
                                rate: "1000000",
                            },
                        },
                    },
                };

                {
                    const tx = relayApi.tx.dataPreservers.updateProfile(profile2, newProfile);
                    await signAndSendAndInclude(tx, bob);
                    await context.waitBlock(1, "Relay");
                }

                const onChainProfile = (await relayApi.query.dataPreservers.profiles(profile2)).unwrap();
                const onChainProfileAccount = u8aToHex(decodeAddress(onChainProfile.account.toString()));
                const bobAccount = u8aToHex(bob.addressRaw);

                expect(onChainProfileAccount).to.be.eq(bobAccount);
                expect(onChainProfile.assignment.toHuman()).to.be.eq(null);
                expect(JSON.stringify(onChainProfile.profile.assignmentRequest.toHuman())).to.be.eq(
                    JSON.stringify({
                        StreamPayment: {
                            config: {
                                timeUnit: "BlockNumber",
                                assetId: "Native",
                                rate: "1,000,000",
                                minimumRequestDeadlineDelay: "0",
                                softMinimumDeposit: "0",
                            },
                        },
                    })
                );
            },
        });

        it({
            id: "T11",
            title: "Start new assignment for chain 2000 with stream payment",
            timeout: 240000,
            test: async () => {
                {
                    // to non-force assign we need to have a para manager, which is not the case
                    // with paras registered in genesis. we thus set the para manager manually here
                    const tx = relayApi.tx.containerRegistrar.setParaManager(2000, alice.address);
                    await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Relay");
                }

                balanceBeforeAssignment = (await relayApi.query.system.account(bob.address)).data.free.toBigInt();
                console.log(`balanceBeforeAssignment: ${balanceBeforeAssignment}`);

                {
                    // pays for 10 blocks of service
                    const tx = relayApi.tx.dataPreservers.startAssignment(profile2, 2000, {
                        StreamPayment: { initialDeposit: 10000000 },
                    });
                    await signAndSendAndInclude(tx, alice);
                    await context.waitBlock(1, "Relay");
                }

                const onChainProfile = (await relayApi.query.dataPreservers.profiles(profile2)).unwrap();
                expect(JSON.stringify(onChainProfile.assignment.toHuman())).to.be.eq(
                    JSON.stringify(["2,000", { StreamPayment: { streamId: "0" } }])
                );

                const streamPayment = (await relayApi.query.streamPayment.streams(0)).unwrap();
                expect(JSON.stringify(streamPayment.source.toHuman())).to.eq(JSON.stringify(alice.address));
                expect(JSON.stringify(streamPayment.target.toHuman())).to.eq(JSON.stringify(bob.address));
                expect(JSON.stringify(streamPayment.config)).to.eq(
                    JSON.stringify({
                        timeUnit: "BlockNumber",
                        assetId: "Native",
                        rate: 1000000,
                        minimumRequestDeadlineDelay: 0,
                        softMinimumDeposit: 0,
                    })
                );

                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2001.log`;
                await expectLogs(logFilePath, 300, ["NotAssigned => Active(Id(2000))"]);

                const wsProvider = new WsProvider("ws://127.0.0.1:9952");
                dataProvider2000BApi = await ApiPromise.create({ provider: wsProvider });

                const newContainerNetwork = dataProvider2000BApi.consts.system.version.specName.toString();
                const newParaId = (await dataProvider2000BApi.query.parachainInfo.parachainId()).toString();
                expect(newContainerNetwork, "Container2000 API incorrect").to.contain("container-chain-template");
                expect(newParaId, "Container2000 API incorrect").to.be.equal("2000");
            },
        });

        it({
            id: "T11b",
            title: "Start new assignment for chain 2000 with stream payment - wait 10 blocks",
            timeout: 180000,
            test: async () => {
                await context.waitBlock(10, "Relay");
            },
        });

        it({
            id: "T12",
            title: "Data preserver services halt after stream payment is stalled",
            timeout: 180000,
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/DataPreserver-2001.log`;
                await expectLogs(logFilePath, 300, ["Active(Id(2000)) => Inactive(Id(2000))"]);

                {
                    // pays for 10 blocks of service
                    const tx = relayApi.tx.streamPayment.performPayment(0);
                    await signAndSendAndInclude(tx, alice);
                    await context.waitBlock(1, "Relay");
                }

                const balanceAfter = (await relayApi.query.system.account(bob.address)).data.free.toBigInt();
                console.log(`balanceAfter: ${balanceAfter}`);
                expect(balanceAfter).to.be.eq(balanceBeforeAssignment + BigInt(10000000));

                await context.waitBlock(1, "Relay");
            },
        });
    },
});
