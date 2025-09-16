import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { ExtrinsicFailedEventDataType } from "../../../utils";

describeSuite({
    id: "DEVT24",
    title: "Technical committee test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;

        beforeAll(async () => {
            api = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice");
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
            },
        });
    },
});
