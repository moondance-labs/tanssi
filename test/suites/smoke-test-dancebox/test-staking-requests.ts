import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getBlockArray } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { GenericExtrinsic } from "@polkadot/types";
import type { FrameSystemEventRecord } from "@polkadot/types/lookup";
import type { AnyTuple } from "@polkadot/types/types";
import Bottleneck from "bottleneck";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);
const hours = (timePeriod / (1000 * 60 * 60)).toFixed(2);

type BlockFilteredRecord = {
    blockNum: number;
    extrinsics: GenericExtrinsic<AnyTuple>[];
    events: FrameSystemEventRecord[];
    session;
    pendingOperations;
};

describeSuite({
    id: "SM06",
    title: `Pending Operations in the last ${hours} should be correctly executed`,
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let blockData: BlockFilteredRecord[];

        beforeAll(async () => {
            api = context.polkadotJs();
            const blockNumArray = await getBlockArray(api, timePeriod);
            log(`Collecting ${hours} hours worth of blocks`);

            const getBlockData = async (blockNum: number) => {
                const blockHash = await api.rpc.chain.getBlockHash(blockNum);
                const blockHashBefore = await api.rpc.chain.getBlockHash(blockNum - 1);

                const signedBlock = await api.rpc.chain.getBlock(blockHash);
                const apiAt = await api.at(blockHash);
                const apiAtBefore = await api.at(blockHashBefore);

                // For all extrinsics that are pending operation executions, we fetch the delegators
                // pending operations in the previous block
                const operatorPendingOps = {};
                for (var extrinsic of signedBlock.block.extrinsics) {
                    const isValidExtrinsic =
                        extrinsic.method.section.toString() === "pooledStaking" &&
                        extrinsic.method.method.toString() === "executePendingOperations";
                    if (isValidExtrinsic) {
                        const injectedOps = extrinsic.args[0].toHuman();
                        for (let i = 0; i < injectedOps.length; i++) {
                            const operation = injectedOps[i];
                            const pendingOps = (
                                await apiAtBefore.query.pooledStaking.pendingOperations.entries(operation.delegator)
                            ).map(([key, _]) => key.toHuman());

                            operatorPendingOps[operation.delegator] = pendingOps;
                        }
                    }
                }

                // We record the session too as we need to compare it against the session at which the request was injected
                return {
                    blockNum: blockNum,
                    extrinsics: signedBlock.block.extrinsics,
                    events: await apiAt.query.system.events(),
                    session: await apiAt.query.session.currentIndex(),
                    pendingOperations: operatorPendingOps,
                };
            };
            const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });
            blockData = await Promise.all(blockNumArray.map((num) => limiter.schedule(() => getBlockData(num))));
        }, timeout);

        it({
            id: "C01",
            title: "If someone executes a pending request, it should have been generated at least 2 sessions ago",
            test: async () => {
                const filteredEvents = blockData
                    .map(({ blockNum, events, session, pendingOperations }) => {
                        const matchedEvents = events
                            .filter(({ event }) => api.events.system.ExtrinsicSuccess.is(event))
                            .filter(({ event }) => {
                                const info = event.data[0] as DispatchInfo;
                                return info.class.isNormal && info.paysFee.isYes;
                            });
                        return { blockNum, matchedEvents, session, pendingOperations };
                    })
                    .filter(({ matchedEvents }) => matchedEvents.length > 0);

                // This function evaluates whether there exists at least one pending op that has been executed
                const isValidPolledStakingOp = async (blockNum: number, index: number, session, pendingOperations) => {
                    const extrinsic = blockData.find((a) => a.blockNum === blockNum)!.extrinsics[index];
                    const isValidExtrinsic =
                        extrinsic.method.section.toString() === "pooledStaking" &&
                        extrinsic.method.method.toString() === "executePendingOperations";

                    if (isValidExtrinsic) {
                        const injectedOps = extrinsic.args[0].toHuman();

                        for (let i = 0; i < injectedOps.length; i++) {
                            const operation = injectedOps[i];
                            const pendingBefore = pendingOperations[operation.delegator].map(([_, op]) => op);

                            const time = Number.parseInt(
                                operation.operation[Object.keys(operation.operation)[0]].at.replace(",", "")
                            );

                            const filtered = pendingBefore.filter(
                                (op) => JSON.stringify(op) === JSON.stringify(operation.operation)
                            );

                            if (filtered.length > 0 && time > session.toNumber() - 2) {
                                return false;
                            }
                        }
                    }

                    return true;
                };

                const failures = filteredEvents
                    .map(({ blockNum, matchedEvents, session, pendingOperations }) => {
                        const pooledStakingEvents = matchedEvents.filter(
                            (a) =>
                                !isValidPolledStakingOp(
                                    blockNum,
                                    a.phase.asApplyExtrinsic.toNumber(),
                                    session,
                                    pendingOperations
                                )
                        );
                        return { blockNum, matchedEvents: pooledStakingEvents };
                    })
                    .filter((a) => a.matchedEvents.length > 0);

                failures.forEach(({ blockNum, matchedEvents }) => {
                    matchedEvents.forEach((a: any) => {
                        log(
                            `Pooled Staking at block #${blockNum} extrinsic #${a.phase.asApplyExtrinsic.toNumber()}` +
                                ": executed before elapsed time"
                        );
                    });
                });

                expect(
                    failures.length,
                    `Please investigate blocks ${failures.map((a) => a.blockNum).join(`, `)}; pays_fee:yes  `
                ).to.equal(0);
            },
        });
    },
});
