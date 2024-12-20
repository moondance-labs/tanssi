import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, customDevRpcRequest } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { jumpBlocks, jumpSessions, jumpToSession } from "util/block";
import { filterAndApply, generateKeyringPair } from "@moonwall/util";
import { EventRecord } from "@polkadot/types/interfaces";
import { bool, u32, u8, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "DTR0304",
    title: "Collator assignment tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            // Enable randomness for this test.
            // Add a custom seed because with the default one this test fails because collators get assigned to the same
            // chain again.
            await customDevRpcRequest("mock_activateRandomness", [
                "4bbc1fee42ce06ff37f5a07744346c1e0b43c8a4130db8ec5ae41f3234f5c421",
            ]);
        });

        it({
            id: "E01",
            title: "Collator should rotate",
            test: async function () {
                const orchestrator = "RotateAll";
                const parachain = "KeepAll";
                const parathread = { KeepPerbill: { percentage: 500_000_000n } }; // 50%
                const tx = context
                    .polkadotJs()
                    .tx.configuration.setFullRotationMode(orchestrator, parachain, parathread);
                await context.createBlock(polkadotJs.tx.sudo.sudo(tx).signAsync(alice));
                const tx2 = context.polkadotJs().tx.configuration.setCollatorsPerParathread(2);
                await context.createBlock(polkadotJs.tx.sudo.sudo(tx2).signAsync(alice));

                // Add 4 collators more
                // Use random accounts
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();
                const randomAccounts = [];

                for (let i = 0; i < 4; i++) {
                    const randomAccount = generateKeyringPair("sr25519");
                    randomAccounts.push(randomAccount);
                }

                // First block, send some balance to each account. This needs to go first because `.signAndSend(randomAccount)`
                // given an error if the account has no balance, even though we send some balance and it's pending.
                for (const randomAccount of randomAccounts) {
                    const value = 100_000_000_000n;
                    await polkadotJs.tx.balances
                        .transferAllowDeath(randomAccount.address, value)
                        .signAndSend(alice, { nonce: aliceNonce++ });
                }

                await context.createBlock();

                // Second block, add keys and register them as invulnerables
                for (const randomAccount of randomAccounts) {
                    const newKey1 = await polkadotJs.rpc.author.rotateKeys();
                    await polkadotJs.tx.session.setKeys(newKey1, []).signAndSend(randomAccount);

                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.invulnerables.addInvulnerable(randomAccount.address))
                        .signAndSend(alice, { nonce: aliceNonce++ });
                }
                await context.createBlock();

                // Deregister container chains and register parathreads instead
                await deregisterAll(context);
                await registerParathreads(context);

                // Collators are registered, wait 2 sessions for them to be assigned
                await jumpSessions(context, 1);

                const fullRotationPeriod = (await polkadotJs.query.configuration.activeConfig())[
                    "fullRotationPeriod"
                ].toString();
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                // Calculate the remaining sessions for next full rotation
                // This is a workaround for running moonwall in run mode
                // as it runs all tests in the same chain instance
                const remainingSessionsForRotation =
                    sessionIndex > fullRotationPeriod ? sessionIndex % fullRotationPeriod : fullRotationPeriod;

                await jumpToSession(context, remainingSessionsForRotation - 2);

                const initialAssignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();

                expect(initialAssignment.containerChains[2002].length).to.eq(2);
                expect((await polkadotJs.query.collatorAssignment.pendingCollatorContainerChain()).isNone);

                // remainingSessionsForRotation - 1
                await jumpSessions(context, 1);
                const rotationEndAssignment = (
                    await polkadotJs.query.collatorAssignment.collatorContainerChain()
                ).toJSON();

                expect((await polkadotJs.query.collatorAssignment.pendingCollatorContainerChain()).isSome);
                // Assignment shouldn't have changed yet
                expect(initialAssignment.containerChains[2002].toSorted()).to.deep.eq(
                    rotationEndAssignment.containerChains[2002].toSorted()
                );

                // In dev-tanssi, randomness depends only on the block number so it is actually deterministic.
                // First, check that the event has randomness
                const events = await polkadotJs.query.system.events();
                const filteredEvents = filterAndApply(
                    events,
                    "collatorAssignment",
                    ["NewPendingAssignment"],
                    ({ event }: EventRecord) =>
                        event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
                );
                expect(filteredEvents[0].fullRotation.toJSON()).toBe(true);
                // In dev mode randomness is deterministic so the seed should not change, but we only want to check that
                // it's not 0x0000..., so it doesn't matter if it changes.
                expect(filteredEvents[0].randomSeed.toHex()).to.deep.eq(
                    "0x8b145bb9825b580a7a571099151e7ac459b83103abec71bc4d322bcd5bef153f"
                );

                // Check that the randomness is set in CollatorAssignment the
                // block previous to the full rotation
                const sessionDuration = 10;
                await jumpBlocks(context, sessionDuration - 1);

                const assignmentRandomness = await polkadotJs.query.collatorAssignment.randomness();
                expect(assignmentRandomness.isEmpty).toBe(false);

                // Start session 5, with the new random assignment
                await jumpSessions(context, 1);

                const newAssignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();

                // Assignment should have changed
                expect(newAssignment).to.not.deep.eq(initialAssignment);

                // Orchestrator collators should change
                // But they don't change because they are invulnerables, and invulnerables that were previously assigned have priority.
                expect(newAssignment.orchestratorChain).to.not.eq(initialAssignment.orchestratorChain);

                const arrayIntersection = (arr1, arr2) => {
                    const set2 = new Set(arr2);
                    return arr1.filter((item) => set2.has(item));
                };

                // Parathread collators should keep 1 and rotate the other one
                expect(newAssignment.containerChains["2002"].length).toBe(2);
                const sameCollators2002 = arrayIntersection(
                    newAssignment.containerChains["2002"],
                    initialAssignment.containerChains["2002"]
                );
                expect(sameCollators2002.length).toBe(1);
                expect(newAssignment.containerChains["2003"].length).toBe(2);
                const sameCollators2003 = arrayIntersection(
                    newAssignment.containerChains["2003"],
                    initialAssignment.containerChains["2003"]
                );
                expect(sameCollators2003.length).toBe(1);
            },
        });
    },
});

