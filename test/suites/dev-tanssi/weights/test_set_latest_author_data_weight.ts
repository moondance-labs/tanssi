import "@polkadot/api-augment";
import { describeSuite, expect } from "@moonwall/cli";
import { FrameSupportDispatchDispatchInfo } from "@polkadot/types/lookup";

describeSuite({
    id: "DT0702",
    title: "On set latest author data weight check",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "E01",
            title: "Weight should be match expected",
            test: async function () {
                await context.createBlock();

                const block = await context.polkadotJs().rpc.chain.getBlock();
                const allRecords = await context.polkadotJs().query.system.events();

                // Get index of authorNoting.setLatestAuthorData
                const setAuthorIntrinsicIndex = block.block.extrinsics.reduce(
                    (filtered, extrinsic, idx) =>
                        filtered.concat(
                            extrinsic.method.section === "authorNoting" &&
                                extrinsic.method.method === "setLatestAuthorData"
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

                expect(usedWeight.refTime.toBigInt()).toEqual(1_064_263_621n);
                expect(usedWeight.proofSize.toBigInt()).toEqual(6_745n);
            },
        });
    },
});
