import solc from "solc";
import chalk from "chalk";
import fs from "node:fs/promises";
import path from "node:path";
import type { Compiled } from "../util/ethereum-contracts";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

const sourceByReference = {} as { [ref: string]: string };
const countByReference = {} as { [ref: string]: number };
const refByContract = {} as { [contract: string]: string };

// For some reasons, solc doesn't provide the relative path to imports :(
const getImports = (fileRef: string) => (dependency: string) => {
    if (sourceByReference[dependency]) {
        countByReference[dependency] = (countByReference[dependency] || 0) + 1;
        return { contents: sourceByReference[dependency] };
    }
    let base = fileRef;
    while (base && base.length > 1) {
        const localRef = path.join(base, dependency);
        if (sourceByReference[localRef]) {
            countByReference[localRef] = (countByReference[localRef] || 0) + 1;
            return { contents: sourceByReference[localRef] };
        }
        base = path.dirname(base);
        if (base === ".") {
        }
    }
    return { error: "Source not found" };
};

function compileSolidity(fileRef: string, contractContent: string): { [name: string]: Compiled } {
    const filename = path.basename(fileRef);
    const result = JSON.parse(
        solc.compile(
            JSON.stringify({
                language: "Solidity",
                sources: {
                    [filename]: {
                        content: contractContent,
                    },
                },
                settings: {
                    optimizer: { enabled: true, runs: 200 },
                    outputSelection: {
                        "*": {
                            "*": ["*"],
                        },
                    },
                    debug: {
                        revertStrings: "debug",
                    },
                },
            }),
            { import: getImports(fileRef) }
        )
    );
    if (!result.contracts) {
        throw result;
    }
    return Object.keys(result.contracts[filename]).reduce(
        (p, contractName) => {
            p[contractName] = {
                byteCode: `0x${result.contracts[filename][contractName].evm.bytecode.object}`,
                contract: result.contracts[filename][contractName],
                sourceCode: contractContent,
            };
            return p;
        },
        {} as { [name: string]: Compiled }
    );
}

// Shouldn't be run concurrently with the same 'name'
async function compile(fileRef: string, destPath: string): Promise<{ [name: string]: Compiled }> {
    const soliditySource = sourceByReference[fileRef];
    countByReference[fileRef]++;
    if (!soliditySource) {
        throw new Error(`Missing solidity file: ${fileRef}`);
    }
    const compiledContracts = compileSolidity(fileRef, soliditySource);

    await Promise.all(
        Object.keys(compiledContracts).map(async (contractName) => {
            const dest = `${path.join(destPath, path.dirname(fileRef), contractName)}.json`;
            if (refByContract[dest]) {
                console.warn(
                    chalk.red(
                        `Contract ${contractName} already exist from ${refByContract[dest]}. Erasing previous version`
                    )
                );
            }
            await fs.mkdir(path.dirname(dest), { recursive: true });
            await fs.writeFile(dest, JSON.stringify(compiledContracts[contractName]), {
                flag: "w",
            });
            console.log(`  - ${chalk.green(`${contractName}.json`)} file has been saved!`);
            refByContract[dest] = fileRef;
        })
    );
    return compiledContracts;
}

async function getFiles(dir) {
    const subdirs = await fs.readdir(dir);
    const files = await Promise.all(
        subdirs.map(async (subdir) => {
            const res = path.resolve(dir, subdir);
            return (await fs.stat(res)).isDirectory() ? getFiles(res) : res;
        })
    );
    return files.reduce((a, f) => a.concat(f), []);
}

const main = async () => {
    const args = process.argv.slice(2);
    const __filename = fileURLToPath(import.meta.url);
    const __dirname = dirname(__filename);

    // Order is important so precompiles are available first
    const contractSourcePaths = [
        {
            filepath:
                args.length > 0 && args[0] !== "undefined" ? args[0] : path.join(__dirname, "../contracts/solidity"),
            importPath: "", // Reference in contracts are local
            compile: true,
        },
    ];

    const sourceToCompile = {};
    for (const contractPath of contractSourcePaths) {
        const contracts = (await getFiles(contractPath.filepath)).filter((filename) => filename.endsWith(".sol"));
        for (const filepath of contracts) {
            const ref = filepath.replace(contractPath.filepath, contractPath.importPath).replace(/^\//, "");
            sourceByReference[ref] = (await fs.readFile(filepath)).toString();
            if (contractPath.compile) {
                countByReference[ref] = 0;
                if (!sourceByReference[ref].includes("// skip-compilation")) {
                    sourceToCompile[ref] = sourceByReference[ref];
                }
            }
        }
    }

    // Compile contracts
    for (const ref of Object.keys(sourceToCompile)) {
        try {
            await compile(ref, "./helpers/compiled/");
        } catch (e) {
            console.log(`Failed to compile: ${ref}`);
            if (e.errors) {
                for (const error of e.errors) {
                    console.log(error.formattedMessage);
                }
            } else {
                console.log(e);
            }
            process.exit(1);
        }
    }
    for (const ref of Object.keys(countByReference)) {
        if (!countByReference[ref]) {
            console.log(`${chalk.red("Warning")}: ${ref} never used: ${countByReference[ref]}`);
        }
    }

    // Forcing exit to avoid solc maintaining the process
    process.exit(0);
};

main();
