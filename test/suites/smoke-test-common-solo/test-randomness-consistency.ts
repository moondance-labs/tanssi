import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import type { ApiPromise } from "@polkadot/api";
import { fetchRandomnessEventTanssiSolo } from "utils";

describeSuite({
    id: "SMOK13",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion: number;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Randomness storage is empty because on-finalize cleans it, unless on session change boundaries",
            test: async ({ skip }) => {
                if (runtimeVersion < 1400) {
                    // We know there exists a bug prior to 1400. so let's skip this
                    skip();
                }
                // After runtime upgrade, randomness in storage is always empty, this test can be removed
                const randomness = await api.query.tanssiCollatorAssignment.randomness();
                expect(randomness.isEmpty).to.be.true;
                return;
            },
        });

        it({
            id: "C02",
            title: "Rotation happened at previous session boundary",
            test: async ({ skip }) => {
                if (runtimeVersion < 1400) {
                    // We know there exists a bug prior to 1400. so let's skip this
                    skip();
                }

                const blockBabeEpochStart = (await api.query.babe.epochStart())[1].toNumber();
                const apiAtNewSession = await api.at(await api.rpc.chain.getBlockHash(blockBabeEpochStart));

                const blockToCheck = blockBabeEpochStart - 1;
                const apiBeforeNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));
                // After, the randomness gets cleaned
                const randomnessAfterSession = await apiAtNewSession.query.tanssiCollatorAssignment.randomness();
                expect(randomnessAfterSession.isEmpty).to.be.true;

                // The rotation event should have kicked in, if enabled
                const events = await apiAtNewSession.query.system.events();
                const randomnessEvent = fetchRandomnessEventTanssiSolo(events);
                const session = await apiAtNewSession.query.session.currentIndex();

                expect(randomnessEvent.randomSeed.toHex()).to.not.be.equal(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );
                expect(randomnessEvent.targetSession.toNumber()).to.be.equal(session.toNumber() + 1);
                const configuration = await apiAtNewSession.query.collatorConfiguration.activeConfig();
                if (
                    configuration.fullRotationPeriod === 0 ||
                    randomnessEvent.targetSession.toNumber() % configuration.fullRotationPeriod !== 0
                ) {
                    expect(randomnessEvent.fullRotation.toHuman()).to.be.false;
                } else {
                    expect(randomnessEvent.fullRotation.toHuman()).to.be.true;
                }
            },
        });
    },
});
