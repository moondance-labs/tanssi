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
        `register`,
        "Registers a parachain, adds bootnodes, and sets it valid for collating",
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
                    parathread: {
                        describe: "Set the chain as a parathread instead of a parachain",
                        type: "boolean",
                        default: false,
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

                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                const containerChainGenesisData = chainSpecToContainerChainGenesisData(api, rawSpec);
                const txs = [];
                let tx1;
                if (argv.parathread) {
                    const slotFreq = api.createType("TpTraitsSlotFrequency", {
                        min: 1,
                        max: 1,
                    });
                    tx1 = api.tx.registrar.registerParathread(rawSpec.para_id, slotFreq, containerChainGenesisData);
                } else {
                    tx1 = api.tx.registrar.registerParathread(rawSpec.para_id, containerChainGenesisData);
                }
                txs.push(tx1);
                if (rawSpec.bootNodes?.length) {
                    const tx2 = api.tx.dataPreservers.setBootNodes(rawSpec.para_id, rawSpec.bootNodes);
                    const tx2s = api.tx.sudo.sudo(tx2);
                    txs.push(tx2s);
                }
                const tx3 = api.tx.registrar.markValidForCollating(rawSpec.para_id);
                const tx3s = api.tx.sudo.sudo(tx3);
                txs.push(tx3s);

                if (txs.length == 2) {
                    process.stdout.write(`Sending register transaction (register + markValidForCollating)... `);
                } else {
                    process.stdout.write(
                        `Sending register transaction (register + setBootNodes + markValidForCollating)... `
                    );
                }
                const txBatch = api.tx.utility.batchAll(txs);
                const txHash = await txBatch.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write(`Done ✅\n`);
            } finally {
                await api.disconnect();
            }
        }
    )
    .command(
        `markValidForCollating`,
        "Marks a registered parachain as valid, allowing collators to start collating",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: false,
                        alias: "account",
                    },
                    "para-id": {
                        describe: "Container chain para id",
                        type: "number",
                    },
                })
                .demandOption(["para-id", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                let tx = api.tx.registrar.markValidForCollating(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
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
    .command(
        `setBootNodes`,
        "Set bootnodes for a container chain",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: false,
                        alias: "account",
                    },
                    "para-id": {
                        describe: "Container chain para id",
                        type: "number",
                    },
                    bootnode: {
                        describe: "Container chain para id",
                        type: "array",
                    },
                    "keep-existing": {
                        describe: "Keep exisiting bootnodes, and append to the list instead of overwriting them",
                        type: "boolean",
                    },
                    "mark-valid-for-collating": {
                        describe: "Also mark the registered chain as valid, if it was not marked already",
                        type: "boolean",
                    },
                })
                .demandOption(["para-id", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv as any);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                let bootnodes = [];
                if (argv.keepExisting) {
                    // Read existing bootnodes
                    const onChainBootnodes = (await api.query.registrar.bootNodes(argv.paraId)) as any;
                    bootnodes = [...bootnodes, ...onChainBootnodes];
                }
                if (!argv.bootnode) {
                    argv.bootnode = [];
                }
                bootnodes = [...bootnodes, ...argv.bootnode];

                const tx1 = api.tx.dataPreservers.setBootNodes(argv.paraId, bootnodes);
                const tx1s = api.tx.sudo.sudo(tx1);
                let tx2s = null;
                if (argv.markValidForCollating) {
                    // Check if not already valid, and only in that case call markValidForCollating
                    const notValidParas = (await api.query.registrar.pendingVerification()) as any;
                    if (notValidParas.toJSON().includes(argv.paraId)) {
                        process.stdout.write(`Will set container chain valid for collating\n`);
                        const tx2 = api.tx.registrar.markValidForCollating(argv.paraId);
                        tx2s = api.tx.sudo.sudo(tx2);
                    } else {
                        // ParaId already valid, or not registered at all
                        process.stdout.write(`Not setting container chain valid for collating\n`);
                    }
                }
                let tx;
                if (tx2s != null) {
                    tx = api.tx.utility.batchAll([tx1s, tx2s]);
                } else {
                    tx = tx1s;
                }
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
    .command(
        `deregister`,
        "Deregister a container chain",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: false,
                        alias: "account",
                    },
                    "para-id": {
                        describe: "Container chain para id",
                        type: "number",
                    },
                })
                .demandOption(["para-id", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv as any);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                let tx = api.tx.registrar.deregister(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
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
    .command(
        `pauseContainerChain`,
        "Pause a container-chain from collating, without modifying its boot nodes nor its parachain config",
        (yargs) => {
            return yargs
                .options({
                    ...NETWORK_YARGS_OPTIONS,
                    "account-priv-key": {
                        type: "string",
                        demandOption: false,
                        alias: "account",
                    },
                    "para-id": {
                        describe: "Container chain para id",
                        type: "number",
                    },
                })
                .demandOption(["para-id", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                const privKey = argv["account-priv-key"];
                const account = keyring.addFromUri(privKey);

                let tx = api.tx.registrar.pauseContainerChain(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
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
