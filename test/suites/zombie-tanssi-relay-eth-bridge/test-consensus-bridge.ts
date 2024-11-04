import { beforeAll, describeSuite, expect, afterAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { spawn, exec } from "node:child_process";
import { signAndSendAndInclude, waitSessions } from "../../util/block.ts";
import { ethers } from "ethers";

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

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            // //BeaconRelay
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const beaconRelay = keyring.addFromUri("//BeaconRelay", { name: "Beacon relay default" });

            const txHash = await relayApi.tx.balances
                .transferAllowDeath(beaconRelay.address, 1_000_000_000_000)
                .signAndSend(alice);
            console.log("Transferred money to beacon relay", txHash.toHex());

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
            console.log("BeefyClient contract address is:", contractInfoData.contracts.BeefyClient.address);
            beefyClientDetails = contractInfoData.contracts.BeefyClient;

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
                const customHttpProvider = new ethers.providers.WebSocketProvider(url);
                const beefyContract = new ethers.Contract(
                    beefyClientDetails.address,
                    beefyClientDetails.abi,
                    customHttpProvider
                );
                const currentBeefyBlock = (await beefyContract.latestBeefyBlock()).toNumber();
                expect(currentBeefyBlock).to.greaterThan(0);
                await waitSessions(context, relayApi, 1, null, "Tanssi-relay");
                const nextBeefyBlock = (await beefyContract.latestBeefyBlock()).toNumber();
                expect(nextBeefyBlock).to.greaterThan(currentBeefyBlock);
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
