import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getTreasuryAddress } from "../../utils";
import { filterAndApply } from "@moonwall/util";
import type { EventRecord } from "@polkadot/types/interfaces";
import type { FrameSystemEventRecord } from "@polkadot/types/lookup";

const RUNTIME_VERSION_THRESHOLD = 1300;
let BLOCKS_AMOUNT_TO_CHECK = 100;
// For debug purposes only, specify block here to check it
const BLOCK_NUMBER_TO_DEBUG = undefined;

describeSuite({
    id: "S07",
    title: "Check if fees go to treasury for all runtimes",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Check if fees go to treasury",
            test: async () => {
                let currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                const treasuryAddress = getTreasuryAddress(api);

                if (BLOCK_NUMBER_TO_DEBUG !== undefined) {
                    BLOCKS_AMOUNT_TO_CHECK = 1;
                    currentBlock = BLOCK_NUMBER_TO_DEBUG + 1;
                }

                for (let i = 1; i <= BLOCKS_AMOUNT_TO_CHECK; i++) {
                    const blockNumber = currentBlock - i;

                    // Track if any assertions executed for the block (to avoid false positives)
                    let isAsserted = false;

                    process.stdout.write(`\rProcessing block [${blockNumber}]: ${i}/${BLOCKS_AMOUNT_TO_CHECK}`);

                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const block = await api.rpc.chain.getBlock(blockHash);
                    const extrinsics = block.block.extrinsics;

                    if (extrinsics.length === 0) {
                        log(`No extrinsics for block ${blockNumber}, skipping...`);
                        continue;
                    }
                    const apiAtBlock = await api.at(blockHash);
                    const specVersion = apiAtBlock.consts.system.version.specVersion.toNumber();

                    if (specVersion < RUNTIME_VERSION_THRESHOLD) {
                        log(
                            `Skip tests for runtimes before ${RUNTIME_VERSION_THRESHOLD}. Current runtime: ${specVersion}`
                        );
                        return;
                    }

                    const allRecords = await apiAtBlock.query.system.events();
                    const extrinsicIndexToEventsMap = new Map<string, FrameSystemEventRecord[]>();

                    for (const eventRecord of allRecords) {
                        if (!eventRecord.phase.isApplyExtrinsic) {
                            continue;
                        }

                        const extrinsicIndex = eventRecord.phase.asApplyExtrinsic.toString();
                        if (!extrinsicIndexToEventsMap.has(extrinsicIndex)) {
                            extrinsicIndexToEventsMap.set(extrinsicIndex, []);
                        }

                        const events = extrinsicIndexToEventsMap.get(extrinsicIndex);
                        events.push(eventRecord);

                        extrinsicIndexToEventsMap.set(extrinsicIndex, events);
                    }

                    for (const [index, extrinsic] of extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const feePaidEvents = filterAndApply(
                            events,
                            "transactionPayment",
                            ["TransactionFeePaid"],
                            ({ event }: EventRecord) =>
                                event.data.toHuman() as unknown as { who: string; actualFee: string; tip: string }
                        );

                        // Get all balances deposit events for the current extrinsic
                        let balancesDepositEvents = filterAndApply(
                            events,
                            "balances",
                            ["Deposit"],
                            ({ event }: EventRecord) =>
                                event.data.toHuman() as unknown as { who: string; amount: string }
                        );

                        // Filter deposits only for treasury and check if the fee matches
                        balancesDepositEvents = balancesDepositEvents.filter(
                            (event) => event.who === treasuryAddress && event.amount === feePaidEvents[0].actualFee
                        );

                        // We skip extrinsics with zero fees
                        if (feePaidEvents[0].actualFee === "0") {
                            continue;
                        }

                        expect(
                            feePaidEvents.length,
                            `[Block: ${blockNumber}, Hash: ${blockHash}] Expect the amount of \`transactionPayment.TransactionFeePaid\` equals 1`
                        ).toEqual(1);

                        expect(
                            balancesDepositEvents.length,
                            `[Block: ${blockNumber}, Hash: ${blockHash}] Expect the amount of \`balances.Deposit\` for treasury address with exact fee value equals 1`
                        ).toEqual(1);

                        isAsserted = true;
                    }

                    if (!isAsserted) {
                        log("No assertions executed in this block, skipping...");
                    }
                }
            },
        });
    },
});
