import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { isDancelightRuntime, isStarlightRuntime } from "../../../utils/runtime.ts";

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
            timeout: 60000,
            title: "Referenda for root is executed",
            test: async ({ skip }) => {
                // Skip if the runtime is Starlight, as OpenGov is not supported
                if (isStarlightRuntime(api)) {
                    skip();
                }

                // Bob wants to add Alice as a proxy
                const delegate = alice.address;
                const proxyType = "Any";
                const delay = 0;

                const sudoTx = api.tx.sudo.sudoAs(bob.address, api.tx.proxy.addProxy(delegate, proxyType, delay));

                // Step 1: Bob creates preimage
                const notePreimageTx = api.tx.preimage.notePreimage(sudoTx.method.toHex());
                const preimageBlock = await context.createBlock(notePreimageTx.signAsync(alice));
                expect(preimageBlock.result?.successful).to.be.true;

                // Step 2: Alice (root) submits referenda to root track (track 0)
                const submitTx = api.tx.referenda.submit(
                    {
                        system: "Root",
                    },
                    { Lookup: { Hash: sudoTx.method.hash.toHex(), len: sudoTx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(submitTx.signAsync(alice));
                expect(submitBlock.result?.successful).to.be.true;

                // Step 3: Extract referendum index
                const proposalIndex = submitBlock.result.events
                    .find((e) => e.event.method === "Submitted")
                    .event.toHuman().data.index;
                expect(proposalIndex).to.not.be.undefined;

                // Step 4: Place decision deposit
                const depositSubmit = await context.createBlock(
                    api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(ferdie)
                );
                expect(depositSubmit.result?.successful).to.be.true;

                // Step 5: Voting
                for (const voter of [alice, bob, dave, charlie, eve]) {
                    const voteSubmit = await context.createBlock(
                        api.tx.convictionVoting
                            .vote(proposalIndex, {
                                Standard: { vote: { aye: true, conviction: "None" }, balance: 999999000000000000n },
                            })
                            .signAsync(voter)
                    );
                    expect(voteSubmit.result?.successful).to.be.true;
                }

                const expectedEvents = [
                    { section: "referenda", method: "DecisionStarted" },
                    { section: "referenda", method: "ConfirmStarted" },
                    { section: "referenda", method: "Confirmed" },
                    { section: "scheduler", method: "Dispatched" },
                    { section: "scheduler", method: "Dispatched" },
                ];

                // Step 5: Wait for referendum to be executed
                for (let i = 0; i < 450; i++) {
                    await context.createBlock();

                    const events = await api.query.system.events();
                    const execEvent = events.find(
                        (e) =>
                            e.event.section === expectedEvents[0].section && e.event.method === expectedEvents[0].method
                    );

                    if (execEvent) {
                        expectedEvents.shift();
                    }

                    if (expectedEvents.length === 0) {
                        break;
                    }
                }

                // Check if all the events happened in the specified order.
                expect(expectedEvents.length).toEqual(0);

                // Step 6: Confirm proxy actually added
                const proxyInfo = await api.query.proxy.proxies(bob.address);
                const [delegates] = proxyInfo.toJSON() as { delegate: string }[][];
                const added = delegates.find((d: any) => d.delegate === delegate);
                expect(added).to.exist;
            },
        });

        it({
            id: "E02",
            title: "Not enough support, referenda rejected",
            test: async ({ skip }) => {
                // Skip if the runtime is Starlight, as OpenGov is not supported
                if (isStarlightRuntime(api)) {
                    skip();
                }

                const tx = api.tx.system.remark("0x00");

                // Step 1: Bob creates preimage
                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(notePreimageTx.signAsync(alice));
                expect(preimageBlock.result?.successful).to.be.true;

                // Step 2: Alice (root) submits referenda to root track (track 0)
                const submitTx = api.tx.referenda.submit(
                    {
                        system: "Root",
                    },
                    { Lookup: { Hash: tx.method.hash.toHex(), len: tx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(submitTx.signAsync(alice));
                expect(submitBlock.result?.successful).to.be.true;

                // Step 3: Extract referendum index
                const proposalIndex = submitBlock.result.events
                    .find((e) => e.event.method === "Submitted")
                    .event.toHuman().data.index;
                expect(proposalIndex).to.not.be.undefined;

                // Step 4: Place decision deposit
                const depositSubmit = await context.createBlock(
                    api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(ferdie)
                );
                expect(depositSubmit.result?.successful).to.be.true;

                // Step 5: Voting
                for (const voter of [alice]) {
                    const voteSubmit = await context.createBlock(
                        api.tx.convictionVoting
                            .vote(proposalIndex, {
                                Standard: { vote: { aye: false, conviction: "None" }, balance: 900000 },
                            })
                            .signAsync(voter)
                    );
                    expect(voteSubmit.result?.successful).to.be.true;
                }

                const expectedEvents = [
                    { section: "referenda", method: "DecisionStarted" },
                    { section: "referenda", method: "Rejected" },
                ];

                // Step 5: Wait for referendum to be executed
                for (let i = 0; i < 450; i++) {
                    await context.createBlock();

                    const events = await api.query.system.events();

                    const execEvent = events.find(
                        (e) =>
                            e.event.section === expectedEvents[0].section && e.event.method === expectedEvents[0].method
                    );

                    if (execEvent) {
                        expectedEvents.shift();
                    }

                    if (expectedEvents.length === 0) {
                        break;
                    }
                }

                // Check if all the events happened in the specified order.
                expect(expectedEvents.length).toEqual(0);
            },
        });

        it({
            id: "E03",
            title: "Referenda is disabled for Starlight",
            test: async ({ skip }) => {
                if (isDancelightRuntime(api)) {
                    skip();
                }

                const delegate = alice.address;
                const proxyType = "Any";
                const delay = 0;

                const tx = api.tx.proxy.addProxy(delegate, proxyType, delay);

                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(notePreimageTx.signAsync(alice), {
                    allowFailures: true,
                });
                expect(preimageBlock.result?.successful).to.be.false;

                const submitTx = api.tx.referenda.submit(
                    {
                        system: "Root",
                    },
                    { Lookup: { Hash: tx.method.hash.toHex(), len: tx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(submitTx.signAsync(bob), { allowFailures: true });
                expect(submitBlock.result?.successful).to.be.false;
            },
        });
    },
});
