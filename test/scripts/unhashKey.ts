/**
unhash-key

Given a raw storage key, try to find which pallet it belongs to, and which storage item

Usage:
# List all pallet prefixes for a selected chain
pnpm unhash-key
# Search for a key in all endpoints
pnpm unhash-key 0x94eadf0156a8ad5156507773d0471e4a49f6c9aa90c04982c05388649310f22f
# Search for a key in a specific endpoint
pnpm unhash-key --url 'wss://dancelight.tanssi-api.network' 0x94eadf0156a8ad5156507773d0471e4a49f6c9aa90c04982c05388649310f22f
 */

import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
import { xxhashAsHex } from "@polkadot/util-crypto";
import type { ApiPromise } from "@polkadot/api/promise/Api";

const DEFAULT_ENDPOINTS = [
    "wss://dancelight.tanssi-api.network",
    "wss://stagelight.tanssi-dev.network",
    "wss://dancebox.tanssi-api.network",
    "wss://fraa-flashbox-rpc.a.stagenet.tanssi.network",
    "wss://stagebox.tanssi-dev.network",
    // Frontier template
    "wss://dancebox-3001.tanssi-api.network",
    // TODO: add simple template rpc endpoint
];

yargs(hideBin(process.argv))
    .usage("Usage: $0 [key]")
    .version("1.0.0")
    .command(
        "$0 [key]",
        "List pallets sorted by storage prefix or search for a storage key",
        (yargs) => {
            return yargs
                .positional("key", {
                    describe: "Storage key to search for. Prefixed by 0x",
                    type: "string",
                })
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                });
        },
        async (argv) => {
            // No key provided: list pallets from a single network
            if (!argv.key) {
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
                    // Wrap connection attempt in a 10 second timeout.
                    const connectPromise: Promise<ApiPromise> = getApiFor({ ...argv, url: chosenEndpoint });
                    const timeoutPromise: Promise<ApiPromise> = new Promise((_, reject) =>
                        setTimeout(() => reject(new Error("Connection timed out after 10 seconds")), 10000)
                    );
                    api = await Promise.race([connectPromise, timeoutPromise]);
                } catch (err) {
                    console.error(`Failed to connect to ${chosenEndpoint}: ${err}`);
                    throw err;
                }
                try {
                    console.log("All pallets sorted by storage prefix for chain:", api.runtimeChain.toString());
                    const pallets = [];
                    const mt = api.runtimeMetadata;
                    for (const module of mt.asLatest.pallets) {
                        if (module.storage.isNone) {
                            continue;
                        }
                        const prefix = xxhashAsHex(module.storage.unwrap().prefix.toString(), 128);
                        pallets.push({ pallet: module.name.toString(), prefix });
                    }
                    // Sort pallets by hex prefix lexicographically.
                    pallets.sort((a, b) => (a.prefix > b.prefix ? 1 : a.prefix < b.prefix ? -1 : 0));
                    for (const p of pallets) {
                        console.log(`${p.prefix} ${p.pallet}`);
                    }
                } finally {
                    await api.disconnect();
                }
                return;
            }

            // Key provided: search mode
            if (!argv.key.startsWith("0x")) {
                console.error("Key must start with 0x");
                process.exit(1);
            }

            // Determine the list of endpoints to check.
            // If a URL is provided via the network options, use that only.
            let endpoints = [];
            if (argv.url) {
                endpoints = [argv.url];
            } else {
                console.log("No rpc provided. Will try to check all the known endpoints. Use --url to specify one");
                endpoints = DEFAULT_ENDPOINTS;
            }

            let found = false;
            for (const endpoint of endpoints) {
                console.log(`\nTrying network endpoint: ${endpoint}`);
                let api: ApiPromise;
                try {
                    // Wrap connection attempt in a 10 second timeout.
                    const connectPromise: Promise<ApiPromise> = getApiFor({ ...argv, url: endpoint });
                    const timeoutPromise: Promise<ApiPromise> = new Promise((_, reject) =>
                        setTimeout(() => reject(new Error("Connection timed out after 10 seconds")), 10000)
                    );
                    api = await Promise.race([connectPromise, timeoutPromise]);
                } catch (err) {
                    console.error(`Failed to connect to ${endpoint}: ${err}`);
                    continue;
                }

                const mt = api.runtimeMetadata;
                // Iterate over all pallets.
                for (const module of mt.asLatest.pallets) {
                    if (module.storage.isNone) {
                        continue;
                    }
                    // Compute pallet storage prefix.
                    const prefix = xxhashAsHex(module.storage.unwrap().prefix.toString(), 128);
                    // Check if the provided key starts with the pallet prefix.
                    if (argv.key.startsWith(prefix)) {
                        // Found pallet, now find key
                        console.log("✅ Found matching prefix: pallet", module.name.toString(), prefix);
                        const storages = module.storage.unwrap().items;
                        let foundMatch = false;
                        for (const storage of storages) {
                            // Each storage key is computed by concatenating the pallet prefix with
                            // the 128-bit hash (without the '0x' prefix) of the storage item name.
                            const keyValue = prefix + xxhashAsHex(storage.name.toString(), 128).slice(2);
                            // Check if the provided key and the storage key are prefix matches.
                            if (argv.key.startsWith(keyValue)) {
                                console.log("✅ Found key:");
                                console.log("");
                                console.log(`${keyValue}: ${module.name.toString()} ${storage.name.toString()}`);
                                console.log("");
                                foundMatch = true;
                                found = true;
                                break;
                            }
                        }
                        if (!foundMatch) {
                            console.log("No matching storage found in this pallet.");
                        }
                        // Stop checking pallets if key found.
                        if (found) {
                            break;
                        }
                    }
                }
                await api.disconnect();
                // Stop checking endpoints if key found.
                if (found) {
                    break;
                }
            }
            if (!found) {
                console.log("Key not found in any provided network endpoint.");
            }
        }
    )
    .parse();
