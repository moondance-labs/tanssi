import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { hasEnoughCredits } from "utils";
import { isLightRuntime } from "../../utils/runtime.ts";

describeSuite({
    id: "S03",
    title: "Check services payment consistency",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion: number;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;
        let blocksPerSession: bigint;
        let chain: any;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
            chain = api.consts.system.version.specName.toString();
            blocksPerSession = chain === "dancebox" || chain === "dancelight" ? 600n : 50n;
        });

        it({
            id: "C01",
            title: "All scheduled parachains should be able to pay for at least 1 session worth of blocks",
            test: async () => {
                if (runtimeVersion < 500) {
                    return;
                }
                const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                const blockToCheck = isLightRuntime(api)
                    ? (await api.query.babe.epochStart()).toJSON()[1]
                    : Math.trunc(currentBlock / Number(blocksPerSession)) * Number(blocksPerSession);
                const apiBeforeLatestNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));

                // If they have collators scheduled, they should have at least enough money to pay
                let pending = isLightRuntime(api)
                    ? await api.query.tanssiCollatorAssignment.pendingCollatorContainerChain()
                    : await api.query.collatorAssignment.pendingCollatorContainerChain();

                if (pending.isNone) {
                    pending = isLightRuntime(api)
                        ? await api.query.tanssiCollatorAssignment.collatorContainerChain()
                        : await api.query.collatorAssignment.collatorContainerChain();
                }

                const current = isLightRuntime(api)
                    ? await api.query.tanssiCollatorAssignment.collatorContainerChain()
                    : await api.query.collatorAssignment.collatorContainerChain();

                if (pending.containerChains !== undefined) {
                    for (const container of Object.keys(pending.toJSON().containerChains)) {
                        // if not currently assigned, then one session
                        // if currently assigned, then 2
                        let sessionRequirements: bigint;

                        if (current.toJSON().containerChains[container.toString()]?.length === 0) {
                            sessionRequirements = 1n;
                        } else {
                            sessionRequirements = 2n;
                        }
                        expect(
                            await hasEnoughCredits(
                                apiBeforeLatestNewSession,
                                container,
                                blocksPerSession,
                                1n,
                                sessionRequirements,
                                costPerSession,
                                costPerBlock
                            ),
                            `Container chain ${container} was assigned collators without having a way to pay for it`
                        ).toBe(true);
                    }
                }
            },
        });
    },
});
