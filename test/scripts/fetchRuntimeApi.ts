/**
fetch-api-info

Fetch the api info of a given endpoint
Output: api info as json
Usage:
# Fetch the rt-api info in a specific endpoint
pnpm get-api-info first.json --url ws://127.0.0.1:34100
 */

import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
import type { ApiPromise } from "@polkadot/api/promise/Api";
import fs from "node:fs";

// List of endpoints from which to get pallet metadata.
// Should contain one endpoint for each network.
const DEFAULT_ENDPOINTS = [
    "wss://rpc.polkadot.io",
    "wss://services.tanssi-testnet.network/dancelight",
    "wss://services.tanssi-dev.network/stagelight",
    "wss://services.tanssi-testnet.network/dancebox",
    "wss://fraa-flashbox-rpc.a.stagenet.tanssi.network",
    "wss://services.tanssi-dev.network/stagebox",
    // Relay chains, ideally we should use our relay endpoint
    "wss://rococo-rpc.polkadot.io",
    "wss://westend-rpc.polkadot.io",
    // Frontier template
    "wss://services.tanssi-testnet.network/dancebox-3001",
    // TODO: add simple template rpc endpoint
];

yargs(hideBin(process.argv))
    .usage("Usage: $0 [output]")
    .version("1.0.0")
    .command(
        "$0 [output]",
        "List pallets sorted by storage prefix or search for a storage key",
        (yargs) => {
            return yargs
                .positional("output", {
                    describe: "Output path where to store the json",
                    required: true,
                })
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                });
        },
        async (argv) => {
            let chosenEndpoint = argv.url;
            if (!chosenEndpoint) {
                // No URL provided via args, so display an interactive menu.
                const { select } = await import("@inquirer/prompts");
                chosenEndpoint = await select({
                    message: "Select an endpoint to list pallet prefixes (scroll down for more):",
                    choices: DEFAULT_ENDPOINTS,
                });
            }
            let api: ApiPromise;
            try {
                api = await getApiFor({ ...argv, url: chosenEndpoint });
            } catch (err) {
                console.error(`Failed to connect to ${chosenEndpoint}:"`, err);
                throw err;
            }
            try {
                const listOfRtApisAsJson = (await api.rpc.state.getRuntimeVersion()).toJSON();
                fs.writeFileSync(argv.output, JSON.stringify(listOfRtApisAsJson, null, 2), "utf8");
                console.log(listOfRtApisAsJson);
            } finally {
                await api.disconnect();
            }
            return;
        }
    )
    .parse();
