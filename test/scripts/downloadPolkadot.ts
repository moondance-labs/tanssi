/* eslint-disable */
import jsonFile from "../polkadotReleaseMapping.json" with { type: "json" };
import fs from "node:fs/promises";
import fsSync from "node:fs";
import assert from "node:assert";
import { parse } from "toml";
import path from "node:path";
import { execSync } from "node:child_process";
import { createHash } from "node:crypto";

const CONFIG = {
    FOLDER_NAME: "tmp",
    BINARIES: ["polkadot", "polkadot-execute-worker", "polkadot-prepare-worker"] as const,
    CARGO_PATH: "../Cargo.toml",
} as const;

async function main() {
    const polkadotVersionMappings: PolkadotVersionMapping = jsonFile;
    const fileContents = await fs.readFile(CONFIG.CARGO_PATH, "utf-8");
    const cargoToml = parse(fileContents) as CargoToml;
    const stableVersion = findPolkadotStableVersion(cargoToml.workspace.dependencies);
    console.log(`🔎 Found polkadot-sdk version: ${stableVersion}`);

    for (const binName of CONFIG.BINARIES) {
        const pathName = path.join(CONFIG.FOLDER_NAME, binName);
        if (fsSync.existsSync(pathName)) {
            const existingChecksum = getSha256(pathName);
            console.log(`✏️ File already exists: ${mini(existingChecksum)}`);

            const savedChecksum = polkadotVersionMappings[stableVersion]?.[binName];

            if (!savedChecksum || savedChecksum !== existingChecksum) {
                if (!savedChecksum) {
                    console.log(`⚠️ Saved checksum not found for ${binName}:${stableVersion}`);
                } else {
                    console.log(`⚠️ File mismatch ${mini(existingChecksum)} vs ${mini(savedChecksum)}, downloading...`);
                }
                execSync(`pnpm moonwall download -d ${binName} ${stableVersion} ${CONFIG.FOLDER_NAME}`, {
                    stdio: "inherit",
                });
                const sha256 = getSha256(pathName);
                polkadotVersionMappings[stableVersion] = {
                    ...polkadotVersionMappings[stableVersion],
                    [binName]: sha256,
                };
                await fs.writeFile("polkadotReleaseMapping.json", JSON.stringify(polkadotVersionMappings, null, 2));
            } else {
                console.log(`✅ Binary ${pathName} matches saved version`);
            }
        } else {
            // New File flow
            console.log("📥️ File does not exist, downloading...");
            execSync(`pnpm moonwall download ${binName} ${stableVersion} ${CONFIG.FOLDER_NAME}`, { stdio: "inherit" });
            const sha256 = getSha256(pathName);
            console.log(`💾 Downloaded file: ${mini(sha256)}`);
            polkadotVersionMappings[stableVersion] = {
                ...polkadotVersionMappings[stableVersion],
                [binName]: sha256,
            };
            await fs.writeFile("polkadotReleaseMapping.json", JSON.stringify(polkadotVersionMappings, null, 2));
            console.log("✅ Saved to version mapping ");
        }
    }
}

main()
    .then(() => console.log(`🎉 Finished verifying binaries: [${CONFIG.BINARIES.join(", ")}]`))
    .catch((err: unknown) => {
        console.error("❌ Error:", err);
        process.exit(1);
    });
/**
 * Interfaces
 **/

interface PolkadotVersionMapping {
    [key: `stable${number}-${number}` | `stable${number}`]: DownloadHashes;
}
interface CargoToml {
    workspace: {
        dependencies: Record<string, { git?: string; branch?: string }>;
    };
}

interface DownloadHashes {
    polkadot: string;
    "polkadot-execute-worker": string;
    "polkadot-prepare-worker": string;
}

/**
 * Functions
 **/
function extractStableVersion(branch: string): string | null {
    const match = branch.match(/stable(\d+)/);
    return match ? `stable${match[1]}` : null;
}

function findPolkadotStableVersion(dependencies: Record<string, any>): string {
    const polkadotDeps = Object.entries(dependencies).filter(
        ([_, config]) => typeof config === "object" && config.git === "https://github.com/moondance-labs/polkadot-sdk"
    );

    let stableVersions: Array<string> | Set<string> = new Set(
        polkadotDeps
            .map(([_, config]) => extractStableVersion(config.branch))
            .filter((version): version is string => version !== null)
    );

    if (stableVersions.size === 0) {
        throw new Error("No stable version found in polkadot-sdk dependencies");
    }

    if (stableVersions.size > 1) {
        stableVersions = Array.from(stableVersions).sort((a, b) => {
            // Extract numbers and compare
            const aMatch = a.match(/stable(\d+)(?:-(\d+))?/);
            const bMatch = b.match(/stable(\d+)(?:-(\d+))?/);

            assert(aMatch, "this is already mapped, this should never happen");
            assert(bMatch, "this is already mapped, this should never happen");

            const mainVersionDiff = Number.parseInt(bMatch[1]) - Number.parseInt(aMatch[1]);
            if (mainVersionDiff !== 0) {
                return mainVersionDiff;
            }

            const aSubVersion = aMatch[2] ? Number.parseInt(aMatch[2]) : 0;
            const bSubVersion = bMatch[2] ? Number.parseInt(bMatch[2]) : 0;
            return bSubVersion - aSubVersion;
        });
        console.error(
            `⚠️ Multiple stable versions found: ${Array.from(stableVersions).join(", ")}. Choosing: ${stableVersions[0]}`
        );
    }

    return Array.from(stableVersions)[0];
}

const getSha256 = (filePath: string) => {
    const fileBuffer = fsSync.readFileSync(filePath);
    const hashSum = createHash("sha256");
    hashSum.update(fileBuffer);
    return hashSum.digest("hex");
};

const mini = (hash: string) => `<${hash.slice(0, 4)}...${hash.slice(-4)}>`;
