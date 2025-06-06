import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getBlockArray } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { GenericExtrinsic } from "@polkadot/types";
import type { DigestItem } from "@polkadot/types/interfaces";
import type {
    FrameSystemEventRecord,
    PolkadotPrimitivesV8InherentData,
    PolkadotRuntimeParachainsConfigurationHostConfiguration,
} from "@polkadot/types/lookup";
import type { AnyTuple } from "@polkadot/types/types";
import Bottleneck from "bottleneck";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);
const hours = (timePeriod / (1000 * 60 * 60)).toFixed(2);

type BlockFilteredRecord = {
    blockNum: number;
    extrinsics: GenericExtrinsic<AnyTuple>[];
    events: FrameSystemEventRecord[];
    logs: DigestItem[];
    config: PolkadotRuntimeParachainsConfigurationHostConfiguration;
    paraInherents: GenericExtrinsic[];
};

interface CollatorAssignment {
    orchestratorChain: string[];
    containerChains: { [paraId: number]: string[] };
}

describeSuite({
    id: "SMOK07",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let blockData: BlockFilteredRecord[];
        const blockNumberMap: Map<string, number> = new Map();
        const collatorsMap: Map<string, CollatorAssignment> = new Map();

        beforeAll(async () => {
            api = context.polkadotJs();

            const blockNumArray = await getBlockArray(api, timePeriod);
            log(`Collecting ${hours} hours worth of authors`);

            const getBlockData = async (blockNum: number) => {
                const blockHash = await api.rpc.chain.getBlockHash(blockNum);
                const signedBlock = await api.rpc.chain.getBlock(blockHash);
                const apiAt = await api.at(blockHash);
                const config =
                    (await apiAt.query.configuration.activeConfig()) as unknown as PolkadotRuntimeParachainsConfigurationHostConfiguration;
                const extrinsics = signedBlock.block.extrinsics;

                const paraInherents = extrinsics.filter(
                    ({ method: { method, section } }) => section === "paraInherent" && method === "enter"
                );

                const {
                    method: { args },
                } = paraInherents[0];

                const { backedCandidates } = args[0] as PolkadotPrimitivesV8InherentData;

                for (const cand of backedCandidates) {
                    const relayParent = cand.candidate.descriptor.relayParent.toHex();

                    if (!blockNumberMap.has(relayParent)) {
                        const apiAtP = await api.at(relayParent);
                        const parentBlockNumber = await apiAtP.query.system.number();

                        blockNumberMap.set(relayParent, parentBlockNumber.toNumber());
                    }

                    if (!collatorsMap.has(relayParent)) {
                        const apiAtP = await api.at(relayParent);
                        const collators = await apiAtP.query.tanssiCollatorAssignment.collatorContainerChain();
                        const containerChainsMap = [...collators.containerChains.entries()].reduce(
                            (acc, [key, value]) => {
                                acc[key.toNumber()] = value.map((a) => a.toHuman());
                                return acc;
                            },
                            {} as { [paraId: number]: string[] }
                        );

                        const blob: CollatorAssignment = {
                            orchestratorChain: collators.orchestratorChain.map((a) => a.toHuman()),
                            containerChains: containerChainsMap,
                        };
                        collatorsMap.set(relayParent, blob);
                    }
                }

                return {
                    blockNum: blockNum,
                    extrinsics,
                    events: await apiAt.query.system.events(),
                    logs: signedBlock.block.header.digest.logs,
                    config,
                    paraInherents,
                };
            };
            const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });
            blockData = await Promise.all(blockNumArray.map((num) => limiter.schedule(() => getBlockData(num))));
        }, timeout);

        it({
            id: "C01",
            title: "Included paras valid",
            test: async () => {
                blockData.map(({ blockNum, config, paraInherents }) => {
                    // Should have exactly 1 paraInherent
                    expect(paraInherents.length, `Block ${blockNum}: missing paraInherent in block`).toBeGreaterThan(0);
                    expect(paraInherents.length, `Block ${blockNum}: duplicate paraInherent in block`).toBeLessThan(2);

                    const {
                        method: { args },
                    } = paraInherents[0];

                    const { backedCandidates } = args[0] as PolkadotPrimitivesV8InherentData;

                    const numBackedCandidates = backedCandidates.length;

                    // assert that numBackedCandidates <= numCores
                    const numCores = config.schedulerParams.numCores.toNumber();
                    expect(
                        numBackedCandidates,
                        `Block #${blockNum}: backed more candidates than cores available: ${numBackedCandidates} vs cores ${numCores}`
                    ).to.be.lessThanOrEqual(numCores);

                    // Assert that each backed candidate:
                    // * has relayParent be at most allowedAncestryLen backwards
                    // * had collators assigned to it at block "relayParent"
                    const allowedAncestryLen = config.asyncBackingParams.allowedAncestryLen.toNumber();
                    for (const cand of backedCandidates) {
                        const paraId = cand.candidate.descriptor.paraId.toNumber();
                        const relayParent = cand.candidate.descriptor.relayParent.toHex();
                        const parentBlockNumber = blockNumberMap.get(relayParent);

                        // allowedAncestryLen = 0 means that we only allow building on top of the parent block
                        // allowedAncestryLen = 1 means that we allow 2 different parent blocks,
                        // so parent + 2 >= current
                        // In general, parent + allowedAncestryLen + 1 >= current
                        expect(
                            parentBlockNumber + allowedAncestryLen + 1,
                            `Block #${blockNum}: backed candidate for para id ${paraId} has too old relayParent: ${parentBlockNumber} vs current ${blockNum}`
                        ).to.be.greaterThanOrEqual(blockNum);

                        const collators = collatorsMap.get(relayParent);
                        expect(
                            collators.containerChains[paraId],
                            `Block #${blockNum}: Found backed candidate for para id ${paraId}, but that para id has no collators assigned. Collator assignment: {containerChains: ${collators.containerChains} , orchestratorChain: ${collators.orchestratorChain}}`
                        ).toBeTruthy();
                    }
                });
            },
        });
    },
});
