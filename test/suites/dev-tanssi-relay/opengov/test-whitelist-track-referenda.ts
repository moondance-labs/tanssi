import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect, fastFowardToNextEvent } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { type SubmittedEventDataType } from "../../../utils";
import type { H256 } from "@polkadot/types/interfaces";
import { maximizeConvictionVotingOf } from "../../../utils/democracy.ts";

export type ProposedEventDataType = {
    account: string;
    proposalIndex: number;
    proposalHash: H256;
    threshold: number;
};

describeSuite({
    id: "DEVT25",
    title: "Whitelist referenda test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let eve: KeyringPair;
        let ferdie: KeyringPair;
        let call: any;

        beforeAll(async () => {
            api = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice");
            bob = keyring.addFromUri("//Bob");
            charlie = keyring.addFromUri("//Charlie");
            dave = keyring.addFromUri("//Dave");
            eve = keyring.addFromUri("//Eve");
            ferdie = keyring.addFromUri("//Ferdie");

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

            // whitelist call so that it succeeds after passing the whitelist track refernda
            // the whitelist could have come after proposing in the referenda, the order does not matter
            // but it has to be whitelisted before the referenda passes
            call = api.tx.system.remarkWithEvent("0x0001");
            const whitelistCall = api.tx.whitelist.whitelistCall(call.method.hash.toHex());
            const whitelistCallProposal = api.tx.openTechCommitteeCollective.propose(
                1, // threshold
                whitelistCall,
                whitelistCall.length
            );

            await context.createBlock(await whitelistCallProposal.signAsync(charlie));

            const isCallWhitelistedAfterProposal = (await api.query.whitelist.whitelistedCall(
                call.method.hash.toHex()
            )) as any;
            expect(isCallWhitelistedAfterProposal.isSome, "The call should be whitelisted");
        });

        it({
            id: "E01",
            title: "Whitelisted call can be dispatched via whitelist track proposal",
            test: async () => {
                // 1. Note whitelisted call and  dispatch preimages
                await context.createBlock(await api.tx.preimage.notePreimage(call.method.toHex()).signAsync(charlie), {
                    allowFailures: false,
                });
                const whitelistedCallDispatchTx = api.tx.whitelist.dispatchWhitelistedCallWithPreimage(call);
                // this is the referenda that we propose
                // it contains inside the inner tx that we want to dispatch (remark)
                await context.createBlock(
                    await api.tx.preimage.notePreimage(whitelistedCallDispatchTx.method.toHex()).signAsync(charlie),
                    {
                        allowFailures: false,
                    }
                );

                // 2. Create whitelisted call referendum proposal for the whitelisted track
                const whitelistedCallReferendumProposalTx = api.tx.referenda.submit(
                    {
                        Origins: "WhitelistedCaller",
                    },
                    {
                        Lookup: {
                            Hash: whitelistedCallDispatchTx.method.hash.toHex(),
                            len: whitelistedCallDispatchTx.method.encodedLength,
                        },
                    },
                    { After: "1" }
                );
                const whitelistedCallReferendumProposalBlock = await context.createBlock(
                    await whitelistedCallReferendumProposalTx.signAsync(charlie)
                );

                // 3. Extract referendum index
                expect(whitelistedCallReferendumProposalBlock.result?.successful).to.be.true;

                const proposalIndex = (
                    whitelistedCallReferendumProposalBlock.result?.events
                        .find((e) => e.event.method === "Submitted")
                        .event.toHuman().data as unknown as SubmittedEventDataType
                ).index;
                expect(proposalIndex).to.not.be.undefined;

                // 4. Place decision deposit for the proposal
                const depositSubmitBlock = await context.createBlock(
                    await api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(charlie)
                );
                expect(depositSubmitBlock.result?.successful).to.be.true;

                await maximizeConvictionVotingOf(context, [dave, eve, ferdie], proposalIndex);

                await fastFowardToNextEvent(context); // Fast forward past preparation
                await fastFowardToNextEvent(context); // Fast forward past decision
                await fastFowardToNextEvent(context); // Fast forward past enactment
                await fastFowardToNextEvent(context); // Fast forward past confirming

                const finishedReferendum = (
                    await context.polkadotJs().query.referenda.referendumInfoFor(proposalIndex)
                ).unwrap();

                expect(finishedReferendum.isApproved, "Not approved").to.be.true;
                expect(finishedReferendum.isOngoing, "Still ongoing").to.be.false;
                expect(finishedReferendum.isTimedOut, "Timed out").to.be.false;

                await fastFowardToNextEvent(context); // Fast forward past dispatched

                // 7. Verify the call is no longer whitelisted and the dispatch was successful
                const isCallWhitelistedAfterFailedWhitelistDispatchTx = (await api.query.whitelist.whitelistedCall(
                    call.method.hash.toHex()
                )) as any;
                expect(
                    isCallWhitelistedAfterFailedWhitelistDispatchTx.isNone,
                    "The call should not be whitelisted anymore"
                );
            },
        });

        it({
            id: "E02",
            title: "Non-whitelisted call cannot be dispatched via whitelist referendum track",
            test: async () => {
                // Pre-check: Verify the call is  not whitelisted
                const delegate = alice.address;
                const proxyType = "Any";
                const delay = 0;

                const call = api.tx.sudo.sudoAs(charlie.address, api.tx.proxy.addProxy(delegate, proxyType, delay));
                const isCallWhitelisted = await api.query.whitelist.whitelistedCall(call.method.hash.toHex());
                expect(isCallWhitelisted.isNone, "The call should not be whitelisted");

                // 1. Note whitelisted call dispatch referendum preimage
                const whitelistedCallDispatchTx = api.tx.whitelist.dispatchWhitelistedCallWithPreimage(call);
                await context.createBlock(
                    await api.tx.preimage.notePreimage(whitelistedCallDispatchTx.method.toHex()).signAsync(charlie),
                    {
                        allowFailures: false,
                    }
                );

                // 2. Create whitelisted call referendum proposal for the whitelisted track
                const whitelistedCallReferendumProposalTx = api.tx.referenda.submit(
                    {
                        Origins: "WhitelistedCaller",
                    },
                    {
                        Lookup: {
                            Hash: whitelistedCallDispatchTx.method.hash.toHex(),
                            len: whitelistedCallDispatchTx.method.encodedLength,
                        },
                    },
                    { After: "1" }
                );

                const whitelistedCallReferendumProposalBlock = await context.createBlock(
                    await whitelistedCallReferendumProposalTx.signAsync(charlie)
                );

                // 3. Extract referendum index
                expect(whitelistedCallReferendumProposalBlock.result?.successful).to.be.true;

                const proposalIndex = (
                    whitelistedCallReferendumProposalBlock.result?.events
                        .find((e) => e.event.method === "Submitted")
                        .event.toHuman().data as unknown as SubmittedEventDataType
                ).index;
                expect(proposalIndex).to.not.be.undefined;

                // 4. Place decision deposit for the proposal
                const depositSubmitBlock = await context.createBlock(
                    await api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(charlie)
                );
                expect(depositSubmitBlock.result?.successful).to.be.true;

                await maximizeConvictionVotingOf(context, [dave, eve, ferdie], proposalIndex);

                await fastFowardToNextEvent(context); // Fast forward past preparation
                await fastFowardToNextEvent(context); // Fast forward past decision
                await fastFowardToNextEvent(context); // Fast forward past enactment
                await fastFowardToNextEvent(context); // Fast forward past confirming

                const finishedReferendum = (
                    await context.polkadotJs().query.referenda.referendumInfoFor(proposalIndex)
                ).unwrap();

                expect(finishedReferendum.isApproved, "Not approved").to.be.true;
                expect(finishedReferendum.isOngoing, "Still ongoing").to.be.false;
                expect(finishedReferendum.isTimedOut, "Timed out").to.be.false;

                await fastFowardToNextEvent(context); // Fast forward past dispatch
                // 7. Confirm proxy is not added
                const proxyInfo = await api.query.proxy.proxies(charlie.address);
                const [delegates] = proxyInfo.toJSON() as { delegate: string }[][];
                const added = delegates.find((d: any) => d.delegate === delegate);
                expect(added).not.to.exist;
            },
        });
    },
});
