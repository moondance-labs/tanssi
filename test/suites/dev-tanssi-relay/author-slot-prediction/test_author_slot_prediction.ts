import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions, jumpToSession } from "../../../util/block";
import { u8aToHex, stringToHex } from "@polkadot/util";
import { Keyring } from "@polkadot/keyring";
const includesAll = (arr, values) => values.every((v) => arr.includes(v));

describeSuite({
    id: "DEVT0101",
    title: "Session keys assignment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let eve: KeyringPair;
        let aliceStash: KeyringPair;
        beforeAll(() => {
            const keyring = new Keyring({ type: "sr25519" });
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            eve = keyring.addFromUri("//Eve");
            aliceStash = keyring.addFromUri("//Alice//stash");
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that authority assignment is correct in the first assignment",
            test: async () => {
                // We need at least a couple of session to start seeing the first assignment
                // The reason is that the session pallet goes before the tanssiInvulnerables in the dancelight runtime
                // Otherwise we would not need this (as in dancebox)
                await jumpToSession(context, 2);

                // for session 2
                const assignment2 = (await polkadotJs.query.tanssiAuthorityAssignment.collatorContainerChain(2))
                    .unwrap()
                    .toHuman();
                const assignment3 = (await polkadotJs.query.tanssiAuthorityAssignment.collatorContainerChain(3))
                    .unwrap()
                    .toHuman();
                expect(assignment2.orchestratorChain).to.deep.equal([]);
                const allKeys = [
                    u8aToHex(bob.publicKey),
                    u8aToHex(charlie.publicKey),
                    u8aToHex(dave.publicKey),
                    u8aToHex(eve.publicKey),
                ];

                // the keys are assigned randomly but we check all of them exist in allKeys
                expect(includesAll(allKeys, assignment3.containerChains["2000"])).to.be.true;
                expect(includesAll(allKeys, assignment3.containerChains["2001"])).to.be.true;

                // Session 3 is the same as session 2
                expect(assignment2).to.deep.equal(assignment3);
                // Session 4 is empty
                expect((await polkadotJs.query.tanssiAuthorityAssignment.collatorContainerChain(4)).isNone).to.be.true;

                // Check authorities are correct

                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();

                const authorities = (
                    await context.polkadotJs().query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                )
                    .unwrap()
                    .toHuman();

                expect(includesAll(allKeys, authorities.containerChains["2000"])).to.be.true;
            },
        });

        it({
            id: "E02",
            title: "Checking session key changes are reflected at the session length boundary block",
            test: async () => {
                const newKey = await polkadotJs.rpc.author.rotateKeys();
                const newKeyCharlie = await polkadotJs.rpc.author.rotateKeys();

                // from alice and also charlie, to make sure the next authorities are changed
                await polkadotJs.tx.session.setKeys(newKey, []).signAndSend(aliceStash);
                await polkadotJs.tx.session.setKeys(newKeyCharlie, []).signAndSend(charlie);

                await context.createBlock();
                // Check key is reflected in next key
                // But its not yet in queued
                const queuedKeys = await polkadotJs.query.session.queuedKeys();
                const result = queuedKeys.filter((keyItem) => keyItem[1].nimbus === newKey);
                expect(result).is.empty;
                const nextKey = await polkadotJs.query.session.nextKeys(aliceStash.address);
                const nextKeyCharlie = await polkadotJs.query.session.nextKeys(charlie.address);
                // the last 32 are the nimbus key
                expect(u8aToHex(nextKey.unwrap().nimbus)).to.be.eq(u8aToHex(newKey.slice(-32)));
                expect(u8aToHex(nextKeyCharlie.unwrap().nimbus)).to.be.eq(u8aToHex(newKeyCharlie.slice(-32)));

                // Let's jump 2 sessiona
                await jumpSessions(context, 2);

                // The very first block produced by the second session should contain the new key

                const babeAuthorities = await polkadotJs.query.babe.authorities();
                const nextBabeKey = nextKey.unwrap().babe;

                expect(babeAuthorities[0].includes(nextBabeKey));
                // The change should have been applied, and now both nimbus and authorityMapping should reflect
                const digests = (await polkadotJs.query.system.digest()).logs;
                const filtered = digests.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BABE")
                );

                expect(filtered[0].asConsensus[1].toHex().includes(nextBabeKey.toHex().slice(2))).to.be.true;

                // Charlie should have his changed reflected in either 2000 or 2001
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();

                const authorities = (
                    await context.polkadotJs().query.tanssiAuthorityAssignment.collatorContainerChain(sessionIndex)
                )
                    .unwrap()
                    .toHuman();
                expect(
                    authorities.containerChains["2000"].includes(u8aToHex(newKeyCharlie.slice(-32))) ||
                        authorities.containerChains["2001"].includes(u8aToHex(newKeyCharlie.slice(-32)))
                ).to.be.true;
            },
        });
    },
});
