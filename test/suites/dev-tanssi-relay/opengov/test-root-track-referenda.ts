import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { maximizeConvictionVotingOf, fastForwardUntilNoMoreEvents } from "../../../utils/democracy.ts";

import type { SubmittedEventDataType } from "../../../utils";

describeSuite({
    id: "DEVT23",
    title: "OpenGov test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let dave: KeyringPair;
        let charlie: KeyringPair;
        let eve: KeyringPair;
        let ferdie: KeyringPair;

        beforeAll(async () => {
            api = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice");
            bob = keyring.addFromUri("//Bob");
            dave = keyring.addFromUri("//Dave");
            charlie = keyring.addFromUri("//Charlie");
            eve = keyring.addFromUri("//Eve");
            ferdie = keyring.addFromUri("//Ferdie");
        });

        it({
            id: "E01",
            timeout: 240000,
            title: "Referenda for root is executed",
            test: async () => {
                // Bob wants to add Alice as a proxy
                const delegate = alice.address;
                const proxyType = "Any";
                const delay = 0;

                const sudoTx = api.tx.sudo.sudoAs(bob.address, api.tx.proxy.addProxy(delegate, proxyType, delay));

                // Step 1: Let's create preimage
                const notePreimageTx = api.tx.preimage.notePreimage(sudoTx.method.toHex());
                const preimageBlock = await context.createBlock(await notePreimageTx.signAsync(alice));
                expect(preimageBlock.result?.successful).to.be.true;

                // Step 2: Alice (root) submits referenda to root track (track 0)
                const submitTx = api.tx.referenda.submit(
                    {
                        system: "Root",
                    },
                    { Lookup: { Hash: sudoTx.method.hash.toHex(), len: sudoTx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(await submitTx.signAsync(alice));
                expect(submitBlock.result?.successful).to.be.true;

                // Step 3: Extract referendum index
                const proposalIndex = (
                    submitBlock.result.events.find((e) => e.event.method === "Submitted").event.toHuman()
                        .data as unknown as SubmittedEventDataType
                ).index;
                expect(proposalIndex).to.not.be.undefined;

                // Step 4: Place decision deposit
                const depositSubmit = await context.createBlock(
                    await api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(ferdie)
                );
                expect(depositSubmit.result?.successful).to.be.true;
                await maximizeConvictionVotingOf(context, [dave, eve, ferdie], proposalIndex);

                await fastForwardUntilNoMoreEvents(context);

                const finishedReferendum = (
                    await context.polkadotJs().query.referenda.referendumInfoFor(proposalIndex)
                ).unwrap();

                expect(finishedReferendum.isApproved, "Not approved").to.be.true;
                expect(finishedReferendum.isOngoing, "Still ongoing").to.be.false;
                expect(finishedReferendum.isTimedOut, "Timed out").to.be.false;

                // Step 6: Confirm proxy actually added
                const proxyInfo = await api.query.proxy.proxies(bob.address);
                const [delegates] = proxyInfo.toJSON() as { delegate: string }[][];
                const added = delegates.find((d: any) => d.delegate === delegate);
                expect(added).to.exist;
            },
        });

        it({
            id: "E02",
            timeout: 120000,
            title: "Not enough support, referenda rejected",
            test: async () => {
                const tx = api.tx.system.remark("0x0005");

                // Step 1: Let's create preimage
                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(await notePreimageTx.signAsync(alice));
                expect(preimageBlock.result?.successful).to.be.true;

                // Redundant block created here, not sure why we need it.
                await context.createBlock();

                // Step 2: Submit referenda to root track (track 0)
                const submitTx = api.tx.referenda.submit(
                    {
                        system: "Root",
                    },
                    { Lookup: { Hash: tx.method.hash.toHex(), len: tx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(await submitTx.signAsync(alice));
                expect(submitBlock.result?.successful).to.be.true;

                // Step 3: Extract referendum index
                const proposalIndex = (
                    submitBlock.result.events.find((e) => e.event.method === "Submitted").event.toHuman()
                        .data as unknown as SubmittedEventDataType
                ).index;
                expect(proposalIndex).to.not.be.undefined;

                // Step 4: Place decision deposit
                const depositSubmit = await context.createBlock(
                    await api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(charlie)
                );
                expect(depositSubmit.result?.successful).to.be.true;

                // Step 5: Voting
                for (const voter of [charlie]) {
                    const voteSubmit = await context.createBlock(
                        await api.tx.convictionVoting
                            .vote(proposalIndex, {
                                Standard: { vote: { aye: false, conviction: "None" }, balance: 900000 },
                            })
                            .signAsync(voter)
                    );
                    expect(voteSubmit.result?.successful).to.be.true;
                }

                await fastForwardUntilNoMoreEvents(context);

                const finishedReferendum = (
                    await context.polkadotJs().query.referenda.referendumInfoFor(proposalIndex)
                ).unwrap();

                expect(finishedReferendum.isApproved, "Not approved").to.be.false;
                expect(finishedReferendum.isOngoing, "Still ongoing").to.be.false;
                expect(finishedReferendum.isTimedOut, "Timed out").to.be.false;
            },
        });
    },
});
