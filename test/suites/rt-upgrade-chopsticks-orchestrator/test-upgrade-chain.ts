import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { Keyring, type ApiPromise } from "@polkadot/api";

const MAX_BALANCE_TRANSFER_TRIES = 5;
describeSuite({
    id: "RT01",
    title: "Chopsticks Dancebox Upgrade Test",
    foundationMethods: "chopsticks",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let specName: string;

        beforeAll(async () => {
            api = context.pjsApi;
            specName = api.consts.system.version.specName.toString();
            const specVersion = getSpecVersion(api);
            log(`Currently connected to chain: ${specName} : ${specVersion}`);
        });

        it({
            id: "T1",
            timeout: 60000,
            title: "Can upgrade runtime",
            test: async () => {
                const rtBefore = getSpecVersion(api);
                const sessionBefore = api.query.session.currentIndex();
                log("About to upgrade to runtime at:");
                log((await MoonwallContext.getContext()).rtUpgradePath);

                await context.upgradeRuntime();
                const sessionAfter = api.query.session.currentIndex();

                // New sessions can lead to the runtime upgrade not being correctly applied
                // Hence we retry once more just in case
                if (
                    (await sessionAfter).toNumber() > (await sessionBefore).toNumber() &&
                    rtBefore === getSpecVersion(api)
                ) {
                    log("New session encountered, just in case retrying");
                    await context.upgradeRuntime();
                }

                // console.log( api.call.tanssiUtilApi)

                const rtafter = getSpecVersion(api);

                expect(rtBefore, `RT Upgrade has not been applied, before: ${rtBefore}, after: ${rtafter}`).not.toBe(
                    rtafter
                );
            },
        });

        it({
            id: "T2",
            title: "Can create new blocks",
            test: async () => {
                const currentHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock({ count: 2 });
                const newHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(newHeight - currentHeight).to.be.equal(2);
            },
        });

        it({
            id: "T3",
            title: "Can send balance transfers",
            test: async () => {
                const randomAccount = generateKeyringPair("sr25519");
                const keyring = new Keyring({ type: "sr25519" });
                context.keyring.alice;
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                let tries = 0;
                const balanceBefore = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// Chopsticks does not have the notion of tx pool either, so we need to retry
                /// Therefore we just retry at most MAX_BALANCE_TRANSFER_TRIES
                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await api.tx.balances
                        .transferAllowDeath(randomAccount.address, 1_000_000_000)
                        .signAndSend(alice);
                    const result = await context.createBlock({ count: 1 });

                    const block = await api.rpc.chain.getBlock(result.result);
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    tries++;
                }

                const balanceAfter = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });

        it({
            id: "T4",
            timeout: 60000,
            title: "Skip blocks until session change",
            test: async () => {
                const queuedValidators = (await api.query.session.queuedKeys())
                    .map(([account, _]) => account.toHuman())
                    .sort();

                const { finalBlockNumber } = await skipToSessionChange();

                const newValidators = (await api.query.session.validators()).map((v) => v.toHuman()).sort();
                expect(
                    queuedValidators,
                    "Queued validators should match new validators after session change"
                ).toMatchObject(newValidators);

                const blockHash = await api.rpc.chain.getBlockHash(finalBlockNumber);
                const block = await api.rpc.chain.getBlock(blockHash);
                const signedExtrinsics = block.block.extrinsics.filter((x) => x.isSigned);
                expect(signedExtrinsics.length, "SessionChange block should never have extrinsics inside").toBe(0);

                const blockEvents = await api.query.system.events();
                const sessionChangeEvent = blockEvents
                    .filter(({ event }) => api.events.session.NewSession.is(event))
                    .map(({ event }) => api.events.session.NewSession.is(event) && event.data)[0];
                expect(sessionChangeEvent, "Session change event should be emitted").toBeTruthy();

                const newSessionIndex = (await api.query.session.currentIndex()).toNumber();
                expect(sessionChangeEvent.sessionIndex.toNumber()).toBe(newSessionIndex);

                //  Dancelight does not have collatorAssignment events
                if (specName !== "dancelight") {
                    const newPendingAssignmentEvent = blockEvents
                        .filter(({ event }) => api.events.collatorAssignment.NewPendingAssignment.is(event))
                        .map(
                            ({ event }) => api.events.collatorAssignment.NewPendingAssignment.is(event) && event.data
                        )[0];
                    expect(
                        newPendingAssignmentEvent,
                        "collatorAssignment.NewPendingAssignment event should be emitted on session change"
                    ).toBeTruthy();
                    expect(
                        newPendingAssignmentEvent.targetSession.toNumber(),
                        "Session index should be incremented"
                    ).toBe(newSessionIndex + 1);
                }
            },
        });

        // TODO: Get session period from runtime helper when merged
        const skipToSessionChange = async () => {
            const currentSession = await api.query.session.currentIndex();
            log(`Current session: ${currentSession}`);
            const initialBlockNumber = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
            for (;;) {
                await context.createBlock({ count: 1 });
                const sessionAfter = await api.query.session.currentIndex();
                if (sessionAfter.toNumber() > currentSession.toNumber()) {
                    break;
                }
            }
            const finalBlockNumber = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
            const blocksSkipped = finalBlockNumber - initialBlockNumber;
            log(`Skipped ${blocksSkipped} blocks to reach session change`);
            return { finalBlockNumber, blocksSkipped };
        };
    },
});

const getSpecVersion = (api: ApiPromise) => {
    return api.consts.system.version.specVersion.toNumber();
};
