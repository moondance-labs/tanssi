import { Keyring } from "@polkadot/api";
import fs from "fs/promises";
import jsonBg from "json-bigint";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { chainSpecToContainerChainGenesisData } from "../util/genesis_data";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
const JSONbig = jsonBg({ useNativeBigInt: true });

yargs(hideBin(process.argv))
    .usage("Usage: $0")
    .version("1.0.0")
    .command(
        `*`,
        "Registers a parachain",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: false,
                        alias: "account",
                    },
                    chain: {
                        describe: "Input path of raw chainSpec file",
                        type: "string",
                    },
                })
                .demandOption(["chain", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                process.stdout.write(`Reading chainSpec from: ${argv.chain}\n`);
                const rawSpec = JSONbig.parse(await fs.readFile(argv.chain!, "utf8"));

                if (rawSpec.bootNodes?.length) {
                    process.stdout.write(
                        `Warning: this chainSpec file has some bootnodes, which must be written manually using sudo: ${JSON.stringify(
                            rawSpec.bootNodes
                        )}\n`
                    );
                }

                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                const containerChainGenesisData = chainSpecToContainerChainGenesisData(api, rawSpec);
                const tx = api.tx.registrar.register(rawSpec.para_id, containerChainGenesisData);
                process.stdout.write(`Sending transaction... `);
                const txHash = await tx.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write(`Done ✅\n`);
            } finally {
                await api.disconnect();
            }
        }
    )
    .parse();
