import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect, customDevRpcRequest } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { nToHex } from "@polkadot/util";

async function rpcStreamPaymentStatus(context, block, streamId, now) {
    if (block == "latest") {
        const blockNumber = (
            await context.polkadotJs().rpc.chain.getBlock()
          ).block.header.number.toBigInt();
  
        const blockHash = await context.polkadotJs().rpc.chain.getBlockHash(blockNumber);

        block = blockHash;
    }

    await customDevRpcRequest("tanssi_streamPaymentStatus", [
        block,
        streamId,
        now,
    ]);
}

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
                try {
                    await rpcStreamPaymentStatus(context, "latest", 0, null);
                    throw { message: "Should have returned an error" }
                } catch(e: any) {
                    expect(e.message.toString()).to.eq("Failed to fetch stream payment status: Unknown stream id")
                }

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

                console.log(`After first block: ${await rpcStreamPaymentStatus(context, "latest", 0, null)}`);
                    
                // 2nd block
                const txPerformPayment = await polkadotJs.tx.streamPayment
                    .performPayment(0)
                    .signAsync(alice, { nonce: aliceNonce++ });

                const txRequestChange = await polkadotJs.tx.streamPayment
                    .requestChange(
                        0,
                        {
                            Mandatory: {
                                deadline: 10,
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
            },
        });
    },
});
