import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { spawn, execSync } from "node:child_process";
import { exec } from "child_process";
import { signAndSendAndInclude } from "../../util/block.ts";

function execCommand(command: string, options?) {
    return new Promise((resolve, reject) => {
        exec(
            command,
            options,
            (error: child.ExecException, stdout: string, stderr: string) => {
                if (error) {
                    reject(error);
                } else {
                    resolve(stdout);
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
        let getBeefyCheckpointProcess;
        let deployContractsProcess;
        let getContractInfoProcess;
        let relayerChildProcess;
        let alice;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

            // //BeaconRelay
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const beaconRelay = keyring.addFromUri("//BeaconRelay", { name: "Beacon relay default"});

            const txHash = await relayApi.tx.balances
                .transferAllowDeath(beaconRelay.address, 1_000_000_000_000)
                .signAndSend(alice);
            console.log("Transferred money to beacon relay", txHash.toHex());

        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on tanssi-relay",
            test: async function () {
                ethereumNodeChildProcess = spawn("./scripts/bridge/start-ethereum-node.sh", {shell: true});
                ethereumNodeChildProcess.stdout.setEncoding('utf-8');
                ethereumNodeChildProcess.stdout.on('data', (chunk) => console.log(chunk));

                const beefyCheckpointOutput = await execCommand("./scripts/bridge/generate-beefy-checkpoint.sh", {env: {
                        RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                        ...process.env
                }});
                console.log(beefyCheckpointOutput);

                // Waiting till ethreum node produces one block
                console.log("Waiting till ethereum node produces one block");
                await sleep(20000);

                const deployOutput = await execCommand("./scripts/bridge/deploy-ethereum-contracts.sh", { shell: true });
                console.log(deployOutput);

                console.log("================== Contracts deployed");

                const contractInfoData = JSON.parse(<string>await execCommand("./scripts/bridge/generate-contract-info.sh"));
                console.log("Gateway proxy is:", contractInfoData.contracts.GatewayProxy.address);

                console.log("Got contract info data");

                const initialBeaconUpdate = JSON.parse(<string>await execCommand("./scripts/bridge/setup-relayer.sh", {
                    shell: true, env: {
                        RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                        ...process.env
                    }
                }));

                // We need to read initial checkpoint data and address of gateway contract to setup the ethereum client
                // Once that is done, we can start the relayer
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(relayApi.tx.ethereumBeaconClient.forceCheckpoint(initialBeaconUpdate)), alice);


                relayerChildProcess = spawn("./scripts/bridge/start-relayer.sh", {shell: true, env: {
                        RELAYCHAIN_ENDPOINT: "ws://127.0.0.1:9947",
                        ...process.env
                }});
                relayerChildProcess.stdout.setEncoding('utf-8');
                relayerChildProcess.stdout.on('data', (chunk) => console.log(chunk));
                relayerChildProcess.stderr.setEncoding('utf-8');
                relayerChildProcess.stderr.on('data', (chunk) => console.log(chunk));
            },
        });

        it({
            id: "T01",
            title: "Blocks are being produced on tanssi-relay",
            test: async function () {
                await sleep(1000000000);
            },
        });

    },
});

const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};
