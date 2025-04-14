import "@tanssi/api-augment";
import { type DevModeContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { Digest, DigestItem, HeadData, Header, ParaId, Slot } from "@polkadot/types/interfaces";
import { jumpToSession } from "utils";
import { stringToHex } from "@polkadot/util";

async function mockAndInsertHeadData(
    context: DevModeContext,
    paraId: ParaId,
    blockNumber: number,
    slotNumber: number,
    sudoAccount: KeyringPair
) {
    const relayApi = context.polkadotJs();
    const aura_engine_id = stringToHex("aura");

    const slotNumberT: Slot = relayApi.createType("Slot", slotNumber);
    const digestItem: DigestItem = relayApi.createType("DigestItem", {
        PreRuntime: [aura_engine_id, slotNumberT.toHex(true)],
    });
    const digest: Digest = relayApi.createType("Digest", {
        logs: [digestItem],
    });
    const header: Header = relayApi.createType("Header", {
        parentHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
        number: blockNumber,
        stateRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        extrinsicsRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        digest,
    });

    const headData: HeadData = relayApi.createType("HeadData", header.toHex());
    const paraHeadKey = relayApi.query.paras.heads.key(paraId);

    await context.createBlock(
        relayApi.tx.sudo
            .sudo(relayApi.tx.system.setStorage([[paraHeadKey, `0xc101${headData.toHex().slice(2)}`]]))
            .signAsync(sudoAccount),
        { allowFailures: false }
    );
}

describeSuite({
    id: "DEVT2001",
    title: "Dancelight: Inactivity tracking test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Pallet should correctly update collators' activity records",
            test: async () => {
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                // No collators assigned to container chains until session 2 so activity tracking storages should be empty
                await jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                let activeCollators = await polkadotJs.query.inactivityTracking.activeCollators(startSession - 2);
                expect(activeCollators.isEmpty).to.be.true;
                activeCollators = await polkadotJs.query.inactivityTracking.activeCollators(startSession - 1);
                expect(activeCollators.isEmpty).to.be.true;
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // be empty
                const activeCollatorsForSession2BeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2BeforeNoting.isEmpty).to.be.true;

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await mockAndInsertHeadData(context, 2000, 2, 2, alice);
                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterNoting.size).to.be.equal(1);

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator again
                await mockAndInsertHeadData(context, 2000, 3, 2, alice);
                await context.createBlock();
                const activeCollatorsForSession2AfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterSecondNoting.size).to.be.equal(1);

                // Check that the collators are not added to the activity tracking storage for the current session
                // before the end of the session
                const activeCollatorsRecordBeforeActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordBeforeActivityWindow.isEmpty).to.be.true;

                // Check that the collators are added to the activity tracking storage for the current session
                // after the end of the session
                await jumpToSession(context, startSession + 1);
                const activeCollatorsRecordWithinActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordWithinActivityWindow.size).to.be.equal(1);

                // After the end of activity period, the collators should be removed from the activity tracking storage
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const activeCollatorsRecordAfterActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordAfterActivityWindow.isEmpty).to.be.true;
            },
        });
    },
});
