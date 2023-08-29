/**
 * This script is designed to update test suite IDs within a directory structure,
 * mimicking the default behavior of Visual Studio Code's file explorer. It reads
 * through a directory, finds files with a specific function call (`describeSuite`),
 * and updates the suite's ID based on the file's position within the directory tree.
 *
 * The naming convention for suite IDs follows these rules:
 * 1. A prefix derived from the directory name.
 * 2. Directories are represented by a 2-digit number.
 * 3. Files are represented by a 2-digit number.
 *
 * Note: The script's sorting logic prioritizes, to match VSC's default behavior:
 * 1. Files with special characters or spaces.
 * 2. Files in a case-insensitive lexicographical order.
 */
import fs from "fs";
import path from "path";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

yargs(hideBin(process.argv))
    .usage("Usage: $0")
    .version("2.0.0")
    .command(
        `process <rootDir>`,
        "Changes the testsuite IDs based on positional order in the directory tree.",
        (yargs) => {
            return yargs.positional("rootDir", {
                describe: "Input path for plainSpecFile to modify",
                type: "string",
            });
        },
        async (argv: any) => {
            const rootDir = argv.rootDir;

            const topLevelDirs = fs
                .readdirSync(rootDir)
                .filter((dir) => fs.statSync(path.join(rootDir, dir)).isDirectory());
            const usedPrefixes: Set<string> = new Set();

            topLevelDirs.forEach((dir) => {
                const prefix = generatePrefix(dir, usedPrefixes);
                generateId(path.join(rootDir, dir), rootDir, prefix);
            });
        }
    )
    .help()
    .parse();

function generatePrefix(directory: string, usedPrefixes: Set<string>): string {
    let prefix = directory[0].toUpperCase();

    if (usedPrefixes.has(prefix)) {
        const match = directory.match(/[-_](\w)/);
        if (match) {
            // if directory name has a '-' or '_'
            prefix += match[1].toUpperCase();
        } else {
            prefix = directory[1].toUpperCase();
        }
    }

    while (usedPrefixes.has(prefix)) {
        const charCode = prefix.charCodeAt(1);
        if (charCode >= 90) {
            // If it's Z, wrap around to A
            prefix = String.fromCharCode(prefix.charCodeAt(0) + 1) + "A";
        } else {
            prefix = prefix[0] + String.fromCharCode(charCode + 1);
        }
    }

    usedPrefixes.add(prefix);
    return prefix;
}

function generateId(directory: string, rootDir: string, prefix: string): void {
    const contents = fs.readdirSync(directory);

    contents.sort((a, b) => {
        const aIsDir = fs.statSync(path.join(directory, a)).isDirectory();
        const bIsDir = fs.statSync(path.join(directory, b)).isDirectory();

        if (aIsDir && !bIsDir) return -1;
        if (!aIsDir && bIsDir) return 1;
        return customFileSort(a, b);
    });

    let fileCount = 1;
    let subDirCount = 1;

    for (const item of contents) {
        const fullPath = path.join(directory, item);

        if (fs.statSync(fullPath).isDirectory()) {
            const subDirPrefix = ("0" + subDirCount).slice(-2);
            generateId(fullPath, rootDir, prefix + subDirPrefix);
            subDirCount++;
        } else {
            const fileContent = fs.readFileSync(fullPath, "utf-8");
            if (fileContent.includes("describeSuite")) {
                const newId = prefix + ("0" + fileCount).slice(-2);
                const updatedContent = fileContent.replace(
                    /(describeSuite\s*?\(\s*?\{\s*?id\s*?:\s*?['"])[^'"]+(['"])/,
                    `$1${newId}$2`
                );
                fs.writeFileSync(fullPath, updatedContent);
            }
            fileCount++;
        }
    }
}

function hasSpecialCharacters(filename: string): boolean {
    return /[ \t!@#$%^&*()_+\-=[\]{};':"\\|,.<>/?]+/.test(filename);
}

function customFileSort(a: string, b: string): number {
    const aHasSpecialChars = hasSpecialCharacters(a);
    const bHasSpecialChars = hasSpecialCharacters(b);

    if (aHasSpecialChars && !bHasSpecialChars) return -1;
    if (!aHasSpecialChars && bHasSpecialChars) return 1;

    return a.localeCompare(b, undefined, { sensitivity: "accent" });
}
