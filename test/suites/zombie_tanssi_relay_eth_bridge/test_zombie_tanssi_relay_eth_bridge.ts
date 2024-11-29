import { beforeAll, describeSuite, expect, afterAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { spawn, exec } from "node:child_process";
import { signAndSendAndInclude, waitSessions } from "../../util/block.ts";
import { ethers } from "ethers";

// Change this if we change the storage parameter in runtime
const GATEWAY_STORAGE_KEY = "0xaed97c7854d601808b98ae43079dafb3";

function execCommand(command: string, options?) {
    return new Promise((resolve, reject) => {
        exec(command, options, (error: child.ExecException, stdout: string, stderr: string) => {
            if (error) {
                reject(error);
            } else {
                resolve({ stdout, stderr });
            }
        });
    });
}

describeSuite({
    id: "ZR-01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let relayApi: ApiPromise;
        let ethereumNodeChildProcess;
        let relayerChildProcess;
        let alice;
        let beefyClientDetails;
        let gatewayProxyAddress;
        let gatewayDetails;
        let ethereumWallet;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            // //BeaconRelay
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const beaconRelay = keyring.addFromUri("//BeaconRelay", { name: "Beacon relay default" });
            const executionRelay = keyring.addFromUri("//ExecutionRelay", { name: "Execution relay default" });

            const fundingTxHash = await relayApi.tx.utility
                .batch([
                    relayApi.tx.balances.transferAllowDeath(beaconRelay.address, 1_000_000_000_000),
                    relayApi.tx.balances.transferAllowDeath(executionRelay.address, 1_000_000_000_000),
                ])
                .signAndSend(alice);
            console.log("Transferred money to relayers", fundingTxHash.toHex());

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

            await execCommand("./scripts/bridge/deploy-ethereum-contracts.sh");

            console.log("Contracts deployed");

            const contractInfoData = JSON.parse(
                <string>(await execCommand("./scripts/bridge/generate-contract-info.sh")).stdout
            );

            console.log("BeefyClient contract address is:", contractInfoData.data.contracts.BeefyClient.address);
            beefyClientDetails = contractInfoData.data.contracts.BeefyClient;

            console.log("Gateway contract proxy address is:", contractInfoData.data.contracts.GatewayProxy.address);
            gatewayProxyAddress = contractInfoData.data.contracts.GatewayProxy.address;
            gatewayDetails = contractInfoData.data.contracts.Gateway;

            console.log("Setting gateway address to proxy contract:", gatewayProxyAddress);
            const setGatewayAddressTxHash = await relayApi.tx.sudo
                .sudo(relayApi.tx.system.setStorage([[GATEWAY_STORAGE_KEY, gatewayProxyAddress]]))
                .signAndSend(alice);
            console.log("Set gateway address transaction hash:", setGatewayAddressTxHash.toHex());

            const customHttpProvider = new ethers.WebSocketProvider("ws://127.0.0.1:8546");
            ethereumWallet = new ethers.Wallet(contractInfoData.ethereum_key, customHttpProvider);

            const initialBeaconUpdate = JSON.parse(
                <string>(
                    await execCommand("./scripts/bridge/setup-relayer.sh", {
                        env: {
                            RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                            ...process.env,
                        },
                    })
                ).stdout
            );

            // We need to read initial checkpoint data and address of gateway contract to setup the ethereum client
            // Once that is done, we can start the relayer
            await signAndSendAndInclude(
                relayApi.tx.sudo.sudo(relayApi.tx.ethereumBeaconClient.forceCheckpoint(initialBeaconUpdate)),
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
            test: async function () {
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
            test: async function () {
                const url = "ws://127.0.0.1:8546";
                const customHttpProvider = new ethers.WebSocketProvider(url);
                const beefyContract = new ethers.Contract(
                    beefyClientDetails.address,
                    beefyClientDetails.abi,
                    customHttpProvider
                );
                const currentBeefyBlock = Number(await beefyContract.latestBeefyBlock());
                expect(currentBeefyBlock).to.greaterThan(0);
                await waitSessions(context, relayApi, 1, null, "Tanssi-relay");
                const nextBeefyBlock = Number(await beefyContract.latestBeefyBlock());
                expect(nextBeefyBlock).to.greaterThan(currentBeefyBlock);
            },
        });

        it({
            id: "T03",
            title: "Message can be passed from ethereum to Starlight",
            test: async function () {
                const gatewayContract = new ethers.Contract(gatewayProxyAddress, gatewayDetails.abi, ethereumWallet);

                /** Example. Please remove it after the test is done
                try {
                    const tx = await gatewayContract.sendOperatorsData(["0x1234567890123456789012345678901234567890123456789012345678901234"], 1);
                    await tx.wait();
                } catch (error) {
                    throw new Error(`Failed to create vault: ${error.message}`, error.code);
                }

                await sleep(100000000);
                */
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

const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};
