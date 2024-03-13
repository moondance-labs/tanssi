import fs from "fs/promises";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { convertExponentials } from "@zombienet/utils";
import jsonBg from "json-bigint";
const JSONbig = jsonBg({ useNativeBigInt: true });

const ALICE_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
yargs(hideBin(process.argv))
    .usage("Usage: $0")
    .version("2.0.0")
    .command(
        `process <inputPath> <outputPath>`,
        "Overwrites a plainSpec with Alice modifications",
        (yargs) => {
            return yargs
                .positional("inputPath", {
                    describe: "Input path for plainSpecFile to modify",
                    type: "string",
                })
                .positional("outputPath", {
                    describe: "Output path for modified file",
                    type: "string",
                });
        },
        async (argv) => {
            process.stdout.write(`Reading from: ${argv.inputPath} ...`);
            const plainSpec = JSONbig.parse((await fs.readFile(argv.inputPath!)).toString());
            process.stdout.write(`Done ✅\n`);

            plainSpec.bootNodes = [];
            plainSpec.genesis.runtimeGenesis.config.invulnerables.invulnerables = [ALICE_ADDRESS];

            process.stdout.write(`Writing to: ${argv.outputPath} ...`);
            await fs.writeFile(argv.outputPath!, convertExponentials(JSONbig.stringify(plainSpec, null, 3)));
            process.stdout.write(`Done ✅\n`);
        }
    )
    .parse();
