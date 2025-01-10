import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair, generateKeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { jumpToSession } from "../../../util/block";
import { PRIMARY_GOVERNANCE_CHANNEL_ID } from "../../../util/constants";

describeSuite({
    id: "DTR1307",
    title: "Slashes are accumulated based on max slashes sent per block",
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
            title: "Slashes are accumulated across blocks",
            test: async function () {
                // we need to start at least one sesssion to start eras
                await jumpToSession(context, 1);
                // Let's inject slashes N+1 slashes, where N is the max slashes to send per block
                // With force inject slash, we can inject a slash for any account
                const maxSlashesPerMessage = (await polkadotJs.consts.externalValidatorSlashes.queuedSlashesProcessedPerBlock).toNumber();
                const slashesToInject = maxSlashesPerMessage +1;
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

                for (let i = 0; i < slashesToInject; i++) {
                    const randomAccount = generateKeyringPair("sr25519");
                    await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.forceInjectSlash(0, randomAccount.address, 1000))
                    .signAndSend(alice, { nonce: aliceNonce++ });
                }
                await context.createBlock();

                // Slash item should be there
                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(slashesToInject);

                const sessionsPerEra = await polkadotJs.consts.externalValidators.sessionsPerEra;

                const currentIndex = await polkadotJs.query.session.currentIndex();

                const targetSession = currentIndex * (sessionsPerEra * (DeferPeriod + 1));

                await jumpToSession(context, targetSession);

                // scheduled slashes
                const expectedSlashesAfterDefer = await polkadotJs.query.externalValidatorSlashes.slashes(
                    DeferPeriod + 1
                );
                // We should have unprocessed messages
                const expectedUnprocessedMessages = await polkadotJs.query.externalValidatorSlashes.unreportedSlashesQueue();
                expect (expectedUnprocessedMessages.length).to.be.eq(slashesToInject);
                expect(expectedSlashesAfterDefer.length).to.be.eq(slashesToInject);
                expect(expectedSlashesAfterDefer[0].confirmed.toHuman()).to.be.true;

                // In the next block we should send the slashes. For this we will confirm:
                // A: that the unprocessed slashes decrease
                // B: that the nonce of the primary channel increases
                const primaryChannelNonceBefore = await polkadotJs.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID)

                await context.createBlock();
                const expectedUnprocessedMessagesAfterOneBlock = await polkadotJs.query.externalValidatorSlashes.unreportedSlashesQueue();
                const primaryChannelNonceAfter = await polkadotJs.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);
                expect (primaryChannelNonceAfter.toBigInt()).toBe(primaryChannelNonceBefore.toBigInt()+ 1n);
                // However we stil should have one unprocessed message
                expect (expectedUnprocessedMessagesAfterOneBlock.length).to.be.eq(1);
            },
        });
    },
});
