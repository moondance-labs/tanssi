import "@tanssi/api-augment/dancelight";
import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { Keyring, type ApiPromise } from "@polkadot/api";
import { SubmittableModuleExtrinsics } from "@polkadot/api-base/types";
import { ApiTypes } from "@polkadot/api-base/types";

import type { KeyringPair } from "@polkadot/keyring/types";
import { type MultiLocation } from "utils";
import { PalletXcmQueryStatus } from "@polkadot/types/lookup";
import { VersionedMultiLocation } from "@polkadot/types/interfaces";
import { xcm } from "@polkadot/types/interfaces/definitions";

const MAX_BALANCE_TRANSFER_TRIES = 5;
describeSuite({
    id: "RT01",
    title: "Chopsticks Dancebox Upgrade Test",
    foundationMethods: "chopsticks",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let specName: string;
        let alice: KeyringPair;
        let runtimeName: String;
        let xcmQueryToAnalyze: Number;
        let xcmPalletAlias: SubmittableModuleExtrinsics<ApiTypes>;

        beforeAll(async () => {
            api = context.pjsApi;
            specName = api.consts.system.version.specName.toString();
            const specVersion = getSpecVersion(api);
            log(`Currently connected to chain: ${specName} : ${specVersion}`);
            runtimeName = api.runtimeVersion.specName.toString();

            const keyring = new Keyring({ type: "sr25519" });
            context.keyring.alice;
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            if (runtimeName != "flashbox") {
                // Inject a query so that there are migrations to execute in queries in case of new xcm version
                // Right now we need to hardcode the xcm version
                // this should not be necessary after we bring https://github.com/paritytech/polkadot-sdk/pull/8173
                // since we would be able to fetch it on-chain
                const queryLocation = runtimeName.includes("light")
                    ? {
                          parents: 0,
                          interior: { X1: [{ Parachain: 1 }] },
                      }
                    : {
                          parents: 1,
                          interior: "Here",
                      };

                // fetch on-chain later
                const previousXcmVersion = 5;
                const latestVersion = "V" + previousXcmVersion.toString();

                const versionedLocation = new Object();
                versionedLocation[latestVersion] = queryLocation;

                xcmQueryToAnalyze = runtimeName.includes("light")
                    ? await api.query.xcmPallet.queryCounter()
                    : await api.query.polkadotXcm.queryCounter();

                let batchTx = [];
                if (runtimeName.includes("light")) {
                    // We first inject a random paras head in parachain 1. this is necessary as we need a chain to which
                    // we send queries
                    batchTx.push(api.tx.registrar.setCurrentHead(1, "0x11"));
                    batchTx.push(api.tx.xcmPallet.forceSubscribeVersionNotify(versionedLocation));
                } else if (runtimeName != "flasbox") {
                    batchTx.push(api.tx.polkadotXcm.forceSubscribeVersionNotify(versionedLocation));
                }

                let tries = 0;

                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await api.tx.sudo.sudo(api.tx.utility.batchAll(batchTx)).signAndSend(alice);
                    const result = await context.createBlock({ count: 1 });

                    const block = await api.rpc.chain.getBlock(result.result);
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    tries++;
                }
            }
        });

        it({
            id: "T1",
            timeout: 80000,
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
            title: "Xcm migrations have runned, if any",
            test: async ({ skip }) => {
                if (runtimeName === "flashbox") {
                    skip();
                }
                // Regardless of migrations, query should be in the latest version
                // Unfortunately the api is not very friendly
                const currentXcmVersion = runtimeName.includes("light")
                    ? (await api.consts.xcmPallet.advertisedXcmVersion).toNumber()
                    : (await api.consts.polkadotXcm.advertisedXcmVersion).toNumber();

                const query = runtimeName.includes("light")
                    ? await api.query.xcmPallet.queries(xcmQueryToAnalyze)
                    : await api.query.polkadotXcm.queries(xcmQueryToAnalyze);
                const version = Object.keys(query.toJSON()["versionNotifier"]["origin"])[0];
                expect(version).to.be.equal("v" + currentXcmVersion.toString());
            },
        });

        it({
            id: "T5",
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

                if (specName === "dancelight") {
                    const newPendingAssignmentEvent = blockEvents
                        .filter(({ event }) => api.events.tanssiCollatorAssignment.NewPendingAssignment.is(event))
                        .map(
                            ({ event }) =>
                                api.events.tanssiCollatorAssignment.NewPendingAssignment.is(event) && event.data
                        )[0];
                    expect(
                        newPendingAssignmentEvent,
                        "collatorAssignment.NewPendingAssignment event should be emitted on session change"
                    ).toBeTruthy();
                    expect(
                        newPendingAssignmentEvent.targetSession.toNumber(),
                        "Session index should be incremented"
                    ).toBe(newSessionIndex + 1);
                } else {
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
