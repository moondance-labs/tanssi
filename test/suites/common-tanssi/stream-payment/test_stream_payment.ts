import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DT0501",
    title: "Stream payment works",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Stream payment works",
            test: async function () {
                // 1st block
                let aliceNonce = 0;
                const txOpenStream = await polkadotJs.tx.streamPayment
                    .openStream(
                        bob.address,
                        {
                            timeUnit: "BlockNumber",
                            assetId: "Native",
                            rate: 2_000_000,
                        },
                        10_000_000
                    )
                    .signAsync(alice, { nonce: aliceNonce++ });
                await context.createBlock([txOpenStream]);

                const openStreamEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamOpened";
                });
                expect(openStreamEvents.length).to.be.equal(1);

                // Check opening storage hold
                const openingHold = (await polkadotJs.query.balances.holds(alice.address)).find((h) =>
                    h.id.value.eq("StreamOpened")
                );
                expect(openingHold.amount.toBigInt()).eq(11_730_000_000_000n);

                // 2nd block
                const txPerformPayment = await polkadotJs.tx.streamPayment
                    .performPayment(0)
                    .signAsync(alice, { nonce: aliceNonce++ });

                const txRequestChange = await polkadotJs.tx.streamPayment
                    .requestChange(
                        0,
                        {
                            Mandatory: {
                                deadline: 0,
                            },
                        },
                        {
                            timeUnit: "BlockNumber",
                            assetId: "Native",
                            rate: 1_000_000,
                        },
                        {
                            Increase: 5_000,
                        }
                    )
                    .signAsync(alice, { nonce: aliceNonce++ });

                await context.createBlock([txPerformPayment, txRequestChange]);

                const performPaymentEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamPayment";
                });
                expect(performPaymentEvents.length).to.be.equal(1);

                const requestChangeEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamConfigChangeRequested";
                });
                expect(requestChangeEvents.length).to.be.equal(1);

                // 3rd block
                const txAcceptChange = await polkadotJs.tx.streamPayment
                    .acceptRequestedChange(0, 1, null)
                    .signAsync(bob);
                await context.createBlock([txAcceptChange]);

                const acceptChangeEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamConfigChanged";
                });
                expect(acceptChangeEvents.length).to.be.equal(1);

                // 4rd block
                const txCloseStream = await polkadotJs.tx.streamPayment
                    .closeStream(0)
                    .signAsync(alice, { nonce: aliceNonce++ });

                await context.createBlock([txCloseStream]);

                const closeStreamEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamClosed";
                });
                expect(closeStreamEvents.length).to.be.equal(1);

                // Check all holds have been released
                const holds = await polkadotJs.query.balances.holds(alice.address);
                expect(holds.length).toBe(0);
            },
        });
    },
});
