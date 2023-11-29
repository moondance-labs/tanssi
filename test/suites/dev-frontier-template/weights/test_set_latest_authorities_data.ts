import "@polkadot/api-augment";
import { describeSuite, expect } from "@moonwall/cli";
import { FrameSupportDispatchDispatchInfo } from "@polkadot/types/lookup";

describeSuite({
    id: "DF1001",
    title: "On set latest authorities data weight check",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "T01",
            title: "Weight should be match expected",
            test: async function () {
                await context.createBlock();

                const block = await context.polkadotJs().rpc.chain.getBlock();
                const allRecords = await context.polkadotJs().query.system.events();

                // Get index of authoritiesNoting.setLatestAuthoritiesData
                const setAuthorIntrinsicIndex = block.block.extrinsics.reduce(
                    (filtered, extrinsic, idx) =>
                        filtered.concat(
                            extrinsic.method.section === "authoritiesNoting" &&
                                extrinsic.method.method === "setLatestAuthoritiesData"
                                ? idx
                                : []
                        ),
                    []
                );

                expect(setAuthorIntrinsicIndex.length).toBe(1);

                const events = allRecords.filter(
                    ({ phase }) => phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(setAuthorIntrinsicIndex[0])
                );

                const usedWeight = (events.at(-1).event.data[0] as unknown as FrameSupportDispatchDispatchInfo).weight;

                expect(usedWeight.refTime.toBigInt()).toEqual(456_622_000n);
                expect(usedWeight.proofSize.toBigInt()).toEqual(6_488n);
            },
        });
    },
});
