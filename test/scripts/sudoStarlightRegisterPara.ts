import { Keyring } from "@polkadot/api";
import fs from "node:fs/promises";
import jsonBg from "json-bigint";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { chainSpecToContainerChainGenesisData } from "../util/genesis_data";
import { NETWORK_YARGS_OPTIONS, getApiFor } from "./utils/network";
import type { SubmittableExtrinsic } from "@polkadot/api/types";
import type { U64 } from "@polkadot/types";
import assert from "node:assert";
const JSONbig = jsonBg({ useNativeBigInt: true });

yargs(hideBin(process.argv))
    .usage("Usage: $0")
    .version("1.0.0")
    .command(
        "register",
        "Registers a parachain, adds bootnodes, and marks the validation code as trusted. Does not mark the para as validForCollating",
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
                    genesisState: {
                        describe: "Input path of genesis state file",
                        type: "string",
                    },
                    parathread: {
                        describe: "Set the chain as a parathread instead of a parachain",
                        type: "boolean",
                        default: false,
                    },
                })
                .demandOption(["chain", "genesis-state", "account-priv-key"]);
        },
        async (argv) => {
            const api = await getApiFor(argv);
            const keyring = new Keyring({ type: "sr25519" });

            try {
                process.stdout.write(`Reading chainSpec from: ${argv.chain}\n`);
                assert(argv.chain, "chain is required");
                assert(typeof argv.genesisState === "string", "genesisState is required");
                const rawSpec = JSONbig.parse(await fs.readFile(argv.chain, "utf8"));
                const genesisCode = rawSpec.genesis.raw.top["0x3a636f6465"];
                const headData = await fs.readFile(argv.genesisState, "utf8");

                const privKey = argv["account-priv-key"];
                assert(privKey, "account-priv-key is required");
                const account = keyring.addFromUri(privKey);

                const containerChainGenesisData = chainSpecToContainerChainGenesisData(api, rawSpec);
                const txs: SubmittableExtrinsic<"promise">[] = [];
                let tx1: any;
                if (argv.parathread) {
                    const slotFreq = api.createType("TpTraitsSlotFrequency", {
                        min: 1,
                        max: 1,
                    });
                    tx1 = api.tx.containerRegistrar.registerParathread(
                        rawSpec.para_id,
                        slotFreq,
                        containerChainGenesisData,
                        headData
                    );
                } else {
                    tx1 = api.tx.containerRegistrar.register(rawSpec.para_id, containerChainGenesisData, headData);
                }
                txs.push(tx1);
                if (rawSpec.bootNodes?.length) {
                    let profileId = ((await api.query.dataPreservers.nextProfileId()) as U64).toNumber();
                    for (const bootnode of rawSpec.bootNodes) {
                        const profileTx = api.tx.dataPreservers.createProfile({
                            url: bootnode,
                            paraIds: "AnyParaId",
                            mode: "Bootnode",
                            assignmentRequest: "Free",
                        });
                        txs.push(profileTx);

                        const tx2 = api.tx.dataPreservers.forceStartAssignment(profileId++, rawSpec.para_id, "Free");
                        const tx2s = api.tx.sudo.sudo(tx2);
                        txs.push(tx2s);
                    }
                }
                // In Starlight we must wait 2 session before calling markValidForCollating, because the para needs to be
                // onboarded in the relay registrar first.
                // And before being allowed to do that, we must mark the validationCode as trusted
                const tx3 = api.tx.paras.addTrustedValidationCode(genesisCode);
                const tx3s = api.tx.sudo.sudo(tx3);
                txs.push(tx3s);

                if (txs.length === 2) {
                    process.stdout.write("Sending register transaction (register + addTrustedValidationCode)... ");
                } else {
                    process.stdout.write(
                        "Sending register transaction (register + createProfile + startAssignment + addTrustedValidationCode)... "
                    );
                }
                const txBatch = api.tx.utility.batchAll(txs);
                const txHash = await txBatch.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write("Done ✅\n");
            } finally {
                await api.disconnect();
            }
        }
    )
    .command(
        "markValidForCollating",
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

                let tx = api.tx.containerRegistrar.markValidForCollating(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
                process.stdout.write("Sending transaction... ");
                const txHash = await tx.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write("Done ✅\n");
            } finally {
                await api.disconnect();
            }
        }
    )
    .command(
        "setBootNodes",
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

                let bootnodes: (string | number)[] = [];
                if (!argv.bootnode) {
                    argv.bootnode = [];
                }
                bootnodes = [...bootnodes, ...argv.bootnode];

                const txs: SubmittableExtrinsic<"promise">[] = [];

                let profileId = ((await api.query.dataPreservers.nextProfileId()) as U64).toNumber();
                for (const bootnode of bootnodes) {
                    const profileTx = api.tx.dataPreservers.createProfile({
                        url: bootnode,
                        paraIds: "AnyParaId",
                        mode: "Bootnode",
                        assignmentRequest: "Free",
                    });
                    txs.push(profileTx);

                    const tx2 = api.tx.dataPreservers.forceStartAssignment(profileId++, argv.paraId, "Free");
                    const tx2s = api.tx.sudo.sudo(tx2);
                    txs.push(tx2s);
                }

                if (argv.markValidForCollating) {
                    // Check if not already valid, and only in that case call markValidForCollating
                    const notValidParas = (await api.query.containerRegistrar.pendingVerification()) as any;
                    if (notValidParas.toJSON().includes(argv.paraId)) {
                        process.stdout.write("Will set container chain valid for collating\n");
                        const tx2 = api.tx.containerRegistrar.markValidForCollating(argv.paraId);
                        const tx2s = api.tx.sudo.sudo(tx2);
                        txs.push(tx2s);
                    } else {
                        // ParaId already valid, or not registered at all
                        process.stdout.write("Not setting container chain valid for collating\n");
                    }
                }
                const batchTx = api.tx.utility.batchAll(txs);
                process.stdout.write("Sending transaction... ");
                const txHash = await batchTx.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write("Done ✅\n");
            } finally {
                await api.disconnect();
            }
        }
    )
    .command(
        "deregister",
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

                let tx = api.tx.containerRegistrar.deregister(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
                process.stdout.write("Sending transaction... ");
                const txHash = await tx.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write("Done ✅\n");
            } finally {
                await api.disconnect();
            }
        }
    )
    .command(
        "pauseContainerChain",
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

                let tx = api.tx.containerRegistrar.pauseContainerChain(argv.paraId);
                tx = api.tx.sudo.sudo(tx);
                process.stdout.write("Sending transaction... ");
                const txHash = await tx.signAndSend(account);
                process.stdout.write(`${txHash.toHex()}\n`);
                // TODO: this will always print Done, even if the extrinsic has failed
                process.stdout.write("Done ✅\n");
            } finally {
                await api.disconnect();
            }
        }
    )
    .parse();
