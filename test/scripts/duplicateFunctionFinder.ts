/**
 * This script recursively scans a directory for TypeScript files and looks for duplicate
 * function definitions (by function name). It uses a crude regex to capture named functions
 * (optionally exported) and then reports any function names that occur more than once.
 *
 * Usage: ts-node duplicateFunctionFinder.ts check <rootDir>
 */
import fs from "node:fs";
import path from "node:path";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

interface FunctionOccurrence {
    file: string;
    line: number;
}

// Map to track function names and their occurrences.
const functionOccurrences: Map<string, FunctionOccurrence[]> = new Map();

/**
 * Process a single file: read its contents and check each line for a function definition.
 * Uses a crude regex matching lines like: "function functionName(...)" or "export function functionName(...)".
 */
function processFile(filePath: string): void {
    const content = fs.readFileSync(filePath, "utf-8");
    const lines = content.split("\n");
    const functionRegex = /(?:export\s+)?function\s+([a-zA-Z0-9_]+)/;

    lines.forEach((line, index) => {
        const trimmedLine = line.trim();
        // Skip single-line comments
        if (trimmedLine.startsWith("//")) return;

        // Process the line if it isn't commented out
        const match = line.match(functionRegex);
        if (match) {
            const functionName = match[1];
            if (!functionOccurrences.has(functionName)) {
                functionOccurrences.set(functionName, []);
            }
            functionOccurrences.get(functionName)?.push({
                file: filePath,
                line: index + 1,
            });
        }
    });
}

/**
 * Recursively process a directory: if an item is a directory, recurse into it; if it is a .ts file,
 * process it.
 */
function processDirectory(directory: string): void {
    const contents = fs.readdirSync(directory);
    for (const item of contents) {
        const fullPath = path.join(directory, item);
        if (fs.statSync(fullPath).isDirectory()) {
            processDirectory(fullPath);
        } else if (fullPath.endsWith(".ts")) {
            processFile(fullPath);
        }
    }
}

yargs(hideBin(process.argv))
    .usage("Usage: $0 check <rootDir>")
    .command(
        "check <rootDir>",
        "Checks all .ts files in a directory recursively for duplicate function definitions.",
        (yargs) => {
            return yargs.positional("rootDir", {
                describe: "The root directory to start scanning",
                type: "string",
            });
        },
        (argv: any) => {
            const rootDir = argv.rootDir;
            processDirectory(rootDir);

            // Print out any duplicate function definitions.
            let duplicatesFound = false;
            functionOccurrences.forEach((occurrences, functionName) => {
                if (occurrences.length > 1) {
                    duplicatesFound = true;
                    console.log(`Duplicate function definition found for "${functionName}":`);
                    occurrences.forEach((occurrence) => {
                        console.log(`  - ${occurrence.file} (line ${occurrence.line})`);
                    });
                    console.log("");
                }
            });

            if (!duplicatesFound) {
                console.log("No duplicate function definitions found.");
            }
        }
    )
    .help()
    .parse();