async function deregisterAll(context) {
    const polkadotJs = context.polkadotJs();
    const alice = context.keyring.alice;
    const parasRegistered = (await polkadotJs.query.registrar.registeredParaIds()).toJSON();

    const txs = [];

    for (const paraId of parasRegistered) {
        const tx = polkadotJs.tx.registrar.deregister(paraId);
        txs.push(tx);
    }

    await context.createBlock([await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batchAll(txs)).signAsync(alice)]);
}

async function registerParathreads(context) {
    const polkadotJs = context.polkadotJs();
    const alice = context.keyring.alice;
    await context.createBlock();

    const currentSesssion = await polkadotJs.query.session.currentIndex();
    const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
    const expectedScheduledOnboarding = BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

    const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
        min: 1,
        max: 1,
    });
    const emptyGenesisData = () => {
        const g = polkadotJs.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
            storage: [
                {
                    key: "0x636f6465",
                    value: "0x010203040506",
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

    const containerChainGenesisData = emptyGenesisData();

    for (const paraId of [2002, 2003]) {
        const tx = polkadotJs.tx.registrar.registerParathread(paraId, slotFrequency, containerChainGenesisData, null);

        const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
        const tx2 = polkadotJs.tx.dataPreservers.createProfile({
            url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
            paraIds: "AnyParaId",
            mode: "Bootnode",
            assignmentRequest: "Free",
        });

        const tx3 = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
        const tx4 = polkadotJs.tx.registrar.markValidForCollating(paraId);
        const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
        await context.createBlock([
            await tx.signAsync(alice, { nonce }),
            await tx2.signAsync(alice, { nonce: nonce.addn(1) }),
            await tx3.signAsync(alice, { nonce: nonce.addn(2) }),
            await polkadotJs.tx.sudo.sudo(tx4).signAsync(alice, { nonce: nonce.addn(3) }),
        ]);
    }

    const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
    expect(pendingParas.length).to.be.eq(1);
    const sessionScheduling = pendingParas[0][0];
    const parasScheduled = pendingParas[0][1];

    expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

    // These will be the paras in session 2
    // TODO: fix once we have types
    expect(parasScheduled.toJSON()).to.deep.equal([2002, 2003]);

    // Check that the on chain genesis data is set correctly
    const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2002);
    // TODO: fix once we have types
    expect(emptyGenesisData().toJSON()).to.deep.equal(onChainGenesisData.toJSON());

    // Check the para id has been given some free credits
    const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(2002)).toJSON();
    expect(credits, "Container chain 2002 should have been given credits").toBeGreaterThan(0);

    // Checking that in session 2 paras are registered
    await jumpSessions(context, 2);

    // Expect now paraIds to be registered
    const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
    // TODO: fix once we have types
    expect(parasRegistered.toJSON()).to.deep.equal([2002, 2003]);
}
