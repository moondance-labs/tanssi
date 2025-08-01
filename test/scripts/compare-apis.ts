/**
compare-apis

Compare API versions between two JSON files.
Usage:
pnpm compare-apis file1.json file2.json
*/

import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import fs from "node:fs";

type ApiEntry = [string, number];

function parseApisFromFile(filePath: string): Record<string, number> {
    const content = fs.readFileSync(filePath, "utf-8");
    const json = JSON.parse(content);

    if (!Array.isArray(json.apis)) {
        throw new Error(`Invalid format in ${filePath}: 'apis' field must be an array`);
    }

    const apis: Record<string, number> = {};
    for (const entry of json.apis) {
        if (!Array.isArray(entry) || entry.length !== 2) {
            throw new Error(`Invalid API entry format in ${filePath}: each 'apis' item must be a [hex, version] tuple`);
        }
        const [hex, version] = entry as ApiEntry;
        apis[hex] = version;
    }

    return apis;
}

yargs(hideBin(process.argv))
    .usage("Usage: $0 <file1> <file2>")
    .command(
        "$0 <file1> <file2>",
        "Compare API versions between two JSON files",
        (yargs) =>
            yargs
                .positional("file1", {
                    describe: "First JSON file with .apis",
                    type: "string",
                    demandOption: true,
                })
                .positional("file2", {
                    describe: "Second JSON file with .apis",
                    type: "string",
                    demandOption: true,
                }),
        (argv) => {
            const file1 = argv.file1 as string;
            const file2 = argv.file2 as string;

            let apis1: Record<string, number>;
            let apis2: Record<string, number>;

            try {
                apis1 = parseApisFromFile(file1);
                apis2 = parseApisFromFile(file2);
            } catch (err) {
                console.error("❌ Error reading or parsing files:", err);
                process.exit(1);
            }

            console.log("🔍 Comparing API versions...");

            let exitCode = 0;
            for (const hex in apis1) {
                if (hex in apis2) {
                    if (apis1[hex] !== apis2[hex]) {
                        console.log(`❌ API ${hex} has different versions: ${apis1[hex]} vs ${apis2[hex]}`);
                        exitCode = 1;
                    }
                }
            }

            if (exitCode === 0) {
                console.log("✅ All matching APIs have the same versions.");
            }

            process.exit(exitCode);
        }
    )
    .help()
    .strict()
    .parse();
