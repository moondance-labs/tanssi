import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { TpTraitsSlotFrequency } from "@polkadot/types/lookup";
import fs, { stat } from "node:fs/promises";
import { generateEmptyGenesisData, createCollatorKeyToNameMap, signAndSendAndInclude, waitSessions } from "utils";

describeSuite({
    id: "ZOMBIETAN01",
    title: "Zombie Tanssi Rotation Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let allCollators: string[];
        let collatorName: Record<string, string>;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

            const paraNetwork = paraApi.consts.system.version.specName.toString();
            const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
            expect(paraNetwork, "Para API incorrect").to.contain("dancebox");
            expect(paraId1000, "Para API incorrect").to.be.equal("1000");

            // Initialize list of all collators, this should match the names from build-spec.sh script
            allCollators = ["Collator-01", "Collator-02"];
            // Initialize reverse map of collator key to collator name
            collatorName = createCollatorKeyToNameMap(paraApi, allCollators);
            console.log(collatorName);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async () => {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Disable full_rotation, set to 1 collator",
            timeout: 120000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                const tx1 = paraApi.tx.configuration.setMinOrchestratorCollators(1);
                const tx2 = paraApi.tx.configuration.setMaxOrchestratorCollators(1);
                const tx3 = paraApi.tx.configuration.setFullRotationPeriod(0);
                const tx123 = paraApi.tx.utility.batchAll([tx1, tx2, tx3]);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx123), alice);
            },
        });

        it({
            id: "T03",
            title: "Register empty wasm as parathread 2000",
            timeout: 240000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                const txs2000 = await registerEmptyParathread(paraApi, alice.address, 2000);
                const txs = paraApi.tx.utility.batchAll([...txs2000]);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(txs), alice);
            },
        });

        it({
            id: "T04",
            title: "Wait for parathread 2000 to be assigned collators",
            timeout: 600000,
            test: async () => {
                await waitSessions(context, paraApi, 2, async () => {
                    const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                    const containerChainCollators = (
                        await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                    ).toJSON().containerChains;
                    // Stop waiting when parathread has been assigned collators
                    return containerChainCollators[2000] !== undefined;
                });

                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains;
                expect(
                    containerChainCollators[2000] !== undefined,
                    "Failed to register parathread: no collators assigned"
                ).to.be.true;
            },
        });

        it({
            id: "T05",
            title: "Check logs, collator failed to start",
            test: async () => {
                // We registered an empty wasm as a parathread. Collator should fail to start, but never panic.
                const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain()).toJSON();
                const oldC2000 = collatorName[assignment.containerChains[2000][0]];
                const logFilePath = `${getTmpZombiePath()}/${oldC2000}.log`;
                await checkLogs(logFilePath, [
                    "[Orchestrator] Detected assignment for container chain 2000",
                    "[Orchestrator] Loaded chain spec for container chain 2000",
                    "[Orchestrator] This is a syncing container chain, using random ports",
                    "[Orchestrator] Container chain sync mode: Warp",
                    "[Orchestrator] Failed to start container chain 2000: Failed to get runtime version: Runtime missing from initial storage, could not read state version.",
                ]);
            },
        });

        it({
            id: "T06",
            title: "Check logs, collator did not panic",
            test: async () => {
                const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain()).toJSON();
                const oldC2000 = collatorName[assignment.containerChains[2000][0]];
                const logFilePath = `${getTmpZombiePath()}/${oldC2000}.log`;
                // Best effort, if anything else panics this test will breaks
                await assertLogsDoNotContain(logFilePath, "panic");
            },
        });

        it({
            id: "T06",
            title: "Check logs, collator is still running",
            test: async () => {
                const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain()).toJSON();
                const oldC2000 = collatorName[assignment.containerChains[2000][0]];
                const logFilePath = `${getTmpZombiePath()}/${oldC2000}.log`;
                await waitForNewLogs(logFilePath);
            },
        });
    },
});

async function registerEmptyParathread(api: ApiPromise, manager: any, paraIdString: string) {
    const parathread = true;
    const paraId = Number.parseInt(paraIdString);
    const containerChainGenesisData = generateEmptyGenesisData(api);

    const txs = [];
    let tx1: any;
    if (parathread) {
        const slotFreq = api.createType<TpTraitsSlotFrequency>("TpTraitsSlotFrequency", {
            min: 1,
            max: 1,
        });
        tx1 = api.tx.registrar.registerParathread(paraId, slotFreq, containerChainGenesisData, null);
    } else {
        tx1 = api.tx.registrar.register(paraId, containerChainGenesisData, null);
    }
    txs.push(
        api.tx.utility.dispatchAs(
            {
                system: { Signed: manager },
            } as any,
            tx1
        )
    );

    const profileId = await api.query.dataPreservers.nextProfileId();
    txs.push(
        api.tx.dataPreservers.forceCreateProfile(
            {
                url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                paraIds: "AnyParaId",
                mode: "Bootnode",
                assignmentRequest: "Free",
            },
            manager
        )
    );
    txs.push(api.tx.dataPreservers.forceStartAssignment(profileId, paraId, "Free"));
    txs.push(api.tx.registrar.markValidForCollating(paraId));

    return txs;
}

const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
async function checkLogs(logFilePath: string, logs: string[]): Promise<void> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    let logIndex = 0;
    let lastFoundLogIndex = 0;

    for (let i = 0; i < lines.length; i++) {
        if (logIndex < logs.length && lines[i].includes(logs[logIndex])) {
            logIndex++;
            lastFoundLogIndex = i;
        }

        if (logIndex === logs.length) {
            break;
        }
    }

    if (logIndex !== logs.length) {
        // In case of missing logs, show some context around the last found log
        const contextSize = 3;
        const contextStart = Math.max(0, lastFoundLogIndex - contextSize);
        const contextEnd = Math.min(lines.length - 1, lastFoundLogIndex + contextSize);
        const contextLines = lines.slice(contextStart, contextEnd + 1);
        const contextStr = contextLines.join("\n");

        expect.fail(
            `Not all logs were found in the correct order. Missing log: '${logs[logIndex]}'\nContext around the last found log:\n${contextStr}`
        );
    }
}

// Checks that the specified log does not appear in the log file.
// If the log appears, it provides context around the first occurrence using expect.fail.
async function assertLogsDoNotContain(logFilePath: string, forbiddenLog: string): Promise<void> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    for (let i = 0; i < lines.length; i++) {
        if (lines[i].includes(forbiddenLog)) {
            const contextSize = 3;
            const contextStart = Math.max(0, i - contextSize);
            const contextEnd = Math.min(lines.length - 1, i + contextSize);
            const contextLines = lines.slice(contextStart, contextEnd + 1);
            const contextStr = contextLines.join("\n");

            expect.fail(
                `The log file should not contain the log: '${forbiddenLog}'\nContext around the occurrence:\n${contextStr}`
            );
            return; // Exit after the first match to provide immediate feedback and efficiency
        }
    }
}

// Wait until log file size changes. This indicates that the node is still alive.
async function waitForNewLogs(logFilePath: string): Promise<void> {
    const initialSize = (await stat(logFilePath)).size;

    // eslint-disable-next-line no-constant-condition
    while (true) {
        const currentSize = (await stat(logFilePath)).size;
        if (currentSize > initialSize) {
            return;
        }

        await sleep(200);
    }
}
