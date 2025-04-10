import type { ApiPromise } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import { spawn } from "node:child_process";
import { createWriteStream } from "node:fs";

// Returns palletVersion stored on chain for each pallet
// Pallets with no explicit storage_version attribute return 0
// Old pallets may not have any value stored, they also return 0
export async function getOnchainPalletVersions(api: ApiPromise): Promise<Record<string, number>> {
    const versions: Record<string, number> = {};

    for (const moduleName in api.query) {
        // Ensure that the module has a 'palletVersion' query method.
        const moduleQuery = api.query[moduleName];
        if (typeof moduleQuery.palletVersion !== "function") {
            continue;
        }

        // Call the query and convert the result to a number.
        const palletVer = await moduleQuery.palletVersion();
        versions[moduleName] = palletVer.toNumber();
    }

    return versions;
}

export function readPalletVersionsFromGenesis(
    api: ApiPromise,
    genesis: Record<string, string>
): Record<string, number> {
    const versions: Record<string, number> = {};

    for (const moduleName in api.query) {
        // Ensure that the module has a 'palletVersion' query method.
        const moduleQuery = api.query[moduleName];
        if (typeof moduleQuery.palletVersion !== "function") {
            continue;
        }

        const keyValue = moduleQuery.palletVersion.key();
        const rawStorageVersion = genesis[keyValue];
        // TODO: if this returns an error because a pallet does not have anything in this storage, just set version to 0
        const palletStorageVersion = api.createType("u16", hexToU8a(rawStorageVersion));

        versions[moduleName] = palletStorageVersion.toNumber();
    }

    return versions;
}

/**
 * Asserts that for each pallet present in both afterRuntimeUpgradePalletVersions and genesisPalletVersions,
 * the version numbers are equal.
 *
 * @param afterRuntimeUpgradePalletVersions - A record mapping pallet names to version numbers.
 * @param genesisPalletVersions - A record mapping pallet names to version numbers.
 * @throws An error if any common pallet has a different version.
 */
export function assertPalletVersionsEqual(
    afterRuntimeUpgradePalletVersions: Record<string, number>,
    genesisPalletVersions: Record<string, number>
): void {
    let commonPalletCount = 0;
    const mismatches: Array<{ pallet: string; old: number; new: number }> = [];

    for (const pallet in afterRuntimeUpgradePalletVersions) {
        if (pallet in genesisPalletVersions) {
            commonPalletCount++;
            if (afterRuntimeUpgradePalletVersions[pallet] !== genesisPalletVersions[pallet]) {
                mismatches.push({
                    pallet,
                    old: afterRuntimeUpgradePalletVersions[pallet],
                    new: genesisPalletVersions[pallet],
                });
            }
        }
    }

    if (mismatches.length > 0) {
        // Compute maximum lengths for pallet names and old version strings.
        const maxNameLength = Math.max(...mismatches.map((m) => m.pallet.length));
        const maxOldLength = Math.max(...mismatches.map((m) => String(m.old).length));

        // Build each line so that the '!=' operator starts in the same column.
        const lines = mismatches
            .map((m) => `  ${m.pallet.padEnd(maxNameLength)}: ${String(m.old).padEnd(maxOldLength)} != ${m.new}`)
            .join("\n");

        throw new Error(
            `❌ ${mismatches.length} pallet version mismatches out of ${commonPalletCount} pallets. Missing migrations?\n${lines}`
        );
    }

    console.log(`✅ ${commonPalletCount} pallet versions match.`);
}

export async function buildRawSpecGenesisStorage(
    buildSpecBinary: string,
    buildSpecArgs: string[]
): Promise<Record<string, string>> {
    const childProcessLogFile = "tmp/buildSpecRawLogs.txt";

    // TODO: idea, instead of using build-spec subcommand, try to call the genesis builder runtime api
    // this way this test will not depend on executing external binary, only on using the existing on chain wasm runtime
    // ^ doesn't work because there is no runtime api method to return the raw storage, there is getPreset that returns
    // what seems to be a non-raw chain spec, but no way to convert it. Maybe using the omni-node?
    // specs/tanssi-relay.json
    /*
    $BINARY_FOLDER/tanssi-relay build-spec --chain dancelight-local --raw > raw_spec.json
     */

    // Spawn child process with stdout/stderr appropriately configured.
    const child = spawn(buildSpecBinary, buildSpecArgs, {
        stdio: ["inherit", "pipe", "pipe"],
    });

    // Capture stdout in chunks.
    const stdoutChunks = [];
    child.stdout.on("data", (chunk) => {
        stdoutChunks.push(chunk);
    });

    // Pipe stderr to a log file.
    const stderrLogStream = createWriteStream(childProcessLogFile);
    child.stderr.pipe(stderrLogStream);

    // Wait for the child process to complete using a promise.
    await new Promise<void>((resolve, reject) => {
        child.on("error", (error) => {
            console.error(`spawn error: ${error}`);
            reject(error);
        });
        child.on("exit", (code, signal) => {
            if (code !== 0) {
                reject(new Error(`Child process exited with code ${code}. Check logs in ${childProcessLogFile}`));
            } else if (signal) {
                reject(
                    new Error(`Child process was killed with signal ${signal}. Check logs in ${childProcessLogFile}`)
                );
            } else {
                resolve();
            }
        });
    });

    // Parse the captured stdout into JSON.
    let rawChainSpec: { genesis: { raw: { top: Record<string, string> } } };
    try {
        const stdoutBuffer = Buffer.concat(stdoutChunks).toString();
        rawChainSpec = JSON.parse(stdoutBuffer);
    } catch (err) {
        throw new Error(`Failed to parse JSON from child process output: ${err.message}`);
    }
    return rawChainSpec.genesis.raw.top;
}

// Assert that the pallet storage versions found on chain are the same as the storage versions generated in a new genesis.
// A mismatch means a missed migration that must be fixed.
export async function testPalletVersions(
    api: ApiPromise,
    buildSpecBinary: string,
    buildSpecArgs: string[]
): Promise<void> {
    const rawGenesis = await buildRawSpecGenesisStorage(buildSpecBinary, buildSpecArgs);
    const genesisPalletVersions = readPalletVersionsFromGenesis(api, rawGenesis);
    const afterRuntimeUpgradePalletVersions = await getOnchainPalletVersions(api);

    assertPalletVersionsEqual(afterRuntimeUpgradePalletVersions, genesisPalletVersions);
}
