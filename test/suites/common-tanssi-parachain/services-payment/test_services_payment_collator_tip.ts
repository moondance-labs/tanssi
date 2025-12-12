// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { fetchCollatorAssignmentTip, jumpSessions, paraIdTank, generateEmptyGenesisData } from "utils";

describeSuite({
    id: "COMMO0803",
    title: "Services payment collator assignment tip test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let collatorAssignmentAlias: any;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            collatorAssignmentAlias = runtimeName.includes("light")
                ? polkadotJs.query.tanssiCollatorAssignment
                : polkadotJs.query.collatorAssignment;
        });
        it({
            id: "E01",
            title: "Tip shouldn't prioritize collator assignment on congestion",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;

                const tip = 123;

                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                await context.createBlock([await tx.signAsync(alice)]);

                const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(txMaxTip).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 0 collators`
                ).toBe(0);
            },
        });
        it({
            id: "E02",
            title: "Tip is not charged when there are enough collators for all chains",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;
                const otherParaId = 2000n;

                // Deregister the other chain, 2000, so that 2001 is the only chain
                const txDeregister = polkadotJs.tx.registrar.deregister(otherParaId);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(txDeregister).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId].length,
                    `Container chain ${paraId} should have 2 collators`
                ).toBe(2);

                // No tip event
                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent).to.be.undefined;
            },
        });
        it({
            id: "E03",
            title: "If parachain tank account does not have enough balance, collators are not assigned",
            test: async () => {
                await context.createBlock();

                const paraId = 2001n;

                // Set tank account to 0 balance, shouldn't matter because the chain doesn't need tip, and also the
                // chain has credits.
                // But actually this will make some checks fail, and result in the chain being not assigned any collators.
                const tx = polkadotJs.tx.balances.forceSetBalance(paraIdTank(paraId), 0);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);
                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                expect(
                    collators.toJSON().containerChains[paraId]?.length,
                    `Container chain ${paraId} should have 0 collators`
                ).toBeUndefined;

                // No tip event
                const events = await polkadotJs.query.system.events();
                const tipEvent = fetchCollatorAssignmentTip(events);
                expect(tipEvent).to.be.undefined;
            },
        });
        it({
            id: "E04",
            title: "When a slot is available, candidate chains should be prioritized by tip",
            test: async () => {
                await context.createBlock();

                // Register 2 chains
                // - 2002
                {
                    const paraId = 2002n;
                    const tip = 200;

                    const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);
                    const registerTx = await polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
                    await context.createBlock([await registerTx.signAsync(alice)]);

                    const creditsTx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                    await context.createBlock([await creditsTx.signAsync(alice)]);

                    const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                    await context.createBlock([await polkadotJs.tx.sudo.sudo(txMaxTip).signAsync(alice)]);

                    const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                    const profile = {
                        bootnodeUrl: "exemple",
                        paraIds: { whitelist: [paraId] },
                        nodeType: "Substrate",
                        assignmentRequest: "Free",
                        additionalInfo: "0x",
                        directRpcUrls: [],
                        proxyRpcUrls: [],
                    };
                    const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                    await context.createBlock([await profileTx.signAsync(bob)]);

                    const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                    await context.createBlock([await assignTx.signAsync(alice)]);

                    const assignment = (await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON();
                    expect(assignment).to.deep.equal([profileId.toJSON()]);
                }

                // - 2003
                {
                    const paraId = 2003n;
                    const tip = 300;

                    const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);
                    const registerTx = await polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
                    await context.createBlock([await registerTx.signAsync(alice)]);

                    const creditsTx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId, 1_000_000_000_000_000);
                    await context.createBlock([await creditsTx.signAsync(alice)]);

                    const txMaxTip = polkadotJs.tx.servicesPayment.setMaxTip(paraId, tip);
                    await context.createBlock([await polkadotJs.tx.sudo.sudo(txMaxTip).signAsync(alice)]);

                    const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                    const profile = {
                        bootnodeUrl: "exemple",
                        paraIds: { whitelist: [paraId] },
                        nodeType: "Substrate",
                        assignmentRequest: "Free",
                        additionalInfo: "0x",
                        directRpcUrls: [],
                        proxyRpcUrls: [],
                    };
                    const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                    await context.createBlock([await profileTx.signAsync(bob)]);

                    const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                    await context.createBlock([await assignTx.signAsync(alice)]);

                    const assignment = (await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON();
                    expect(assignment).to.deep.equal([profileId.toJSON()]);
                }

                // We mark both chains valid at the same time to ensure they are both considered in the same session.
                const validTx2002 = polkadotJs.tx.sudo.sudo(polkadotJs.tx.registrar.markValidForCollating(2002));
                const validTx2003 = polkadotJs.tx.sudo.sudo(polkadotJs.tx.registrar.markValidForCollating(2003));
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await validTx2002.signAsync(alice),
                    await validTx2003.signAsync(alice, { nonce: nonce.addn(1) }),
                ]);

                await jumpSessions(context, 2);

                const collators = await collatorAssignmentAlias.collatorContainerChain();

                // 2003 should be selected as it has higher tip
                expect(
                    collators.toJSON().containerChains[2003n].length,
                    "Container chain 2003 should have 2 collators"
                ).toBe(2);
            },
        });
    },
});
