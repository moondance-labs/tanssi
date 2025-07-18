import "@tanssi/api-augment/dancelight";

import { afterAll, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex, hexToU8a } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import { ethers } from "ethers";
import { type ChildProcessWithoutNullStreams, exec, spawn } from "node:child_process";
import {
    ASSET_HUB_CHANNEL_ID,
    ASSET_HUB_PARA_ID,
    SNOWBRIDGE_FEES_ACCOUNT,
    signAndSendAndInclude,
    sleep,
    waitSessions,
} from "utils";

import { keccak256 } from "viem";
import { ETHEREUM_NETWORK_TESTNET, FOREIGN_ASSET_ID } from "utils/constants";

// Change this if we change the storage parameter in runtime
const GATEWAY_STORAGE_KEY = "0xaed97c7854d601808b98ae43079dafb3";
const RESERVE_TRANSFER_FEE = 100000000000;

function execCommand(command: string, options?): Promise<{ stdout: string; stderr: string }> {
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
        let tokenContract: ethers.Contract;
        let wETHContract: ethers.Contract;
        let wETHAddress: string;
        let gatewayProxyAddress: string;
        let gatewayOwnerAddress: string;
        let middlewareAddress: string;
        let operatorRewardAddress: string;
        let operatorRewardContract: ethers.Contract;
        let operatorRewardContractImpl: ethers.Contract;
        let operatorRewardDetails: any;

        let tokenId: any;
        let wETHBalanceFromEthereum: bigint;
        let wETHTokenLocation: any;

        let ethInfo: any;

        let operatorAccount: KeyringPair;
        let operatorNimbusKey: string;
        let executionRelay: KeyringPair;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            relayCharlieApi = context.polkadotJs("Tanssi-charlie");

            // //BeaconRelay
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const beaconRelay = keyring.addFromUri("//BeaconRelay", {
                name: "Beacon relay default",
            });
            executionRelay = keyring.addFromUri("//ExecutionRelay", {
                name: "Execution relay default",
            });

            // Operator keys
            operatorAccount = keyring.addFromUri("//Charlie", {
                name: "Charlie default",
            });
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

            // We override the operator 3 key because it goes to a slashing vault
            await execCommand("./scripts/bridge/deploy-ethereum-contracts.sh", {
                env: {
                    OPERATOR3_KEY: u8aToHex(operatorAccount.addressRaw),
                    ...process.env,
                },
            });

            console.log("Contracts deployed");

            ethInfo = JSON.parse((await execCommand("./scripts/bridge/generate-eth-info.sh")).stdout);

            console.log("BeefyClient contract address is:", ethInfo.snowbridge_info.contracts.BeefyClient.address);
            beefyClientDetails = ethInfo.snowbridge_info.contracts.BeefyClient;

            console.log("Gateway contract proxy address is:", ethInfo.snowbridge_info.contracts.GatewayProxy.address);
            gatewayProxyAddress = ethInfo.snowbridge_info.contracts.GatewayProxy.address;

            console.log("WETH9 contract address is:", ethInfo.snowbridge_info.contracts.WETH9.address);
            wETHAddress = ethInfo.snowbridge_info.contracts.WETH9.address;

            console.log("Symbiotic middleware address is: ", ethInfo.symbiotic_info.contracts.MiddlewareProxy.address);
            const middlewareCallerDetails = ethInfo.symbiotic_info.contracts.Middleware;
            const middlewareReaderDetails = ethInfo.symbiotic_info.contracts.OBaseMiddlewareReader;
            const combinedMiddlewareAbi = [...middlewareCallerDetails.abi, ...middlewareReaderDetails.abi];
            middlewareAddress = ethInfo.symbiotic_info.contracts.MiddlewareProxy.address;

            console.log(
                "Symbiotic Rewards address is: ",
                ethInfo.symbiotic_info.contracts.ODefaultOperatorRewards.address
            );
            operatorRewardAddress = ethInfo.symbiotic_info.contracts.ODefaultOperatorRewards.address;
            operatorRewardDetails = ethInfo.symbiotic_info.contracts.ODefaultOperatorRewards;

            console.log("Setting gateway address to proxy contract:", gatewayProxyAddress);
            const setGatewayAddressTxHash = await signAndSendAndInclude(
                relayApi.tx.sudo.sudo(relayApi.tx.system.setStorage([[GATEWAY_STORAGE_KEY, gatewayProxyAddress]])),
                alice
            );
            console.log("Set gateway address transaction hash:", setGatewayAddressTxHash.txHash.toHex());

            customHttpProvider = new ethers.WebSocketProvider(ethUrl);
            ethereumWallet = new ethers.Wallet(ethInfo.ethereum_key, customHttpProvider);
            gatewayOwnerAddress = ethereumWallet.address.toLowerCase();

            // Setting up Middleware
            middlewareContract = new ethers.Contract(middlewareAddress, combinedMiddlewareAbi, ethereumWallet);

            const tx = await middlewareContract.setGateway(gatewayProxyAddress);
            await tx.wait();

            // Setting up operatorRewards
            operatorRewardContract = new ethers.Contract(
                await middlewareContract.i_operatorRewards(),
                operatorRewardDetails.abi,
                ethereumWallet
            );
            operatorRewardContractImpl = new ethers.Contract(
                operatorRewardAddress,
                operatorRewardDetails.abi,
                ethereumWallet
            );

            gatewayContract = new ethers.Contract(
                gatewayProxyAddress,
                ethInfo.snowbridge_info.contracts.Gateway.abi,
                ethereumWallet
            );

            wETHContract = new ethers.Contract(
                wETHAddress,
                ethInfo.snowbridge_info.contracts.WETH9.abi,
                ethereumWallet
            );

            const setMiddlewareTx = await gatewayContract.setMiddleware(middlewareAddress);
            await setMiddlewareTx.wait();

            const registerTokenFee = await gatewayContract.quoteRegisterTokenFee();
            const registerWETHTx = await gatewayContract.registerToken(wETHAddress, { value: registerTokenFee * 10n });
            await registerWETHTx.wait();

            const isWETHTokenRegistered = await gatewayContract.isTokenRegistered(wETHAddress);
            expect(isWETHTokenRegistered).to.be.true;

            const depositWETHTx = await wETHContract.deposit({ value: 10000000000000000000n });
            await depositWETHTx.wait();

            wETHBalanceFromEthereum = 300000000000000n;
            const approveWETHTx = await wETHContract.approve(gatewayProxyAddress, wETHBalanceFromEthereum);
            await approveWETHTx.wait();

            const initialBeaconUpdate = JSON.parse(
                (
                    await execCommand("./scripts/bridge/setup-relayer.sh", {
                        env: {
                            RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                            ...process.env,
                        },
                    })
                ).stdout
            );

            wETHTokenLocation = {
                parents: 1,
                interior: {
                    X2: [
                        {
                            GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                        },
                        {
                            AccountKey20: {
                                network: ETHEREUM_NETWORK_TESTNET,
                                key: hexToU8a(wETHAddress),
                            },
                        },
                    ],
                },
            };

            const tokenLocation = {
                parents: 0,
                interior: "Here",
            };
            const versionedNativeTokenLocation = {
                V3: tokenLocation,
            };

            const metadata = {
                name: "dance",
                symbol: "dance",
                decimals: 12,
            };

            // We need to read initial checkpoint data and address of gateway contract to setup the ethereum client
            // We also need to register token
            // Once that is done, we can start the relayer
            await signAndSendAndInclude(
                relayApi.tx.sudo.sudo(
                    relayApi.tx.utility.batch([
                        relayApi.tx.ethereumBeaconClient.forceCheckpoint(initialBeaconUpdate),
                        relayApi.tx.ethereumSystem.registerToken(versionedNativeTokenLocation, metadata),
                        relayApi.tx.foreignAssetsCreator.createForeignAsset(
                            wETHTokenLocation,
                            FOREIGN_ASSET_ID,
                            alice.address,
                            true,
                            1
                        ),
                    ])
                ),
                alice
            );

            // let's fetch the token id
            const allEntries = await relayApi.query.ethereumSystem.nativeToForeignId.entries();
            const tokenIds = allEntries.map(([, id]) => id.toHuman());

            tokenId = tokenIds[0];

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
                const currentOperatorsKeys = await middlewareContract.sortOperatorsByPower(epoch);

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

        it({
            id: "T06",
            title: "Rewards are claimable",
            test: async () => {
                // Find the first era index claimable
                const currentEra = (await relayApi.query.externalValidators.activeEra()).unwrap().index;

                let eraToAnalyze = currentEra.toNumber();

                const DEFAULT_ERA_ROOT = "0x0000000000000000000000000000000000000000000000000000000000000000";
                // Try to find latest reported era
                // eraRoot returns a struct of 5 items, the 4th of which is the era root
                // we try to retrieve the first non-default one
                while (
                    (await operatorRewardContract.eraRoot(eraToAnalyze))[3] === DEFAULT_ERA_ROOT &&
                    eraToAnalyze >= 0
                ) {
                    eraToAnalyze--;

                    console.log("era to analyze ", eraToAnalyze);
                    console.log(await operatorRewardContract.eraRoot(eraToAnalyze));
                    console.log("thid");
                    console.log((await operatorRewardContract.eraRoot(eraToAnalyze))[3]);
                }
                if (eraToAnalyze < 0) {
                    throw new Error("No era was found in operator rewards to be claimed");
                }

                const operatorMerkleProof = await relayApi.call.externalValidatorsRewardsApi.generateRewardsMerkleProof(
                    operatorAccount.address,
                    eraToAnalyze
                );

                const eraRewardsInfo = await relayApi.query.externalValidatorsRewards.rewardPointsForEra(eraToAnalyze);
                //(uint256, bytes, bytes)
                // no hints and I am passing a max admin fee
                const additionalData =
                    "0x0000000000000000000000000000000000000000000000000000000000001" +
                    "000000000000000000000000000000000000000000000000000000000000000006000000000000000000" +
                    "000000000000000000000000000000000000000000000800000000000000000000000000000000000000" +
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000" +
                    "0000000";

                const claimRewardsInput = {
                    operatorKey: operatorAccount.addressRaw,
                    eraIndex: eraToAnalyze,
                    totalPointsClaimable: eraRewardsInfo.individual.toJSON()[operatorAccount.address.toString()],
                    proof: operatorMerkleProof.toHuman().proof,
                    data: additionalData,
                };
                console.log(`Claiming rewards with inputs ${claimRewardsInput}`);
                expect(operatorMerkleProof.isEmpty).to.be.false;
                try {
                    const claimTx = await operatorRewardContract.claimRewards(claimRewardsInput);
                    await claimTx.wait();
                } catch (e) {
                    if (e.data) {
                        console.log(e.data);

                        const decodedError = operatorRewardContractImpl.interface.parseError(e.data);
                        throw new Error(`Failed to claim rewards with error: ${decodedError}`);
                    }
                    throw new Error(`Failed to claim rewards with error: ${e.toHuman()}`);
                }

                const tokenAddress = await gatewayContract.tokenAddressOf(tokenId);

                tokenContract = new ethers.Contract(
                    tokenAddress,
                    ethInfo.symbiotic_info.contracts.Token.abi,
                    ethereumWallet
                );

                const operator = await middlewareContract.operatorByKey(operatorAccount.addressRaw);
                const operatorBalance = await tokenContract.balanceOf(operator);
                expect(operatorBalance).to.not.be.eq(0n);
            },
        });

        it({
            id: "T07",
            title: "Slash reaches slasher contract",
            test: async () => {
                const epoch = await middlewareContract.getCurrentEpoch();
                const operatorAndVaults = await middlewareContract.getOperatorVaultPairs(epoch);
                const operator = await middlewareContract.operatorByKey(operatorAccount.addressRaw);
                const matchedPair = operatorAndVaults.find((operatorVaultPair) => operatorVaultPair[0] === operator);

                console.log("operator is ", operator);
                console.log("oepratorVaults ", operatorAndVaults);
                console.log("matchedPair ", matchedPair);

                const vaultDetails = ethInfo.symbiotic_info.contracts.Vault;
                const vaultContract = new ethers.Contract(matchedPair[1][0], vaultDetails.abi, ethereumWallet);
                const slasher = await vaultContract.slasher();
                // Here we load a random slasher, to check its type
                const slasherDetails = ethInfo.symbiotic_info.contracts.Slasher;
                const vetoSlasherDetails = ethInfo.symbiotic_info.contracts.VetoSlasher;

                // Setting up slasher
                const slasherContract = new ethers.Contract(slasher, slasherDetails.abi, ethereumWallet);

                const network = await middlewareContract.NETWORK();
                const subnetwork = `${network}000000000000000000000000`;
                // type 0 means instant slash
                if ((await slasherContract.TYPE()) === 0n) {
                    const cummulativeSlash = await slasherContract.cumulativeSlash(subnetwork, operator);
                    expect(cummulativeSlash).to.be.greaterThan(0);
                } else {
                    // else we hav e a veto slash
                    const vetoSlasherContract = new ethers.Contract(slasher, vetoSlasherDetails.abi, ethereumWallet);
                    const lengthSlashes = await vetoSlasherContract.slashRequestsLength();
                    expect(lengthSlashes).to.be.greaterThan(0);
                }
            },
        });

        it({
            id: "T08",
            title: "Native token is transferred to (and from) Ethereum successfully",
            test: async () => {
                // Wait a few sessions to ensure the token was properly registered on Ethereum
                await waitSessions(context, relayApi, 4, null, "Tanssi-relay");

                // How to encode the channel id for it to be compliant with Solidity
                const assetHubParaId = relayApi.createType("ParaId", ASSET_HUB_PARA_ID);
                const assetHubChannelId = keccak256(
                    new Uint8Array([...new TextEncoder().encode("para"), ...assetHubParaId.toU8a().reverse()])
                );
                expect(assetHubChannelId).to.be.eq(ASSET_HUB_CHANNEL_ID);

                // Get the channel info
                const channelInfo = (await relayApi.query.ethereumSystem.channels(assetHubChannelId)).unwrap().toJSON();

                const channelOperatingModeOf = await gatewayContract.channelOperatingModeOf(assetHubChannelId);

                // Ensure channel is in Normal operations mode
                expect(channelOperatingModeOf).to.be.eq(0n);

                // Create channel in EthereumTokenTransfers
                const setTokenTransferChannelTx = await relayApi.tx.sudo
                    .sudo(
                        relayApi.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            assetHubChannelId,
                            channelInfo.agentId.toString(),
                            Number(channelInfo.paraId)
                        )
                    )
                    .signAndSend(alice);

                console.log("Set token transfer channel tx was submitted:", setTokenTransferChannelTx.toHex());

                await context.waitBlock(2, "Tanssi-relay");

                // Ensure the channel is created on EthereumTokenTransfers
                const channelInfoFromEthTokenTransfers = (
                    await relayApi.query.ethereumTokenTransfers.currentChannelInfo()
                ).toJSON();
                expect(channelInfoFromEthTokenTransfers).to.not.be.undefined;

                const recipient = "0x90a987b944cb1dcce5564e5fdecd7a54d3de27fe";
                const amountFromStarlight = 1000000000000000n;

                const existentialDeposit = relayApi.consts.balances.existentialDeposit.toBigInt();
                const feesAccountBalanceBeforeSending = (await relayApi.query.system.account(SNOWBRIDGE_FEES_ACCOUNT))
                    .data.free;
                expect(feesAccountBalanceBeforeSending.toBigInt()).to.be.eq(existentialDeposit);

                // Send the token
                const transferNativeTokenTx = await relayApi.tx.ethereumTokenTransfers
                    .transferNativeToken(amountFromStarlight, recipient)
                    .signAndSend(alice);

                console.log("Transfer native token tx was submitted:", transferNativeTokenTx.toHex());

                await context.waitBlock(1, "Tanssi-relay");

                const events = await relayApi.query.system.events();
                const filteredEventForTokenTransfers = events.filter((a) => {
                    return a.event.method === "NativeTokenTransferred";
                });

                const tokenTransfersEventData = filteredEventForTokenTransfers[0].event.toJSON().data;

                const tokenTransferMessageId = tokenTransfersEventData[0];
                const tokenTransferChannelId = tokenTransfersEventData[1];
                const tokenTransferSource = tokenTransfersEventData[2];
                const tokenTransferRecipient = tokenTransfersEventData[3];
                const tokenTransferTokenId = tokenTransfersEventData[4];
                const tokenTransferAmount = tokenTransfersEventData[5];

                expect(tokenTransferChannelId).to.be.eq(assetHubChannelId);
                expect(tokenTransferSource).to.be.eq(alice.address);
                expect(tokenTransferRecipient).to.be.eq(recipient);
                expect(tokenTransferAmount).to.be.eq(Number(amountFromStarlight));

                // Fees are collected
                const feesAccountBalanceAfterSending = (await relayApi.query.system.account(SNOWBRIDGE_FEES_ACCOUNT))
                    .data.free;
                expect(feesAccountBalanceAfterSending.toNumber()).to.be.greaterThan(
                    feesAccountBalanceBeforeSending.toNumber()
                );

                // Ensure the expected tokenId is properly set on Starlight
                const expectedNativeToken = (
                    await relayApi.query.ethereumSystem.foreignToNativeId(tokenTransferTokenId)
                ).toJSON();
                expect(expectedNativeToken).to.not.be.undefined;

                // Ensure the token is properly registered on Ethereum
                const tokenAddress = await gatewayContract.tokenAddressOf(tokenTransferTokenId);
                const tokenIsRegistered = await gatewayContract.isTokenRegistered(tokenAddress);
                expect(tokenIsRegistered).to.be.true;

                let tokenTransferReceived = false;
                let tokenTransferSuccess = false;

                console.log("Waiting for InboundMessageDispatched event...");

                await gatewayContract.on("InboundMessageDispatched", (_channelID, _nonce, messageID, success) => {
                    if (tokenTransferMessageId === messageID) {
                        tokenTransferReceived = true;
                        tokenTransferSuccess = success;
                    }
                });

                while (!tokenTransferReceived) {
                    await sleep(1000);
                }

                expect(tokenTransferSuccess).to.be.true;

                // Send the token back
                const amountBackFromETH = 300000000000000n;
                const fee = 0n;

                console.log(`Sending ${amountBackFromETH} tokens back from ETH`);

                tokenContract = new ethers.Contract(
                    tokenAddress,
                    ethInfo.symbiotic_info.contracts.Token.abi,
                    ethereumWallet
                );

                const approvalTx = await tokenContract.approve(gatewayProxyAddress, amountBackFromETH);

                await approvalTx.wait();

                const ownerBalanceBefore = await tokenContract.balanceOf(recipient);
                expect(ownerBalanceBefore).to.eq(amountFromStarlight);

                const neededFeeWei = await gatewayContract.quoteSendTokenFee(tokenAddress, ASSET_HUB_PARA_ID, fee);

                const randomAccount = generateKeyringPair("sr25519");
                const randomBalanceBefore = (await relayApi.query.system.account(randomAccount.address)).data.free;
                const executionRelayBefore = (await relayApi.query.system.account(executionRelay.address)).data.free;
                expect(randomBalanceBefore.toBigInt()).to.be.eq(0n);

                // Send the native TANSSI token from Ethereum
                const tx = await gatewayContract.sendToken(
                    tokenAddress,
                    ASSET_HUB_PARA_ID,
                    {
                        kind: 1,
                        data: u8aToHex(randomAccount.addressRaw),
                    },
                    fee,
                    amountBackFromETH,
                    {
                        value: neededFeeWei * 10n,
                    }
                );

                await tx.wait();

                // Send the WETH token as well
                const neededFeeWETH = await gatewayContract.quoteSendTokenFee(wETHAddress, ASSET_HUB_PARA_ID, fee);
                const sendWETHTokenTx = await gatewayContract.sendToken(
                    wETHAddress,
                    ASSET_HUB_PARA_ID,
                    {
                        kind: 1,
                        data: u8aToHex(alice.addressRaw),
                    },
                    fee,
                    wETHBalanceFromEthereum,
                    {
                        value: neededFeeWETH * 10n,
                    }
                );

                await sendWETHTokenTx.wait();

                const ownerBalanceAfter = await tokenContract.balanceOf(recipient);

                // Ensure the token has been sent
                expect(ownerBalanceAfter).to.be.eq(ownerBalanceBefore - amountBackFromETH);

                // We retrieve the current nonce and wait at most 6 sessions to see the message being relayed
                const nonceInChannelBefore = await relayApi.query.ethereumInboundQueue.nonce(assetHubChannelId);

                // wait some time for the data to be relayed
                // As soon as the nonce increases, then we get out
                await waitSessions(
                    context,
                    relayApi,
                    6,
                    async () => {
                        try {
                            const nonceAfter = await relayApi.query.ethereumInboundQueue.nonce(assetHubChannelId);
                            expect(nonceAfter.toNumber()).to.be.eq(nonceInChannelBefore.toNumber() + 2);
                        } catch (error) {
                            return false;
                        }
                        return true;
                    },
                    "Tanssi-relay"
                );

                // Reward is reduced from fees account
                // at least the amount decided in localReward
                const localReward = (await relayApi.query.ethereumSystem.pricingParameters()).rewards.local.toBigInt();

                const feesAccountBalanceAfterReceiving = (await relayApi.query.system.account(SNOWBRIDGE_FEES_ACCOUNT))
                    .data.free;
                expect(
                    feesAccountBalanceAfterSending.toBigInt() - feesAccountBalanceAfterReceiving.toBigInt() >
                        localReward
                ).to.be.true;

                // Reward is added to execution relay account
                const executionRelayAfter = (await relayApi.query.system.account(executionRelay.address)).data.free;
                expect(executionRelayAfter.toNumber()).to.be.greaterThan(executionRelayBefore.toNumber());

                // Ensure the token has been received on the Starlight side
                const randomBalanceAfter = (await relayApi.query.system.account(randomAccount.address)).data.free;
                expect(randomBalanceAfter.toBigInt()).to.be.eq(randomBalanceBefore.toBigInt() + amountBackFromETH);

                // Ensure the WETH token has been received on the Starlight side
                const aliceWETHBalanceAfter = await relayApi.query.foreignAssets.account(
                    FOREIGN_ASSET_ID,
                    alice.address
                );

                expect(aliceWETHBalanceAfter.unwrap().balance.toBigInt()).to.be.eq(wETHBalanceFromEthereum);

                const ethLocation = {
                    V4: {
                        parents: 1,
                        interior: {
                            X1: [
                                {
                                    GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                                },
                            ],
                        },
                    },
                };

                const beneficiaryLocation = {
                    V4: {
                        parents: 0,
                        interior: {
                            X1: [
                                {
                                    AccountKey20: {
                                        network: ETHEREUM_NETWORK_TESTNET,
                                        key: hexToU8a(gatewayOwnerAddress),
                                    },
                                },
                            ],
                        },
                    },
                };

                const wETHBalanceToSend = wETHBalanceFromEthereum - 200000000000000n;
                const assets = {
                    V4: [
                        {
                            id: wETHTokenLocation,
                            fun: {
                                Fungible: wETHBalanceToSend,
                            },
                        },
                    ],
                };

                const wETHBalanceBefore = await wETHContract.balanceOf(gatewayOwnerAddress);

                console.log("Sending WETH back from Tanssi to Ethereum");

                const transferWETHTx = await relayApi.tx.xcmPallet
                    .transferAssets(ethLocation, beneficiaryLocation, assets, 0, "Unlimited")
                    .signAndSend(alice);

                console.log("Transfer WETH tx was submitted:", transferWETHTx.toHex());

                let wETHTransferReceived = false;
                let wETHTransferSuccess = false;

                await gatewayContract.on(
                    "InboundMessageDispatched",
                    async (_channelID, _nonce, _messageID, success) => {
                        const balanceAfter = await wETHContract.balanceOf(gatewayOwnerAddress);
                        expect(balanceAfter).to.be.eq(wETHBalanceBefore + wETHBalanceToSend);
                        wETHTransferReceived = true;
                        wETHTransferSuccess = success;
                    }
                );

                while (!wETHTransferReceived) {
                    await sleep(1000);
                }

                expect(wETHTransferSuccess).to.be.true;
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
