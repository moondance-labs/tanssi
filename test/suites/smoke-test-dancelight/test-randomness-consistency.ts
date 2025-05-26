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
            test: async () => {
                // After runtime upgrade, randomness in storage is always empty, this test can be removed
                if (runtimeVersion >= 1400) {
                    const randomness = await api.query.tanssiCollatorAssignment.randomness();
                    expect(randomness.isEmpty).to.be.true;
                    return;
                }

                const randomness = await api.query.tanssiCollatorAssignment.randomness();

                // take the most recent
                const blockBabeEpochStart = (await api.query.babe.epochStart())[1].toNumber();
                const currentSlot = await api.query.babe.currentSlot();

                const apiAtSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockBabeEpochStart));

                const slotAtEpochStart = await apiAtSessionChange.query.babe.currentSlot();
                const sessionLength = await api.consts.babe.epochDuration;

                // if the next block is a session change, then this storage will be populated
                if (currentSlot.toBigInt() - slotAtEpochStart.toBigInt() > sessionLength.toBigInt()) {
                    expect(randomness.isEmpty).to.not.be.true;
                } else {
                    expect(randomness.isEmpty).to.be.true;
                }
            },
        });

        it({
            id: "C02",
            title: "Rotation happened at previous session boundary",
            test: async () => {
                const blockBabeEpochStart = (await api.query.babe.epochStart())[1].toNumber();
                const apiAtNewSession = await api.at(await api.rpc.chain.getBlockHash(blockBabeEpochStart));

                const blockToCheck = blockBabeEpochStart - 1;
                const apiBeforeNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));

                // After runtime upgrade, randomness in storage is always empty
                if (runtimeVersion < 1400) {
                    // Just before, the randomness was not empty
                    const randomnessBeforeSession = await apiBeforeNewSession.query.tanssiCollatorAssignment.randomness();
                    expect(randomnessBeforeSession.isEmpty).to.not.be.true;
                }

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
