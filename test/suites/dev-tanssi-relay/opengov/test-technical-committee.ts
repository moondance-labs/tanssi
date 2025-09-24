import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { BN } from "@polkadot/util";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ExtrinsicFailedEventDataType } from "../../../utils";
import { isStarlightRuntime } from "../../../utils/runtime.ts";
import type { H256 } from "@polkadot/types/interfaces";

export type ProposedEventDataType = {
    account: string;
    proposalIndex: number;
    proposalHash: H256;
    threshold: number;
};

describeSuite({
    id: "DEVT24",
    title: "Technical committee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;

        beforeAll(async () => {
            api = context.polkadotJs();
            if (isStarlightRuntime(api)) {
                return;
            }
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice");
            bob = keyring.addFromUri("//Bob");
            charlie = keyring.addFromUri("//Charlie");
            dave = keyring.addFromUri("//Dave");

            // Adding 2 technical committee members (Charlie, Dave) so we can test
            const addCommitteeTx = api.tx.openTechCommitteeCollective.setMembers(
                [charlie.address, dave.address],
                charlie.address,
                2
            );
            const submitBlock = await context.createBlock(await api.tx.sudo.sudo(addCommitteeTx).signAsync(alice));
            expect(submitBlock.result?.successful).to.be.true;

            // Verify members added
            const committeeMembers = await api.query.openTechCommitteeCollective.members();
            expect(committeeMembers.isEmpty).to.be.false;
            expect(committeeMembers).to.contain(charlie.address);
            expect(committeeMembers).to.contain(dave.address);
        });

        it({
            id: "E01",
            title: "Non-technical committee member address cannot submit a proposal",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }
                const call = api.tx.system.remark("0x0001");
                const failedProposal = api.tx.openTechCommitteeCollective.propose(
                    2, // threshold
                    call,
                    call.length
                );
                const submitFailedProposalBlock = await context.createBlock(await failedProposal.signAsync(alice));
                expect(submitFailedProposalBlock.result?.successful).to.be.false;

                const metadata = await api.rpc.state.getMetadata();
                const techCommitteePalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "OpenTechCommitteeCollective")
                    .index.toString();

                const errorData = submitFailedProposalBlock.result.events
                    .find((e) => e.event.method === "ExtrinsicFailed")
                    .event.toHuman().data as unknown as ExtrinsicFailedEventDataType;
                expect(errorData.dispatchError.Module.index).toEqual(techCommitteePalletIndex);

                const errorBytes = Uint8Array.from(Buffer.from(errorData.dispatchError.Module.error.slice(2), "hex"));
                const errorIndex = errorBytes[0];

                const errorMeta = api.registry.findMetaError({
                    index: new BN(errorData.dispatchError.Module.index),
                    error: new BN(errorIndex),
                });

                expect(errorMeta.method).toEqual("NotMember");
            },
        });

        it({
            id: "E02",
            title: "Technical committee can enable maintenance mode",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // Verify that maintenance mode is disabled
                const initialMaintenanceStatus = await api.query.maintenanceMode.maintenanceMode();
                expect(initialMaintenanceStatus.isFalse, "Maintenance mode should be disabled");

                // 1. Compose the technical committee proposal to enable maintenance mode
                const maintenanceModeCall = api.tx.maintenanceMode.enterMaintenanceMode();
                const maintenanceModeProposal = api.tx.openTechCommitteeCollective.propose(
                    2, // threshold
                    maintenanceModeCall,
                    maintenanceModeCall.length
                );

                // 2. Submit the proposal and get the proposal index and hash
                const maintenanceModeProposalBlock = await context.createBlock(
                    await maintenanceModeProposal.signAsync(charlie)
                );
                expect(maintenanceModeProposalBlock.result?.successful).to.be.true;
                const proposals = await api.query.openTechCommitteeCollective.proposals();
                expect(proposals.length).to.be.equal(1, "There should be one active proposal");

                const proposedEventData = maintenanceModeProposalBlock.result?.events.find(
                    ({ event: { method } }) => method.toString() === "Proposed"
                )?.event.data as unknown as ProposedEventDataType;
                expect(proposedEventData).to.not.be.undefined;

                const proposalIndex = proposedEventData.proposalIndex;
                const proposalHash = proposedEventData.proposalHash;
                expect(proposalHash).to.not.be.undefined;

                // 3. Technical committee members votes for the proposal
                const tallyBeforeVoting = await api.query.openTechCommitteeCollective.voting(proposalHash);
                expect(tallyBeforeVoting.isSome).to.be.true;
                expect(tallyBeforeVoting.unwrap().ayes.length).to.be.equal(0);
                await context.createBlock(
                    [
                        api.tx.openTechCommitteeCollective.vote(proposalHash, proposalIndex, true).signAsync(charlie),
                        api.tx.openTechCommitteeCollective.vote(proposalHash, proposalIndex, true).signAsync(dave),
                    ],
                    {
                        allowFailures: false,
                        expectEvents: [api.events.openTechCommitteeCollective.Voted],
                    }
                );
                const tallyAfterVoting = await api.query.openTechCommitteeCollective.voting(proposalHash);
                expect(tallyAfterVoting.isSome).to.be.true;
                expect(tallyAfterVoting.unwrap().ayes.length).to.be.equal(2);

                // 4. Close the proposal and verify maintenance mode is enabled
                await context.createBlock(
                    api.tx.openTechCommitteeCollective
                        .close(
                            proposalHash,
                            proposalIndex,
                            {
                                refTime: 5_000_000_000,
                                proofSize: 100_000,
                            },
                            maintenanceModeCall.length
                        )
                        .signAsync(charlie),
                    {
                        expectEvents: [
                            api.events.openTechCommitteeCollective.Closed,
                            api.events.openTechCommitteeCollective.Approved,
                            api.events.openTechCommitteeCollective.Executed,
                            api.events.maintenanceMode.EnteredMaintenanceMode,
                        ],
                    }
                );
                const maintenanceStatus = await api.query.maintenanceMode.maintenanceMode();
                expect(maintenanceStatus.isTrue, "Maintenance mode should be enabled");
                const proposalsAfterClosing = await api.query.openTechCommitteeCollective.proposals();
                expect(proposalsAfterClosing.length).to.be.equal(0, "There should be no active proposal");

                // Cleanup: Exit maintenance mode for other tests
                const exitMaintenanceBlock = await context.createBlock(
                    await api.tx.sudo.sudo(api.tx.maintenanceMode.resumeNormalOperation()).signAsync(alice)
                );
                expect(exitMaintenanceBlock.result?.successful).to.be.true;
                const finalMaintenanceStatus = await api.query.maintenanceMode.maintenanceMode();
                expect(finalMaintenanceStatus.isFalse, "Maintenance mode should be disabled again");
            },
        });

        it({
            id: "E03",
            title: "Technical committee can't enable maintenance mode without enough votes",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // Verify that maintenance mode is disabled
                const initialMaintenanceStatus = await api.query.maintenanceMode.maintenanceMode();
                expect(initialMaintenanceStatus.isFalse, "Maintenance mode should be disabled");

                // 1. Compose the technical committee proposal to enable maintenance mode
                const maintenanceModeCall = api.tx.maintenanceMode.enterMaintenanceMode();
                const maintenanceModeProposal = api.tx.openTechCommitteeCollective.propose(
                    2, // threshold
                    maintenanceModeCall,
                    maintenanceModeCall.length
                );

                // 2. Submit the proposal and get the proposal index and hash
                const maintenanceModeProposalBlock = await context.createBlock(
                    await maintenanceModeProposal.signAsync(charlie)
                );
                expect(maintenanceModeProposalBlock.result?.successful).to.be.true;
                const proposals = await api.query.openTechCommitteeCollective.proposals();
                expect(proposals.length).to.be.equal(1, "There should be one active proposal");

                const proposedEventData = maintenanceModeProposalBlock.result?.events.find(
                    ({ event: { method } }) => method.toString() === "Proposed"
                )?.event.data as unknown as ProposedEventDataType;
                expect(proposedEventData).to.not.be.undefined;

                const proposalIndex = proposedEventData.proposalIndex;
                const proposalHash = proposedEventData.proposalHash;
                expect(proposalHash).to.not.be.undefined;

                // 3. Make only half technical committee members votes for the proposal
                await context.createBlock(
                    api.tx.openTechCommitteeCollective.vote(proposalHash, proposalIndex, true).signAsync(dave),
                    {
                        allowFailures: false,
                    }
                );
                const tallyAfterVoting = await api.query.openTechCommitteeCollective.voting(proposalHash);
                expect(tallyAfterVoting.isSome).to.be.true;
                expect(tallyAfterVoting.unwrap().ayes.length).to.be.equal(1);

                // 4. Try to close the proposal and verify maintenance mode is not enabled
                const failedProposalClosingBlock = await context.createBlock(
                    api.tx.openTechCommitteeCollective
                        .close(
                            proposalHash,
                            1,
                            {
                                refTime: 5_000_000_000,
                                proofSize: 100_000,
                            },
                            maintenanceModeCall.length
                        )
                        .signAsync(charlie)
                );
                expect(failedProposalClosingBlock.result?.successful).to.be.false;

                const metadata = await api.rpc.state.getMetadata();
                const techCommitteePalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "OpenTechCommitteeCollective")
                    .index.toString();

                const errorData = failedProposalClosingBlock.result.events
                    .find((e) => e.event.method === "ExtrinsicFailed")
                    .event.toHuman().data as unknown as ExtrinsicFailedEventDataType;
                expect(errorData.dispatchError.Module.index).toEqual(techCommitteePalletIndex);

                const errorBytes = Uint8Array.from(Buffer.from(errorData.dispatchError.Module.error.slice(2), "hex"));
                const errorIndex = errorBytes[0];

                const errorMeta = api.registry.findMetaError({
                    index: new BN(errorData.dispatchError.Module.index),
                    error: new BN(errorIndex),
                });
                expect(errorMeta.method).toEqual("TooEarly");

                const maintenanceStatus = await api.query.maintenanceMode.maintenanceMode();
                expect(maintenanceStatus.isFalse, "Maintenance mode should still be disabled");

                const proposalsAfterClosing = await api.query.openTechCommitteeCollective.proposals();
                expect(proposalsAfterClosing.length).to.be.equal(1, "The proposal should still be active");
            },
        });

        it({
            id: "E04",
            title: "Non-technical committee member address cannot vote on a technical committee proposal",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // 1. Compose the technical committee proposal
                const call = api.tx.system.remark("0x0001");
                const proposal = api.tx.openTechCommitteeCollective.propose(
                    2, // threshold
                    call,
                    call.length
                );
                const proposalBlock = await context.createBlock(await proposal.signAsync(charlie));
                expect(proposalBlock.result?.successful).to.be.true;

                // 2. Get the proposal index and hash
                const proposedEventData = proposalBlock.result?.events.find(
                    ({ event: { method } }) => method.toString() === "Proposed"
                )?.event.data as unknown as ProposedEventDataType;
                expect(proposedEventData).to.not.be.undefined;

                const proposalIndex = proposedEventData.proposalIndex;
                const proposalHash = proposedEventData.proposalHash;
                expect(proposalHash).to.not.be.undefined;

                // 3. Try to vote on the proposal with a non-committee member address
                const submitFailedVoteBlock = await context.createBlock(
                    api.tx.openTechCommitteeCollective.vote(proposalHash, proposalIndex, true).signAsync(bob)
                );
                expect(submitFailedVoteBlock.result?.successful).to.be.false;

                const metadata = await api.rpc.state.getMetadata();
                const techCommitteePalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "OpenTechCommitteeCollective")
                    .index.toString();

                const errorData = submitFailedVoteBlock.result.events
                    .find((e) => e.event.method === "ExtrinsicFailed")
                    .event.toHuman().data as unknown as ExtrinsicFailedEventDataType;
                expect(errorData.dispatchError.Module.index).toEqual(techCommitteePalletIndex);

                const errorBytes = Uint8Array.from(Buffer.from(errorData.dispatchError.Module.error.slice(2), "hex"));
                const errorIndex = errorBytes[0];

                const errorMeta = api.registry.findMetaError({
                    index: new BN(errorData.dispatchError.Module.index),
                    error: new BN(errorIndex),
                });

                expect(errorMeta.method).toEqual("NotMember");
                const tallyAfterVotingAttempt = await api.query.openTechCommitteeCollective.voting(proposalHash);
                expect(tallyAfterVotingAttempt.isSome).to.be.true;
                expect(tallyAfterVotingAttempt.unwrap().ayes.length).to.be.equal(0);
            },
        });

        it({
            id: "E05",
            title: "Technical committee can disable maintenance mode",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }
                // 1. Enable maintenance mode
                await context.createBlock(
                    api.tx.sudo.sudo(api.tx.maintenanceMode.enterMaintenanceMode()).signAsync(alice)
                );
                const maintenanceStatusAfterEnabling = await api.query.maintenanceMode.maintenanceMode();
                expect(maintenanceStatusAfterEnabling.isTrue, "Maintenance mode should be enabled");

                // 2. Compose the technical committee proposal to enable maintenance mode
                const disableMaintenanceModeCall = api.tx.maintenanceMode.resumeNormalOperation();
                const disableMaintenanceModeProposal = api.tx.openTechCommitteeCollective.propose(
                    1, // threshold
                    disableMaintenanceModeCall,
                    disableMaintenanceModeCall.length
                );

                // 3. Since the proposal has threshold of 1, we can skip voting and go to closing the proposal directly
                const disableMaintenanceModeProposalBlock = await context.createBlock(
                    await disableMaintenanceModeProposal.signAsync(charlie)
                );
                expect(disableMaintenanceModeProposalBlock.result?.successful).to.be.true;
                const maintenanceStatusAfterDisabling = await api.query.maintenanceMode.maintenanceMode();
                expect(maintenanceStatusAfterDisabling.isFalse, "Maintenance mode should be disabled");
            },
        });

        it({
            id: "E06",
            title: "Technical committee can whitelist a call",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // 1. Compose the technical committee proposal to whitelist a call
                const call = api.tx.system.remark("0x0001");
                const whitelistCall = api.tx.whitelist.whitelistCall(call.method.hash.toHex());
                const whitelistCallProposal = api.tx.openTechCommitteeCollective.propose(
                    1, // threshold
                    whitelistCall,
                    whitelistCall.length
                );

                // Pre-check: Verify the call is not whitelisted
                const isCallWhitelistedBeforeProposal = await api.query.whitelist.whitelistedCall(
                    call.method.hash.toHex()
                );
                expect(isCallWhitelistedBeforeProposal.isNone, "The call should not be whitelisted yet");

                // 2. Since the proposal has threshold of 1, we can skip voting and go to closing the proposal directly
                const whitelistedProposalBlock = await context.createBlock(
                    await whitelistCallProposal.signAsync(charlie)
                );

                expect(whitelistedProposalBlock.result?.successful).to.be.true;

                // 3. Verify the call is whitelisted
                const isCallWhitelistedAfterProposal = await api.query.whitelist.whitelistedCall(
                    call.method.hash.toHex()
                );
                expect(isCallWhitelistedAfterProposal.isSome, "The call should be whitelisted");
            },
        });

        it({
            id: "E07",
            title: "Non-whitelist origin cannot dispatch a whitelisted call",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // Pre-check: Verify the call is whitelisted
                const call = api.tx.system.remark("0x0001");
                const isCallWhitelisted = await api.query.whitelist.whitelistedCall(call.method.hash.toHex());
                expect(isCallWhitelisted.isSome, "The call should be whitelisted");

                // 1. Compose the whitelisted call dispatch
                const whitelistedCallDispatchTx = api.tx.whitelist.dispatchWhitelistedCallWithPreimage(call);

                // 2. Try to dispatch the whitelisted call using a non-whitelist origin (Charlie)
                const failedWhitelistedCallDispatchBlock = await context.createBlock(
                    await whitelistedCallDispatchTx.signAsync(charlie)
                );
                expect(failedWhitelistedCallDispatchBlock.result?.successful).to.be.false;

                const isCallWhitelistedAfterFailedWhitelistDispatch = await api.query.whitelist.whitelistedCall(
                    call.method.hash.toHex()
                );
                expect(isCallWhitelistedAfterFailedWhitelistDispatch.isSome, "The call should still be whitelisted");
            },
        });
    },
});
