import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";
import { jumpToSession } from "../../../util/block";

describeSuite({
    id: "DTR1308",
    title: "Slashes are accumulated across eras based on max slashes sent per block",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Slashes are accumulated across eras",
            test: async function () {
                // we need to start at least one sesssion to start eras
                await jumpToSession(context, 1);
                // Let's inject slashes N+1 slashes, where N is the max slashes to send per block
                // With force inject slash, we can inject a slash for any account
                const maxSlashesPerMessage = (
                    await polkadotJs.consts.externalValidatorSlashes.queuedSlashesProcessedPerBlock
                ).toNumber();
                const epochDuration = (await polkadotJs.consts.babe.epochDuration).toNumber();
                const sessionsPerEra = await polkadotJs.consts.externalValidators.sessionsPerEra;

                const slashesToInject = maxSlashesPerMessage * epochDuration * sessionsPerEra.toNumber() + 1;
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

                for (let i = 0; i < slashesToInject; i++) {
                    const randomAccount = generateKeyringPair("sr25519");
                    await polkadotJs.tx.sudo
                        .sudo(
                            polkadotJs.tx.externalValidatorSlashes.forceInjectSlash(0, randomAccount.address, 1000, 1)
                        )
                        .signAndSend(alice, { nonce: aliceNonce++ });
                }
                await context.createBlock();

                // Slash item should be there
                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(slashesToInject);

                const currentIndex = await polkadotJs.query.session.currentIndex();

                const targetSession = currentIndex * (sessionsPerEra * (DeferPeriod + 1));

                await jumpToSession(context, targetSession);

                // scheduled slashes
                const expectedSlashesAfterDefer = await polkadotJs.query.externalValidatorSlashes.slashes(
                    DeferPeriod + 1
                );
                // We should have unprocessed messages
                const expectedUnprocessedMessages =
                    await polkadotJs.query.externalValidatorSlashes.unreportedSlashesQueue();
                expect(expectedUnprocessedMessages.length).to.be.eq(slashesToInject);
                expect(expectedSlashesAfterDefer.length).to.be.eq(slashesToInject);
                expect(expectedSlashesAfterDefer[0].confirmed.toHuman()).to.be.true;

                // Now we will jump one entire era
                // After one era, we should still have one slash to send
                const currentIndexAfterSlashes = await polkadotJs.query.session.currentIndex();
                const targetSessionToNextEra = currentIndexAfterSlashes.toNumber() + sessionsPerEra.toNumber();
                console.log(currentIndexAfterSlashes.toBigInt());
                console.log(sessionsPerEra.toBigInt());
                console.log(targetSessionToNextEra);

                await jumpToSession(context, targetSessionToNextEra);

                // However we stil should have one unprocessed message
                const expectedUnprocessedMessagesAfterOneEra =
                    await polkadotJs.query.externalValidatorSlashes.unreportedSlashesQueue();
                expect(expectedUnprocessedMessagesAfterOneEra.length).to.be.eq(1);
            },
        });
    },
});
