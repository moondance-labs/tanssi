import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "utils";

describeSuite({
    id: "DEVT0302",
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
                nextProfileId.toNumber(),
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
                polkadotJs.tx.servicesPayment.purchaseCredits(2000, 1_000_000_000_000_000),
                polkadotJs.tx.servicesPayment.purchaseCredits(2001, 1_000_000_000_000_000),
                polkadotJs.tx.servicesPayment.purchaseCredits(2002, 1_000_000_000_000_000),
                polkadotJs.tx.servicesPayment.purchaseCredits(2003, 1_000_000_000_000_000),
                polkadotJs.tx.servicesPayment.purchaseCredits(2004, 1_000_000_000_000_000),
            ];

            // We are setting tip for everybody except 2000 and 2002
            const tipTxs = [
                polkadotJs.tx.servicesPayment.setMaxTip(2001, 123456),
                polkadotJs.tx.servicesPayment.setMaxTip(2003, 800000),
                polkadotJs.tx.servicesPayment.setMaxTip(2004, 900000),
            ];

            // Have 1 collators per parathread
            const configChangeTx = polkadotJs.tx.collatorConfiguration.setCollatorsPerParathread(1);
            await context.createBlock([await polkadotJs.tx.sudo.sudo(configChangeTx).signAsync(alice)]);

            const paraTxs = responseFor2002.txs.concat(...responseFor2003.txs).concat(...responseFor2004.txs);
            await context.createBlock([
                await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batchAll(paraTxs)).signAsync(alice),
            ]);

            const pendingParas = await polkadotJs.query.containerRegistrar.pendingParaIds();
            // @ts-expect-error Missing Orchestrator Pallets in api-augment
            expect(pendingParas.length).to.be.eq(1);
            const parasScheduled = pendingParas[0][1];
            expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001, 2002, 2003, 2004]);

            await context.createBlock([await polkadotJs.tx.utility.batch(purchaseCreditTxs).signAsync(alice)]);

            await context.createBlock([
                await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batch(tipTxs)).signAsync(alice),
            ]);

            await jumpSessions(context, 2);

            const activeConfig = (await polkadotJs.query.collatorConfiguration.activeConfig()).toJSON();

            // @ts-expect-error Missing Orchestrator Pallets in api-augment
            const numberOfInvulnerables = (await polkadotJs.query.tanssiInvulnerables.invulnerables()).length;

            // We will have two collators less than we need so that we can detect changes in order
            // in below tests easily.
            const numberOfInvulnerablesNeeded =
                // @ts-expect-error Missing Orchestrator Pallets in api-augment
                activeConfig.collatorsPerContainer * 2 +
                // @ts-expect-error Missing Orchestrator Pallets in api-augment
                activeConfig.collatorsPerParathread * 3 -
                numberOfInvulnerables -
                2;

            const sr25519keyring = new Keyring({ type: "sr25519" });
            const ed25519keyring = new Keyring({ type: "ed25519" });
            const ecdsakeyring = new Keyring({ type: "ecdsa" });

            const setBalanceTxs = [];
            const setKeysTxs = [];
            const collatorAccountIds = [];

            let sudoNonce = (await context.polkadotJs().rpc.system.accountNextIndex(alice.address)).toNumber();

            // Call register collator keys
            for (let i = 0; i < numberOfInvulnerablesNeeded; i++) {
                const { setBalanceTx, setKeysTx, collatorAccountId } = await getRegisterCollatorKeyTx(
                    ed25519keyring,
                    sr25519keyring,
                    ecdsakeyring,
                    polkadotJs,
                    String(i),
                    alice,
                    sudoNonce + i
                );
                setBalanceTxs.push(setBalanceTx);
                setKeysTxs.push(setKeysTx);
                collatorAccountIds.push(collatorAccountId);
            }

            // Create set of tx to put in the block
            await context.createBlock(setBalanceTxs);
            await context.createBlock(setKeysTxs);

            await jumpSessions(context, 2);

            // Call invulnerables tx building
            sudoNonce = (await context.polkadotJs().rpc.system.accountNextIndex(alice.address)).toNumber();
            const setInvlunerablesTxs = [];
            for (let i = 0; i < numberOfInvulnerablesNeeded; i++) {
                setInvlunerablesTxs.push(
                    await getRegisterInvulnerableTx(polkadotJs, alice, collatorAccountIds[i], sudoNonce + i)
                );
            }

            // Create set of tx to put in the block
            await context.createBlock(setInvlunerablesTxs);

            await jumpSessions(context, 2);
        });

        it({
            id: "E01",
            title: "Set of Parathreads would not be truncated",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const collatorAssignmentBefore = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignment(collatorAssignmentBefore)).to.be.deep.equal([
                    2000, 2001, 2004, 2002, 2003,
                ]);

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
                expect(sortCollatorAssignment(collatorAssignmentAfter)).to.be.deep.equal([
                    2000, 2001, 2004, 2002, 2003,
                ]);
            },
        });

        it({
            id: "E02",
            title: "Set of Parachains should be sort by tip and truncated according to max cores allocated if we have less cores",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const collatorAssignmentBefore = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                expect(sortCollatorAssignment(collatorAssignmentBefore)).to.be.deep.equal([
                    2000, 2001, 2004, 2002, 2003,
                ]);

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
                expect(sortCollatorAssignment(collatorAssignmentAfter)).to.be.deep.equal([2001, 2002, 2003, 2004]);

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
                expect(sortCollatorAssignment(collatorAssignmentAtZeroPercent)).to.be.deep.equal([2002, 2003, 2004]);
            },
        });
    },
});

