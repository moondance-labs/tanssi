import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "util/block";

const emptyGenesisData = (api) => {
    const g = api.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
        storage: [
            {
                key: "0x3a636f6465",
                value: "0x0102030405060708090a",
            },
        ],
        name: "0x436f6e7461696e657220436861696e2032303030",
        id: "0x636f6e7461696e65722d636861696e2d32303030",
        forkId: null,
        extensions: "0x",
        properties: {
            tokenMetadata: {
                tokenSymbol: "0x61626364",
                ss58Format: 42,
                tokenDecimals: 12,
            },
            isEthereum: false,
        },
    });
    return g;
};

const sortCollatorAssignmentByNumberOfCollators = (collatorAssignment) => {
    return Object.keys(collatorAssignment["containerChains"]).sort((a, b) => {
        return collatorAssignment["containerChains"][b].length - collatorAssignment["containerChains"][a].length;
    });
};

const sortCollatorAssignmentByParaId = (collatorAssignment) => {
    return Object.keys(collatorAssignment["containerChains"]).sort((a, b) => {
        return Number(a) - Number(b);
    });
};

describeSuite({
    id: "DTR0401",
    title: "Collator assignment tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
            const nextProfileId = await polkadotJs.query.dataPreservers.nextProfileId();
            const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                min: 6,
                max: 6,
            });
            const responseFor2002 = await createTxBatchForCreatingPara(
                polkadotJs,
                alice.address,
                2002,
                slotFrequency,
                nextProfileId,
                emptyGenesisData(polkadotJs),
                "0x01"
            );
            const responseFor2003 = await createTxBatchForCreatingPara(
                polkadotJs,
                alice.address,
                2003,
                slotFrequency,
                responseFor2002.nextProfileId,
                emptyGenesisData(polkadotJs),
                "0x01"
            );
            const responseFor2004 = await createTxBatchForCreatingPara(
                polkadotJs,
                alice.address,
                2004,
                slotFrequency,
                responseFor2003.nextProfileId,
                emptyGenesisData(polkadotJs),
                "0x01"
            );

            const purchaseCreditTxs = [
                polkadotJs.tx.servicesPayment.purchaseCredits(2001, 1_000_000_000_000_000),
                polkadotJs.tx.servicesPayment.purchaseCredits(2004, 1_000_000_000_000_000),
            ];

            // We are setting tip for 2004 and 2001
            const tipTxs = [
                polkadotJs.tx.servicesPayment.setMaxTip(2004, 123456),
                polkadotJs.tx.servicesPayment.setMaxTip(2001, 123456),
            ];

            // Have 2 collators per parathread
            const configChangeTx = polkadotJs.tx.collatorConfiguration.setCollatorsPerParathread(2);
            await context.createBlock([await polkadotJs.tx.sudo.sudo(configChangeTx).signAsync(alice)]);

            const paraTxs = responseFor2002.txs.concat(...responseFor2003.txs).concat(...responseFor2004.txs);
            await context.createBlock([
                await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batchAll(paraTxs)).signAsync(alice),
            ]);

            const pendingParas = await polkadotJs.query.containerRegistrar.pendingParaIds();
            expect(pendingParas.length).to.be.eq(1);
            const parasScheduled = pendingParas[0][1];
            expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001, 2002, 2003, 2004]);

            await context.createBlock([await polkadotJs.tx.utility.batch(purchaseCreditTxs).signAsync(alice)]);

            await context.createBlock([
                await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batch(tipTxs)).signAsync(alice),
            ]);

            await jumpSessions(context, 2);
        });

        it({
            id: "E01",
            title: "Set of Parachains should be sort by tip and truncated according to max cores allocated if we have less cores",
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const collatorAssignmentBefore = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignmentByNumberOfCollators(collatorAssignmentBefore).toString()).to.be.eq(
                    "2001,2004,2000,2002,2003"
                );

                // Record previous config value to restore later
                const previousConfig = (await polkadotJs.query.collatorConfiguration.activeConfig()).toJSON();

                // Let's change percentage of parachain to 30%
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.collatorConfiguration.setMaxParachainCoresPercentage(300000000))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Wait for two sessions for the effect
                await jumpSessions(context, 2);

                // Check the active assignment
                const collatorAssignmentAfter = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignmentByNumberOfCollators(collatorAssignmentAfter).toString()).to.be.eq(
                    "2001,2004,2002,2003"
                );

                // Let's change percentage of parachain to 0
                const zeroParachaintx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.collatorConfiguration.setMaxParachainCoresPercentage(0))
                    .signAsync(alice);
                await context.createBlock([zeroParachaintx]);

                // Wait for two sessions for the effect
                await jumpSessions(context, 2);

                // Check the active assignment, we will not have any para id
                const collatorAssignmentAtZeroPercent = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignmentByParaId(collatorAssignmentAtZeroPercent).toString()).to.be.eq(
                    "2002,2003,2004"
                );

                // Restore previous config
                const restoringTx = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.collatorConfiguration.setMaxParachainCoresPercentage(
                            previousConfig.maxParachainCoresPercentage
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([restoringTx]);

                // Wait for two sessions for the effect
                await jumpSessions(context, 2);
            },
        });

        it({
            id: "E02",
            title: "Set of Parathreads would not be truncated",
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const collatorAssignmentBefore = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignmentByNumberOfCollators(collatorAssignmentBefore).toString()).to.be.eq(
                    "2001,2004,2000,2002,2003"
                );

                // Let's change the parachain percentage to 90
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.collatorConfiguration.setMaxParachainCoresPercentage(900000000))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Wait for two sessions for the effect
                await jumpSessions(context, 2);

                // Check the active assignment
                const collatorAssignmentAfter = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                // Pool paras are not truncated but they are sorted by tip
                expect(sortCollatorAssignmentByNumberOfCollators(collatorAssignmentAfter).toString()).to.be.eq(
                    "2001,2004,2000,2002,2003"
                );
            },
        });
    },
});

async function createTxBatchForCreatingPara(
    api,
    manager,
    paraId,
    slotFreq,
    nextProfileId,
    containerChainGenesisData,
    headData
) {
    const txs = [];

    let registerTx;
    if (slotFreq == null) {
        registerTx = api.tx.containerRegistrar.register(paraId, containerChainGenesisData, headData);
    } else {
        registerTx = api.tx.containerRegistrar.registerParathread(
            paraId,
            slotFreq,
            containerChainGenesisData,
            headData
        );
    }
    txs.push(
        api.tx.utility.dispatchAs(
            {
                system: { Signed: manager },
            } as any,
            registerTx
        )
    );
    const profileTx = api.tx.dataPreservers.forceCreateProfile(
        {
            url: "0x02",
            paraIds: "AnyParaId",
            mode: "Bootnode",
            assignmentRequest: "Free",
        },
        manager
    );
    txs.push(profileTx);
    const assignmentTx = api.tx.sudo.sudo(api.tx.dataPreservers.forceStartAssignment(nextProfileId++, paraId, "Free"));
    txs.push(assignmentTx);
    const trustedValidationCodeTx = api.tx.paras.addTrustedValidationCode("0x0102030405060708090a");
    txs.push(trustedValidationCodeTx);
    const markValidForCollating = api.tx.containerRegistrar.markValidForCollating(paraId);
    txs.push(markValidForCollating);

    return { txs: txs, nextProfileId: nextProfileId };
}
