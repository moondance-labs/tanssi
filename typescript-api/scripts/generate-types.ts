import { execSync, spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { existsSync, writeFileSync } from "node:fs";
import path from "node:path";

let nodeProcess: ChildProcessWithoutNullStreams | undefined = undefined;

async function main() {
    const CHAINS = ["dancebox"];

    const RUNTIME_CHAIN_SPEC = process.argv[2];

    // Bump package version
    if (process.argv.length > 2) {
        console.log(`Bump package version to 0.${RUNTIME_CHAIN_SPEC}.0`);
        execSync(`pnpm version --no-git-tag-version 0.${RUNTIME_CHAIN_SPEC}.0`, { stdio: "inherit" });
    }

    if (!existsSync("../target/release/tanssi-node")) {
        console.error("Missing ../target/release/tanssi binary");
        process.exit(1);
    }

    // Install dependencies
    execSync("pnpm install", { stdio: "inherit" });

    // Get runtimes metadata
    for (const CHAIN of CHAINS) {
        console.log(`Starting ${CHAIN} node`);
        nodeProcess = spawn("../target/release/tanssi-node", [
            "--no-hardware-benchmarks",
            "--no-telemetry",
            "--no-prometheus",
            "--alice",
            "--tmp",
            `--chain=${CHAIN}-local`,
            "--dev-service",
            "--wasm-execution=interpreted-i-know-what-i-do",
            "--rpc-port=9933",
            "--unsafe-force-node-key-generation"
        ]);

        const onProcessExit = () => {
            nodeProcess?.kill();
        };

        process.once("exit", onProcessExit);
        process.once("SIGINT", onProcessExit);

        nodeProcess.once("exit", () => {
            process.removeListener("exit", onProcessExit);
            process.removeListener("SIGINT", onProcessExit);
        });

        await new Promise((resolve, reject) => {
            const onData = (data: any) => {
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

                            execSync("pnpm run load:meta:local", { stdio: "inherit" });
                            nodeProcess?.kill();
                            setTimeout(() => { }, 5000); // Sleep for 5 seconds
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
            nodeProcess?.stdout?.on("error", (error) => {
                console.error(error);
                reject(error);
            });
        });
    }

    // Generate typescript api code
    console.log("Generating typescript api code...");
    execSync("pnpm run generate:defs", { stdio: "inherit" });
    execSync("pnpm run generate:meta", { stdio: "inherit" });
    execSync("pnpm run postgenerate", { stdio: "inherit" });

    // Build the package
    execSync("pnpm run build", { stdio: "inherit" });
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