async function getRegisterCollatorKeyTx(ed25519Keyring, sr25519Keyring, ecdsaKeyring, api, name, sudoKey, nonce) {
    const collatorKey = sr25519Keyring.addFromUri(`//${name}COLLATOR_ACC`, { name: `COLLATOR${name} ACC` });
    const existentialDeposit = api.consts.balances.existentialDeposit.toBigInt();

    return {
        setBalanceTx: await api.tx.sudo
            .sudo(api.tx.balances.forceSetBalance(collatorKey.address, existentialDeposit + 1_000_000_000_000_000n))
            .signAsync(sudoKey, { nonce: nonce }),
        setKeysTx: await api.tx.session
            .setKeys(
                {
                    grandpa: ed25519Keyring.addFromUri(`//${name}COLLATOR_GRANDPA`, {
                        name: "COLLATOR" + " GRANDPA",
                    }).publicKey,
                    babe: sr25519Keyring.addFromUri(`//${name}COLLATOR_BABE`, { name: "COLLATOR" + " BABE" }).publicKey,
                    para_validator: sr25519Keyring.addFromUri(`//${name}COLLATOR_PV`, { name: "COLLATOR" + " PV" })
                        .publicKey,
                    para_assignment: sr25519Keyring.addFromUri(`//${name}COLLATOR_PA`, {
                        name: "COLLATOR" + " PA",
                    }).publicKey,
                    authority_discovery: sr25519Keyring.addFromUri(`//${name}COLLATOR_AD`, {
                        name: "COLLATOR" + " AD",
                    }).publicKey,
                    beefy: ecdsaKeyring.addFromUri(`//${name}COLLATOR_BEEFY`, { name: "COLLATOR" + " BEEFY" })
                        .publicKey,
                    nimbus: sr25519Keyring.addFromUri(`//${name}COLLATOR_NIMBUS`, { name: "COLLATOR" + " NIMBUS" })
                        .publicKey,
                },
                []
            )
            .signAsync(collatorKey),
        collatorAccountId: collatorKey.address,
    };
}

async function getRegisterInvulnerableTx(api, sudoKey, collatorAccountId, nonce) {
    return api.tx.sudo
        .sudo(api.tx.tanssiInvulnerables.addInvulnerable(collatorAccountId))
        .signAsync(sudoKey, { nonce: nonce });
}

async function createTxBatchForCreatingPara(
    api,
    manager,
    paraId,
    slotFreq,
    nextProfileId: number,
    containerChainGenesisData,
    headData
) {
    let nextProfile = nextProfileId;
    const txs = [];
    const reserveTx = api.tx.registrar.reserve();
    txs.push(
        api.tx.utility.dispatchAs(
            {
                system: { Signed: manager },
            } as any,
            reserveTx
        )
    );

    let registerTx: any;
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
    const assignmentTx = api.tx.sudo.sudo(api.tx.dataPreservers.forceStartAssignment(nextProfile++, paraId, "Free"));
    txs.push(assignmentTx);
    const trustedValidationCodeTx = api.tx.paras.addTrustedValidationCode("0x0102030405060708090a");
    txs.push(trustedValidationCodeTx);
    const markValidForCollating = api.tx.containerRegistrar.markValidForCollating(paraId);
    txs.push(markValidForCollating);

    return { txs: txs, nextProfileId: nextProfile };
}

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

const sortCollatorAssignment = (collatorAssignment) => {
    return Object.keys(collatorAssignment.containerChains)
        .sort((a, b) => {
            const b_collators = collatorAssignment.containerChains[b].length;
            const a_collators = collatorAssignment.containerChains[a].length;
            if (a_collators !== b_collators) {
                return collatorAssignment.containerChains[b].length - collatorAssignment.containerChains[a].length;
            }
            return Number(a) - Number(b);
        })
        .map((x) => Number(x));
};
