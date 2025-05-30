import { execSync, spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { existsSync, writeFileSync } from "node:fs";
import path from "node:path";
import chalk from "chalk";

let nodeProcess: ChildProcessWithoutNullStreams | undefined = undefined;

async function main() {
    const CHAINS = ["dancebox", "flashbox", "dancelight", "starlight"];

    const RUNTIME_CHAIN_SPEC = process.argv[2];

    // Bump package version
    if (process.argv.length > 2) {
        console.log(`Bump package version to 0.${RUNTIME_CHAIN_SPEC}.0`);
        execSync(`pnpm version --no-git-tag-version 0.${RUNTIME_CHAIN_SPEC}.0`, {
            stdio: "inherit",
        });
    }

    if (!existsSync("../target/release/tanssi-node")) {
        console.error("Missing ../target/release/tanssi binary");
        process.exit(1);
    }

    // Get runtimes metadata
    for (const CHAIN of CHAINS) {
        console.log(`Starting ${CHAIN} node`);
        const isStarlightChain = CHAIN.includes("light");
        nodeProcess = spawn(`../target/release/tanssi-${isStarlightChain ? "relay" : "node"}`, [
            "--no-hardware-benchmarks",
            "--no-telemetry",
            "--no-prometheus",
            "--alice",
            "--tmp",
            `--chain=${CHAIN}-local`,
            "--dev-service",
            "--wasm-execution=interpreted-i-know-what-i-do",
            "--rpc-port=9933",
            "--unsafe-force-node-key-generation",
            "--rpc-cors=all",
        ]);

        nodeProcess.stdout.on("data", (data) => {
            console.log(`stdout: ${data}`);
        });

        nodeProcess.stderr.on("data", (data) => {
            console.error(`stderr: ${data}`);
        });

        const onProcessExit = (code: number) => {
            console.log(`Process exited with code: ${code}`);
            nodeProcess?.kill();
        };

        const onSignal = (signal: NodeJS.Signals) => {
            console.log(`Received signal: ${signal}`);
            nodeProcess?.kill();
        };

        process.once("exit", onProcessExit);
        process.once("SIGINT", onSignal);

        await new Promise((resolve, reject) => {
            const onData = async (data: any) => {
                if (data.includes("Running JSON-RPC server")) {
                    console.log(`Getting ${CHAIN} metadata`);

                    const requestOptions = {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({
                            id: "1",
                            jsonrpc: "2.0",
                            method: "state_getMetadata",
                            params: [],
                        }),
                    };

                    fetch("http://localhost:9933", requestOptions)
                        .then((response) => response.json())
                        .then((data) => {
                            writeFileSync(path.join(process.cwd(), `metadata-${CHAIN}.json`), JSON.stringify(data));

                            nodeProcess?.kill();
                            setTimeout(() => {}, 5000); // Sleep for 5 seconds
                            resolve("success");
                        });
                }
            };

            nodeProcess?.stderr?.on("data", onData);
            nodeProcess?.stdout?.on("data", onData);

            nodeProcess?.stderr?.on("error", (error) => {
                console.error(error);
                reject(error);
            });
            nodeProcess?.stderr?.on("error", (error) => {
                console.error(error);
                reject(error);
            });
        });
    }

    // Generate typescript api code
    console.log("Generating typescript api code...");
    execSync("pnpm run generate:defs", { stdio: "inherit" });
    execSync("pnpm run generate:meta", { stdio: "inherit" });

    // Build the package
    console.log("Building package...");
    execSync("pnpm run build", { stdio: "inherit" });

    console.log("Post build...");
    execSync("pnpm run postgenerate", { stdio: "inherit" });

    console.log(`Script complete ${chalk.bgBlackBright.greenBright("api-augment")} package built successfully ✅`);
}

main()
    .catch((error) => {
        console.error(error);
        nodeProcess?.kill();
        process.exit(1);
    })
    .then(() => {
        nodeProcess?.kill();
        process.exit(0);
    });
