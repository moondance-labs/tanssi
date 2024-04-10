import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect, customDevRpcRequest } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

async function rpcStreamPaymentStatus(context, block, streamId, now) {
    if (block == "latest") {
        const blockNumber = (await context.polkadotJs().rpc.chain.getBlock()).block.header.number.toBigInt();

        const blockHash = await context.polkadotJs().rpc.chain.getBlockHash(blockNumber);

        block = blockHash;
    }

    return await customDevRpcRequest("tanssi_streamPaymentStatus", [block, streamId, now]);
}

describeSuite({
    id: "DT0502",
    title: "Stream payment RPC",
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
            title: "Stream payment RPC",
            test: async function () {
                try {
                    await rpcStreamPaymentStatus(context, "latest", 0, null);
                    throw { message: "Should have returned an error" };
                } catch (e: any) {
                    expect(e.message.toString()).to.eq("Failed to fetch stream payment status: Unknown stream id");
                }

                // 1st block
                let aliceNonce = 0;
                const txOpenStream = await polkadotJs.tx.streamPayment
                    .openStream(
                        bob.address,
                        {
                            timeUnit: "BlockNumber",
                            assetId: "Native",
                            rate: 100_000,
                        },
                        10_000_000
                    )
                    .signAsync(alice, { nonce: aliceNonce++ });
                let newBlock = await context.createBlock([txOpenStream]);

                const openStreamEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamOpened";
                });
                expect(openStreamEvents.length).to.be.equal(1);

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 10_000_000,
                    stalled: false,
                    payment: 0,
                });

                // 2nd block: create an empty block to check status
                newBlock = await context.createBlock();

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 9_900_000,
                    stalled: false,
                    payment: 100_000,
                });

                // 3nd block
                const txPerformPayment = await polkadotJs.tx.streamPayment
                    .performPayment(0)
                    .signAsync(alice, { nonce: aliceNonce++ });

                const txRequestChange = await polkadotJs.tx.streamPayment
                    .requestChange(
                        0,
                        {
                            Mandatory: {
                                deadline: 2, // deadline in the past, which should make the stream stalled
                            },
                        },
                        {
                            timeUnit: "BlockNumber",
                            assetId: "Native",
                            rate: 50_000,
                        },
                        {
                            Increase: 5_000,
                        }
                    )
                    .signAsync(alice, { nonce: aliceNonce++ });

                newBlock = await context.createBlock([txPerformPayment, txRequestChange]);

                const performPaymentEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamPayment";
                });
                expect(performPaymentEvents.length).to.be.equal(1);

                const requestChangeEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamConfigChangeRequested";
                });
                expect(requestChangeEvents.length).to.be.equal(1);

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 9_800_000,
                    stalled: true,
                    payment: 0,
                });

                // 4th block: create an empty block to check status
                newBlock = await context.createBlock();

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 9_800_000,
                    stalled: true,
                    payment: 0,
                });

                // produce empty block on session change, which cannot contain extrinsics
                await context.createBlock();

                // 6th block: accept change, resuming stream
                const txAcceptChange = await polkadotJs.tx.streamPayment
                    .acceptRequestedChange(0, 1, null)
                    .signAsync(bob);
                newBlock = await context.createBlock([txAcceptChange]);

                const acceptChangeEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamConfigChanged";
                });
                expect(acceptChangeEvents.length).to.be.equal(1);

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 9_805_000, // old deposit + increase
                    stalled: false,
                    payment: 0,
                });

                // 7th block: create an empty block to check status
                newBlock = await context.createBlock();

                expect(await rpcStreamPaymentStatus(context, newBlock.block.hash, 0, null)).to.deep.equal({
                    deposit_left: 9_755_000,
                    stalled: false,
                    payment: 50_000,
                });

                // 8th block: close the stream
                const txCloseStream = await polkadotJs.tx.streamPayment
                    .closeStream(0)
                    .signAsync(alice, { nonce: aliceNonce++ });

                await context.createBlock([txCloseStream]);

                const closeStreamEvents = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "StreamClosed";
                });
                expect(closeStreamEvents.length).to.be.equal(1);

                try {
                    await rpcStreamPaymentStatus(context, "latest", 0, null);
                    throw { message: "Should have returned an error" };
                } catch (e: any) {
                    expect(e.message.toString()).to.eq("Failed to fetch stream payment status: Unknown stream id");
                }
            },
        });
    },
});
