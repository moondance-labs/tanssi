import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { fetchRandomnessEvent } from "util/block";
describeSuite({
    id: "S09",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Randomness storage is empty because on-finalize cleans it, unless on session change boundaries",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }
                const sessionLength = 600;
                const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                const randomness = await api.query.collatorAssignment.randomness();

                // if the next block is a session change, then this storage will be populated
                if (currentBlock + (1 % sessionLength) == 0) {
                    expect(randomness.isEmpty).to.not.be.true;
                } else {
                    expect(randomness.isEmpty).to.be.true;
                }
            },
        });

        it({
            id: "C02",
            title: "Rotation happened at previous session boundary",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }
                const sessionLength = runtimeVersion > 500 ? 600 : 300;

                const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();

                const blockToCheck = Math.trunc(currentBlock / sessionLength) * sessionLength;
                const apiAtIssuanceNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));
                const apiAtIssuanceBeforeNewSession = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));

                // Just before, the randomness was not empty
                const randomnessBeforeSession =
                    await apiAtIssuanceBeforeNewSession.query.collatorAssignment.randomness();
                expect(randomnessBeforeSession.isEmpty).to.not.be.true;

                // After, the randomness gets cleaned
                const randomnessAfterSession = await apiAtIssuanceNewSession.query.collatorAssignment.randomness();
                expect(randomnessAfterSession.isEmpty).to.be.true;

                // The rotation event should have kicked in, if enabled
                const events = await apiAtIssuanceNewSession.query.system.events();
                const randomnessEvent = fetchRandomnessEvent(events);
                const session = await apiAtIssuanceNewSession.query.session.currentIndex();

                expect(randomnessEvent.randomSeed.toHex()).to.not.be.equal(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );
                expect(randomnessEvent.targetSession.toNumber()).to.be.equal(session.toNumber() + 1);
                const configuration = await apiAtIssuanceNewSession.query.configuration.activeConfig();
                if (
                    configuration.fullRotationPeriod == 0 ||
                    randomnessEvent.targetSession.toNumber() % configuration.fullRotationPeriod != 0
                ) {
                    expect(randomnessEvent.fullRotation.toHuman()).to.be.false;
                } else {
                    expect(randomnessEvent.fullRotation.toHuman()).to.be.true;
                }
            },
        });
    },
});
