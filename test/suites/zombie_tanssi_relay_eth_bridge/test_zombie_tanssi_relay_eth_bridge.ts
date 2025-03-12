import "@tanssi/api-augment/dancelight";

import { afterAll, beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { MultiLocation } from "@polkadot/types/interfaces/xcm/types";
import { u8aToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import { ethers } from "ethers";
import { type ChildProcessWithoutNullStreams, exec, spawn } from "node:child_process";
import { signAndSendAndInclude, sleep, waitSessions } from "utils";

// Change this if we change the storage parameter in runtime
const GATEWAY_STORAGE_KEY = "0xaed97c7854d601808b98ae43079dafb3";

function execCommand(command: string, options?) {
    return new Promise((resolve, reject) => {
        exec(command, options, (error: unknown, stdout: string, stderr: string) => {
            if (error) {
                reject({ error, stdout, stderr });
            } else {
                resolve({ stdout, stderr });
            }
        });
    });
}

async function calculateNumberOfBlocksTillNextEra(api, blocksPerSession) {
    // Wait till the second block of next era
    const sessionsPerEra = await api.consts.externalValidators.sessionsPerEra;

    // Check when next era will be enacted
    // 1. Get current era info
    const activeEraInfo = (await api.query.externalValidators.activeEra()).toJSON();
    // 2. Get next era start block
    const nextEraStartBlock = (activeEraInfo.index + 1) * sessionsPerEra * blocksPerSession + 1;
    // 3. Get current block
    const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
    // 4. calculate how much to wait
    return nextEraStartBlock - currentBlock;
}

describeSuite({
    id: "ZOMBIETANSSI01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let relayCharlieApi: ApiPromise;
        let ethereumNodeChildProcess: ChildProcessWithoutNullStreams;
        let relayerChildProcess: ChildProcessWithoutNullStreams;
        let alice: KeyringPair;
        let beefyClientDetails: any;

        const ethUrl = "ws://127.0.0.1:8546";
        let customHttpProvider: ethers.WebSocketProvider;
        let ethereumWallet: ethers.Wallet;
        let middlewareContract: ethers.Contract;
        let gatewayContract: ethers.Contract;
        let gatewayProxyAddress: string;
        let middlewareAddress: string;
        let middlewareDetails: any;

        let operatorAccount: KeyringPair;
        let operatorNimbusKey: string;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            relayCharlieApi = context.polkadotJs("Tanssi-charlie");

            // //BeaconRelay
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const beaconRelay = keyring.addFromUri("//BeaconRelay", { name: "Beacon relay default" });
            const executionRelay = keyring.addFromUri("//ExecutionRelay", { name: "Execution relay default" });

            // Operator keys
            operatorAccount = keyring.addFromUri("//Charlie", { name: "Charlie default" });
            // We rotate the keys for charlie so that we have access to them from this test as well as the node
            operatorNimbusKey = (await relayCharlieApi.rpc.author.rotateKeys()).toHex();
            await relayApi.tx.session.setKeys(operatorNimbusKey, []).signAndSend(operatorAccount);

            const fundingTxHash = await signAndSendAndInclude(
                relayApi.tx.utility.batch([
                    relayApi.tx.balances.transferAllowDeath(beaconRelay.address, 1_000_000_000_000),
                    relayApi.tx.balances.transferAllowDeath(executionRelay.address, 1_000_000_000_000),
                ]),
                alice
            );
            console.log("Transferred money to relayers", fundingTxHash.txHash.toHex());

            ethereumNodeChildProcess = spawn("./scripts/bridge/start-ethereum-node.sh", {
                shell: true,
                detached: true,
            });
            ethereumNodeChildProcess.stderr.setEncoding("utf-8");
            ethereumNodeChildProcess.stderr.on("data", (chunk) => console.log(chunk));

            await execCommand("./scripts/bridge/generate-beefy-checkpoint.sh", {
                env: {
                    RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                    ...process.env,
                },
            });

            // Waiting till ethreum node produces one block
            console.log("Waiting some time for ethereum node to produce block, before we deploy contract");
            await sleep(20000);

            await execCommand("./scripts/bridge/deploy-ethereum-contracts.sh", {
                env: {
                    OPERATOR1_KEY: u8aToHex(operatorAccount.addressRaw),
                    ...process.env,
                },
            });

            console.log("Contracts deployed");

            const ethInfo = JSON.parse(<string>(await execCommand("./scripts/bridge/generate-eth-info.sh")).stdout);

            console.log("BeefyClient contract address is:", ethInfo.snowbridge_info.contracts.BeefyClient.address);
            beefyClientDetails = ethInfo.snowbridge_info.contracts.BeefyClient;

            console.log("Gateway contract proxy address is:", ethInfo.snowbridge_info.contracts.GatewayProxy.address);
            gatewayProxyAddress = ethInfo.snowbridge_info.contracts.GatewayProxy.address;

            console.log("Symbiotic middleware address is: ", ethInfo.symbiotic_info.contracts.ERC1967Proxy.address);
            middlewareDetails = ethInfo.symbiotic_info.contracts.Middleware;
            middlewareAddress = ethInfo.symbiotic_info.contracts.ERC1967Proxy.address;

            console.log("Setting gateway address to proxy contract:", gatewayProxyAddress);
            const setGatewayAddressTxHash = await signAndSendAndInclude(
                relayApi.tx.sudo.sudo(relayApi.tx.system.setStorage([[GATEWAY_STORAGE_KEY, gatewayProxyAddress]])),
                alice
            );
            console.log("Set gateway address transaction hash:", setGatewayAddressTxHash.txHash.toHex());

            customHttpProvider = new ethers.WebSocketProvider(ethUrl);
            ethereumWallet = new ethers.Wallet(ethInfo.ethereum_key, customHttpProvider);

            // Setting up Middleware
            middlewareContract = new ethers.Contract(middlewareAddress, middlewareDetails.abi, ethereumWallet);
            const tx = await middlewareContract.setGateway(gatewayProxyAddress);
            await tx.wait();

            gatewayContract = new ethers.Contract(
                gatewayProxyAddress,
                ethInfo.snowbridge_info.contracts.Gateway.abi,
                ethereumWallet
            );
            const setMiddlewareTx = await gatewayContract.setMiddleware(middlewareAddress);
            await setMiddlewareTx.wait();

            const initialBeaconUpdate = JSON.parse(<string>(
                    await execCommand("./scripts/bridge/setup-relayer.sh", {
                        env: {
                            RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                            ...process.env,
                        },
                    })
                ).stdout);

            const tokenLocation = relayApi.createType<MultiLocation>("MultiLocation", {
                parents: 0,
                interior: "Here",
            });
            const versionedLocation = {
                V3: tokenLocation,
            };

            const metadata = {
                name: "dance",
                symbol: "dance",
                decimals: 18,
            };

            // We need to read initial checkpoint data and address of gateway contract to setup the ethereum client
            // We also need to register token
            // Once that is done, we can start the relayer
            await signAndSendAndInclude(
                relayApi.tx.sudo.sudo(
                    relayApi.tx.utility.batch([
                        relayApi.tx.ethereumBeaconClient.forceCheckpoint(initialBeaconUpdate),
                        relayApi.tx.ethereumSystem.registerToken(versionedLocation, metadata),
                    ])
                ),
                alice
            );

            relayerChildProcess = spawn("./scripts/bridge/start-relayer.sh", {
                shell: true,
                detached: true,
                env: {
                    RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                    ...process.env,
                },
            });
            relayerChildProcess.stderr.setEncoding("utf-8");
            relayerChildProcess.stderr.on("data", (chunk) => console.log(chunk));
        }, 12000000);

        it({
            id: "T01",
            title: "Ethereum Blocks are being recognized on tanssi-relay",
            test: async () => {
                await waitSessions(context, relayApi, 1, null, "Tanssi-relay");
                const firstFinalizedBlockRoot = (
                    await relayApi.query.ethereumBeaconClient.latestFinalizedBlockRoot()
                ).toJSON();
                expect(firstFinalizedBlockRoot).to.not.equal(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
                const secondFinalizedBlockRoot = (
                    await relayApi.query.ethereumBeaconClient.latestFinalizedBlockRoot()
                ).toJSON();
                expect(secondFinalizedBlockRoot).to.not.equal(firstFinalizedBlockRoot);
            },
        });

        it({
            id: "T02",
            title: "Dancelight Blocks are being recognized on ethereum",
            test: async () => {
                const beefyContract = new ethers.Contract(
                    beefyClientDetails.address,
                    beefyClientDetails.abi,
                    customHttpProvider
                );
                const currentBeefyBlock = Number(await beefyContract.latestBeefyBlock());
                expect(currentBeefyBlock).to.greaterThan(0);
                await waitSessions(context, relayApi, 3, null, "Tanssi-relay");
                const nextBeefyBlock = Number(await beefyContract.latestBeefyBlock());
                expect(nextBeefyBlock).to.greaterThan(currentBeefyBlock);
            },
        });

        it({
            id: "T03",
            title: "Message can be passed from ethereum to Starlight",
            test: async () => {
                const externalValidatorsBefore = await relayApi.query.externalValidators.externalValidators();

                const epoch = await middlewareContract.getCurrentEpoch();
                const currentOperatorsKeys = await middlewareContract.sortOperatorsByVaults(epoch);

                console.log("Middleware: Epoch is:", epoch);
                console.log("Middleware: Operator keys are:", currentOperatorsKeys);
                console.log("Starlight: External validators are:", externalValidatorsBefore.toJSON());

                try {
                    const tx = await middlewareContract.sendCurrentOperatorsKeys();
                    await tx.wait();
                } catch (error) {
                    throw new Error(`Failed to send operator data: ${error.message}`, error.code);
                }

                // wait some time for the data to be relayed
                await waitSessions(
                    context,
                    relayApi,
                    6,
                    async () => {
                        try {
                            const externalValidators = await relayApi.query.externalValidators.externalValidators();
                            expect(externalValidators).to.not.deep.eq(externalValidatorsBefore);
                        } catch (error) {
                            return false;
                        }
                        return true;
                    },
                    "Tanssi-relay"
                );

                const externalValidators = await relayApi.query.externalValidators.externalValidators();
                expect(externalValidators).to.not.deep.eq(externalValidatorsBefore);

                const externalValidatorsHex = externalValidators.toJSON().map((x) => {
                    return u8aToHex(decodeAddress(x));
                });

                console.log("After message transfer:");

                console.log("Middleware: Operator keys are:", currentOperatorsKeys);
                console.log("Starlight: External validators are:", externalValidatorsHex);

                expect(externalValidatorsHex).to.deep.eq(currentOperatorsKeys);
            },
        });

        it({
            id: "T04",
            title: "Operator produces blocks",
            test: async () => {
                // wait some time for the operator to be part of session validator
                await waitSessions(
                    context,
                    relayApi,
                    6,
                    async () => {
                        try {
                            const sessionValidators = await relayApi.query.session.validators();
                            expect(sessionValidators).to.contain(operatorAccount.address);
                        } catch (error) {
                            return false;
                        }
                        return true;
                    },
                    "Tanssi-relay"
                );

                // In new era's first session at least one block need to be produced by the operator
                const blocksPerSession = 10;
                for (let i = 0; i < 3 * blocksPerSession; ++i) {
                    const latestBlockHash = await relayApi.rpc.chain.getBlockHash();
                    const author = (await relayApi.derive.chain.getHeader(latestBlockHash)).author;
                    if (author?.toString() === operatorAccount.address) {
                        return;
                    }
                    await context.waitBlock(1, "Tanssi-relay");
                }
                expect.fail("operator didn't produce a block");
            },
        });

        it({
            id: "T05",
            title: "Rewards and slashes are being sent to symbiotic successfully",
            test: async () => {
                // Send slash event forcefully
                const activeEraInfo = (await relayApi.query.externalValidators.activeEra()).toJSON();
                const currentExternalIndex = await relayApi.query.externalValidators.currentExternalIndex();
                const forceInjectSlashCall = relayApi.tx.externalValidatorSlashes.forceInjectSlash(
                    activeEraInfo.index,
                    operatorAccount.address,
                    1000,
                    currentExternalIndex
                );
                const forceInjectTx = await relayApi.tx.sudo.sudo(forceInjectSlashCall).signAndSend(alice);

                console.log("Force inject tx was submitted:", forceInjectTx.toHex());

                const blocksToWaitForRewards = await calculateNumberOfBlocksTillNextEra(relayApi, 10);
                // The slash message event is fired in the second block of Era
                const blocksToWaitForSlashes = blocksToWaitForRewards + 1;
                const blocksToWaitFor = Math.max(blocksToWaitForRewards, blocksToWaitForSlashes) + 1;

                const currentBlock = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                const blockToFetchRewardEventFrom = currentBlock + blocksToWaitForRewards;
                const blockToFetchSlashEventFrom = currentBlock + blocksToWaitForSlashes;

                console.log("We will wait till", currentBlock + blocksToWaitFor, "blocks");

                await context.waitBlock(blocksToWaitFor, "Tanssi-relay");
                const relayApiAtRewardEventBlock = await relayApi.at(
                    await relayApi.query.system.blockHash(blockToFetchRewardEventFrom)
                );
                const relayApiAtSlashEventBlock = await relayApi.at(
                    await relayApi.query.system.blockHash(blockToFetchSlashEventFrom)
                );

                // Get the reward event
                const rewardBlockEvents = await relayApiAtRewardEventBlock.query.system.events();
                const filteredEventsForReward = rewardBlockEvents.filter((a) => {
                    return a.event.method === "RewardsMessageSent";
                });
                expect(filteredEventsForReward.length).to.be.equal(1);
                const rewardEvent = filteredEventsForReward[0];
                // Extract message id
                const rewardMessageId = rewardEvent.event.toJSON().data[0];
                console.log("Reward message id:", rewardMessageId);

                // Get the slash event
                const slashBlockEvents = await relayApiAtSlashEventBlock.query.system.events();
                const filteredEventsForSlash = slashBlockEvents.filter((a) => {
                    return a.event.method === "SlashesMessageSent";
                });
                expect(filteredEventsForSlash.length).to.be.equal(1);
                const slashEvent = filteredEventsForSlash[0];
                // Extract message id
                const slashMessageId = slashEvent.event.toJSON().data[0];
                console.log("Slash message id:", slashMessageId);

                let rewardMessageReceived = false;
                let slashMessageReceived = false;
                let rewardMessageSuccess = false;
                let slashMessageSuccess = false;

                gatewayContract.on("InboundMessageDispatched", (_channelID, _nonce, messageID, success) => {
                    if (rewardMessageId === messageID) {
                        rewardMessageReceived = true;
                        rewardMessageSuccess = success;
                    } else if (slashMessageId === messageID) {
                        slashMessageReceived = true;
                        slashMessageSuccess = success;
                    }
                });

                while (!rewardMessageReceived || !slashMessageReceived) {
                    await sleep(1000);
                }

                expect(rewardMessageSuccess).to.be.true;
                expect(slashMessageSuccess).to.be.true;
            },
        });

        afterAll(async () => {
            console.log("Cleaning up");
            if (ethereumNodeChildProcess) {
                ethereumNodeChildProcess.kill("SIGINT");
            }
            if (relayerChildProcess) {
                relayerChildProcess.kill("SIGINT");
            }
            await execCommand("./scripts/bridge/cleanup.sh olep");
        });
    },
});
