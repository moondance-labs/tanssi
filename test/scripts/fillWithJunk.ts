import { Keyring } from "@polkadot/api";
import fs from "fs/promises";
import jsonBg from "json-bigint";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { chainSpecToContainerChainGenesisData } from "../util/genesis_data";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
import { numberToHex, u8aToHex } from "@polkadot/util";

const JSONbig = jsonBg({ useNativeBigInt: true });

yargs(hideBin(process.argv))
    .usage("Usage: $0")
    .version("1.0.0")
    .command(
        `*`,
        "Fill a parachain with junk",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: true,
                        alias: "account",
                    },
                    iterations: {
                        describe: "Number of iterations",
                        type: "number",
                    },
                    sleep: {
                        describe: "Number of seconds to sleep before",
                        type: "number",
                    },
                    length: {
                        describe: "Number of seconds to sleep before",
                        type: "number",
                    }
                })
                .demandOption(["account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv);
            const keyring = new Keyring({ type: "sr25519" });
            let arr = new Uint8Array(argv.length);
            for (let i = 0; i < arr.length; i++) {
                arr[i] = Math.floor(Math.random() * 256);
            }
            console.log(arr)
            try {
                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);
                let nonce = (await api.rpc.system.accountNextIndex(account.address)).toNumber();

                for (let i = 0; i < argv.iterations; i++) {
                    const tx = api.tx.sudo.sudo(
                        api.tx.system.setStorage([[numberToHex(i+1), u8aToHex(arr)]])
                    );
                    await tx.signAndSend(account, {nonce: nonce});
                    nonce = nonce +1;
                    await delay(argv.sleep)
                  }
            } finally {
                await api.disconnect();
            }
        }
    )
    .parse();

function delay(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}