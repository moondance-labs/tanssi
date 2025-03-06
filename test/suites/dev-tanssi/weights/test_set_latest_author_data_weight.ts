import "@tanssi/api-augment";

import { describeSuite, expect } from "@moonwall/cli";
import type { FrameSupportDispatchDispatchInfo } from "@polkadot/types/lookup";
import { BN } from "@polkadot/util";

describeSuite({
    id: "DEV0902",
    title: "On set latest author data weight check",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "E01",
            title: "Weight should be match expected",
            test: async () => {
                // TODO: is it expected that this test breaks, just copy the new weights
                const expectedRefTime = { avg: new BN(786288068) };
                expectedRefTime.min = expectedRefTime.avg.divn(1.1);
                expectedRefTime.max = expectedRefTime.avg.muln(1.1);
                const expectedProofSize = { avg: new BN(5507) };
                expectedProofSize.min = expectedProofSize.avg.divn(1.1);
                expectedProofSize.max = expectedProofSize.avg.muln(1.1);

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
                const refTime = usedWeight.refTime.toBn();
                const proofSize = usedWeight.proofSize.toBn();

                // Allow 10% variance
                expect(
                    refTime,
                    `refTime is ${refTime} but expected a value between ${expectedRefTime.min} and ${expectedRefTime.max}`
                ).to.satisfy((val) => val >= expectedRefTime.min && val <= expectedRefTime.max);

                expect(
                    proofSize,
                    `proofSize is ${proofSize} but expected a value between ${expectedProofSize.min} and ${expectedProofSize.max}`
                ).to.satisfy((val) => val >= expectedProofSize.min && val <= expectedProofSize.max);
            },
        });
    },
});
