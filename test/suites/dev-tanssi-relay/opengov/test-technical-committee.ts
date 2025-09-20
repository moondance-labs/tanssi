import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { BN } from "@polkadot/util";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ExtrinsicFailedEventDataType } from "../../../utils";
import { isStarlightRuntime } from "../../../utils/runtime.ts";

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
                const proposalIndex = proposals.length - 1;
                const proposalHash = proposals[0];
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
            },
        });

        it({
            id: "E03",
            title: "Technical committee can't enable maintenance mode without enough votes",
            test: async ({ skip }) => {
                if (isStarlightRuntime(api)) {
                    skip();
                }
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
                const proposalIndex = proposals.length;
                const proposalHash = proposals[0];
                expect(proposalHash).to.not.be.undefined;

                // 3. Make only half technical committee members votes for the proposal
                await context.createBlock(
                    api.tx.openTechCommitteeCollective.vote(proposalHash, proposalIndex, true).signAsync(dave),
                    {
                        allowFailures: false,
                        expectEvents: [api.events.openTechCommitteeCollective.Voted],
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
                            proposalIndex,
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
    },
});
