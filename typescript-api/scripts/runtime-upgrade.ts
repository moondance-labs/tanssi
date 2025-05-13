import { execSync, spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import chalk from "chalk";

let nodeProcess: ChildProcessWithoutNullStreams | undefined = undefined;

// Hack: polkadot-js does not support XCM v5 yet, we need to manually change some types
//
// Lookup88 => StagingXcmV5Junction
// Lookup77 => StagingXcmV5Junction
// The index of LookupXX depends on this comment in the same file:
//     /** @name StagingXcmV5Junction (77) */
//     /** @name StagingXcmV5Junction (89) */
/*
src/dancebox/interfaces/types-lookup.ts
1616:        readonly asX1: Vec<Lookup88>;
1618:        readonly asX2: Vec<Lookup88>;
1620:        readonly asX3: Vec<Lookup88>;
1622:        readonly asX4: Vec<Lookup88>;
1624:        readonly asX5: Vec<Lookup88>;
1626:        readonly asX6: Vec<Lookup88>;
1628:        readonly asX7: Vec<Lookup88>;
1630:        readonly asX8: Vec<Lookup88>;

src/dancelight/interfaces/types-lookup.ts & src/starlight/interfaces/types-lookup.ts
902:        readonly asX1: Vec<Lookup77>;
904:        readonly asX2: Vec<Lookup77>;
906:        readonly asX3: Vec<Lookup77>;
908:        readonly asX4: Vec<Lookup77>;
910:        readonly asX5: Vec<Lookup77>;
912:        readonly asX6: Vec<Lookup77>;
914:        readonly asX7: Vec<Lookup77>;
916:        readonly asX8: Vec<Lookup77>;
 */
function hackXcmV5Support() {
    // For dancebox, replace "Lookup88" with "StagingXcmV5Junction"
    const danceboxFilePath = "src/dancebox/interfaces/types-lookup.ts";
    hackTypeReplacement(danceboxFilePath, "Lookup90", "StagingXcmV5Junction", 8);

    // For dancelight, replace "Lookup77" with "StagingXcmV5Junction"
    const dancelightFilePath = "src/dancelight/interfaces/types-lookup.ts";
    hackTypeReplacement(dancelightFilePath, "Lookup76", "StagingXcmV5Junction", 8);

    // For starlight, replace "Lookup77" with "StagingXcmV5Junction"
    const starlightFilePath = "src/starlight/interfaces/types-lookup.ts";
    hackTypeReplacement(starlightFilePath, "Lookup76", "StagingXcmV5Junction", 8);
}

function hackTypeReplacement(filePath: string, oldType: string, newType: string, expectedCount: number) {
    if (!existsSync(filePath)) {
        console.error(chalk.red(`Error: File ${filePath} does not exist.`));
        process.exit(1);
    }
    const content = readFileSync(filePath, "utf-8");

    console.log("XCM v5 hack: updating ", filePath);
    logMatchingLines(filePath, "@name StagingXcmV5Junction ");
    console.log("Line above should say", oldType);

    const regex = new RegExp(oldType, "g");
    const matches = content.match(regex);
    const count = matches ? matches.length : 0;
    if (count !== expectedCount) {
        // This check is to ensure we don't accidentally replace more than needed, if there is a Lookup777 for example,
        // we only want to replace Lookup77
        console.error(
            chalk.red(
                `Error: Expected ${expectedCount} occurrences of "${oldType}" in ${filePath} but found ${count}. Aborting hack.`
            )
        );
        process.exit(1);
    }
    const newContent = content.replace(regex, newType);
    writeFileSync(filePath, newContent);
    console.log(
        chalk.green(`Successfully replaced ${count} occurrences of "${oldType}" with "${newType}" in ${filePath}`)
    );
}

function logMatchingLines(filePath: string, substring: string) {
    const content = readFileSync(filePath, "utf-8");
    const lines = content.split(/\r?\n/);
    for (const line of lines) {
        if (line.includes(substring)) {
            console.log(`Found matching line in ${filePath}: ${line}`);
        }
    }
}

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

    // Hack: polkadot-js does not support XCM v5 yet, we need to manually change some types
    hackXcmV5Support();

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
