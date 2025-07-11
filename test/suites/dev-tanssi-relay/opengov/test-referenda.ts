import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { isDancelightRuntime, isStarlightRuntime } from "../../../utils/runtime.ts";
import { BN } from "@polkadot/util";

export type ExtrinsicFailedEventDataType = {
    dispatchError: {
        Module: {
            index: string;
            error: string;
        };
    };
};

export type SubmittedEventDataType = {
    index: number;
};

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
                if (!isDancelightRuntime(api)) {
                    skip();
                }

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

                // Step 5: Voting
                for (const voter of [alice, bob, dave]) {
                    const voteSubmit = await context.createBlock(
                        await api.tx.convictionVoting
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
                if (!isDancelightRuntime(api)) {
                    skip();
                }

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
                if (!isStarlightRuntime(api)) {
                    skip();
                }

                const delegate = alice.address;
                const proxyType = "Any";
                const delay = 0;

                const tx = api.tx.proxy.addProxy(delegate, proxyType, delay);

                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(await notePreimageTx.signAsync(alice), {
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

                const submitBlock = await context.createBlock(await submitTx.signAsync(bob), { allowFailures: true });
                expect(submitBlock.result?.successful).to.be.false;
            },
        });

        it({
            id: "E04",
            title: "Only Root track is enabled",
            test: async ({ skip }) => {
                if (!isDancelightRuntime(api)) {
                    skip();
                }

                const tx = api.tx.system.remark("0x0001");

                // Step 1: Let's create preimage
                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(await notePreimageTx.signAsync(eve));
                expect(preimageBlock.result?.successful).to.be.true;

                // Step 2: Alice submits referenda for not existing track
                const submitTx = api.tx.referenda.submit(
                    {
                        Origins: "WhitelistedCaller",
                    },
                    { Lookup: { Hash: tx.method.hash.toHex(), len: tx.method.encodedLength } },
                    { After: "1" }
                );

                const submitBlock = await context.createBlock(await submitTx.signAsync(alice));
                expect(submitBlock.result?.successful).to.be.false;

                const metadata = await api.rpc.state.getMetadata();
                const referendaPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Referenda")
                    .index.toString();

                const errorData = submitBlock.result.events
                    .find((e) => e.event.method === "ExtrinsicFailed")
                    .event.toHuman().data as unknown as ExtrinsicFailedEventDataType;
                expect(errorData.dispatchError.Module.index).toEqual(referendaPalletIndex);

                const errorBytes = Uint8Array.from(Buffer.from(errorData.dispatchError.Module.error.slice(2), "hex"));
                const errorIndex = errorBytes[0];

                const errorMeta = api.registry.findMetaError({
                    index: new BN(errorData.dispatchError.Module.index),
                    error: new BN(errorIndex),
                });

                expect(errorMeta.method).toEqual("NoTrack");
            },
        });

        it({
            id: "E05",
            title: "Referenda without votes will be rejected",
            test: async ({ skip }) => {
                if (!isDancelightRuntime(api)) {
                    skip();
                }

                const tx = api.tx.system.remark("0x0002");

                // Step 1: Let's create preimage
                const notePreimageTx = api.tx.preimage.notePreimage(tx.method.toHex());
                const preimageBlock = await context.createBlock(await notePreimageTx.signAsync(eve));
                expect(preimageBlock.result?.successful).to.be.true;

                // Step 2: Alice submits referenda for not existing track
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
                const events = await api.query.system.events();
                const proposalIndex = (
                    events.find((e) => e.event.method === "Submitted").event.toHuman()
                        .data as unknown as SubmittedEventDataType
                ).index;
                expect(proposalIndex).to.not.be.undefined;

                // Step 4: Place decision deposit
                const depositSubmit = await context.createBlock(
                    await api.tx.referenda.placeDecisionDeposit(proposalIndex).signAsync(ferdie)
                );
                expect(depositSubmit.result?.successful).to.be.true;

                const expectedEvents = [
                    { section: "referenda", method: "DecisionStarted" },
                    { section: "referenda", method: "Rejected" },
                ];

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
    },
});
