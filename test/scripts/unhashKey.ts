#!/usr/bin/env node
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
import { xxhashAsHex } from '@polkadot/util-crypto';

yargs(hideBin(process.argv))
    .usage("Usage: $0 [key]")
    .version("1.0.0")
    .command(
        "$0 [key]",
        "List pallets sorted by storage prefix or search for a storage key",
        (yargs) => {
            return yargs
                .positional("key", {
                    describe: "Hex key to search for",
                    type: "string",
                })
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                });
        },
        async (argv) => {
            const api = await getApiFor(argv);
            try {
                if (!argv.key) {
                    // No key provided: list all pallets sorted by storage prefix.
                    console.log("All pallets sorted by storage prefix:");
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
                    pallets.forEach((p) => {
                        console.log(`${p.prefix}  ${p.pallet}`);
                    });
                } else {
                    if (!argv.key.startsWith("0x")) {
                        console.error("Key must start with 0x");
                        throw new Error("Key must start with 0x");
                    }
                    // A key is provided: search for matching storage queries in each pallet.
                    console.log(`Searching for key ${argv.key}`);
                    const pallets = [];
                    const mt = api.runtimeMetadata;

                    for (const module of mt.asLatest.pallets) {
                        if (module.storage.isNone) {
                            continue;
                        }
                        const prefix = xxhashAsHex(module.storage.unwrap().prefix.toString(), 128);
                        pallets.push({ pallet: module.name.toString(), prefix });

                        if (argv.key.startsWith(prefix)) {
                            // Found pallet, now find key
                            console.log("✅ Found matching prefix: pallet", module.name.toString());

                            const storages = module.storage.unwrap().items;

                            let foundMatch = false;
                            for (const storage of storages) {
                                let keyValue = prefix + xxhashAsHex(storage.name.toString(), 128).slice(2);

                                // Check if the provided key and the storage key are prefix matches.
                                if (argv.key.startsWith(keyValue)) {
                                    console.log("✅ Found key:");
                                    console.log("");
                                    console.log(`${keyValue}: ${module.name.toString()} ${storage.name.toString()}`);
                                    console.log("");
                                    foundMatch = true;
                                    break;
                                }
                            }
                            if (!foundMatch) {
                                console.log("No matching storage found");
                            }

                        }
                    }
                }
            } finally {
                await api.disconnect();
            }
        }
    )
    .parse();
