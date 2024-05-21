import "@polkadot/api-augment";
import { describeSuite, expect } from "@moonwall/cli";
import { FrameSupportDispatchDispatchInfo } from "@polkadot/types/lookup";
import { BN } from "@polkadot/util";

describeSuite({
    id: "DT0402",
    title: "On set latest author data weight check",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "E01",
            title: "Weight should be match expected",
            test: async function () {
                const expectedRefTime = new BN(1_245_284_212);
                const expectedProofSize = new BN(15_236);

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
                expect(refTime.gte(expectedRefTime.divn(1.1)) && refTime.lte(expectedRefTime.muln(1.1))).to.be.true;
                expect(proofSize.gte(expectedProofSize.divn(1.1)) && proofSize.lte(expectedProofSize.muln(1.1))).to.be
                    .true;
            },
        });
    },
});
