import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { filterAndApply } from "@moonwall/util";

import type { EventRecord } from "@polkadot/types/interfaces";
import { type BlockData, getBlocksDataForPeriodMs } from "../../utils";
import { calculateIdentityDeposit, calculateSubIdentityDeposit } from "../../utils/identity.ts";
import type { u128 } from "@polkadot/types-codec";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);

describeSuite({
    id: "S10",
    title: "Verify identities deposits",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let api: ApiPromise;
        let blocksData: BlockData[];

        beforeAll(async () => {
            api = context.polkadotJs();

            blocksData = await getBlocksDataForPeriodMs(api, timePeriod);
        }, timeout);

        it({
            id: "C01",
            title: "Set Identity transaction holds the deposit",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const identitySetEvents = filterAndApply(
                            events,
                            "identity",
                            ["IdentitySet"],
                            ({ event }: EventRecord) => event.data.toHuman() as unknown as { who: string }
                        );

                        if (!identitySetEvents.length) {
                            continue;
                        }

                        for (const identitySetEvent of identitySetEvents) {
                            log(`Found "IdentitySet" event for block: ${blockToCheck.blockNum}. Checking...`);

                            const prevBlockHash = await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1);
                            const prevApiAtBlock = await api.at(prevBlockHash);
                            const apiAtBlock = await api.at(blockToCheck.blockHash);

                            const prevIdentity = await prevApiAtBlock.query.identity.identityOf(identitySetEvent.who);
                            const identity = await apiAtBlock.query.identity.identityOf(identitySetEvent.who);

                            const registration = identity.unwrap() as unknown as { info: unknown; deposit: u128 };

                            const expectedAmount = calculateIdentityDeposit(api, registration.info);

                            const reserved = filterAndApply(
                                events,
                                "balances",
                                ["Reserved"],
                                ({ event }: EventRecord) => event.data as unknown as unknown as { amount: u128 }
                            );

                            // If the identity was not set before, we expect the deposit to be equal 0
                            const prevDeposit = prevIdentity.isNone
                                ? 0n
                                : (
                                      prevIdentity.unwrap() as unknown as {
                                          info: unknown;
                                          deposit: u128;
                                      }
                                  ).deposit.toBigInt();

                            const actuallyReserved = reserved[0]?.amount.toBigInt() || 0n; // In case we update the identity info, we don't pay
                            expect(
                                actuallyReserved + prevDeposit,
                                `Block #${blockToCheck.blockNum}. Expecting actuallyReserved + identityDeposit: ${actuallyReserved + prevDeposit} to equal expectedAmount: ${expectedAmount}`
                            ).toEqual(expectedAmount);
                        }
                    }
                }
            },
        });

        it({
            id: "C02",
            title: "Clear Identity transaction unholds the deposit",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const identityClearedEvents = filterAndApply(
                            events,
                            "identity",
                            ["IdentityCleared"],
                            ({ event }: EventRecord) => event.data.toHuman() as unknown as { who: string }
                        );

                        if (!identityClearedEvents.length) {
                            continue;
                        }

                        for (const identityClearedEvent of identityClearedEvents) {
                            log(`Found "IdentityCleared" event for block: ${blockToCheck.blockNum}. Checking...`);

                            const prevBlockHash = await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1);
                            const prevApiAtBlock = await api.at(prevBlockHash);

                            const prevIdentity = await prevApiAtBlock.query.identity.identityOf(
                                identityClearedEvent.who
                            );
                            const unreserved = filterAndApply(
                                events,
                                "balances",
                                ["Unreserved"],
                                ({ event }: EventRecord) => event.data as unknown as unknown as { amount: u128 }
                            );

                            const prevUnwrappedIdentity = prevIdentity.unwrap() as unknown as {
                                info: unknown;
                                deposit: u128;
                            };
                            const expectedAmount = calculateIdentityDeposit(api, prevUnwrappedIdentity.info);

                            expect(unreserved.length).toBeGreaterThan(0);
                            const actuallyUnreserved = unreserved[0]?.amount.toBigInt();
                            expect(
                                actuallyUnreserved,
                                `Block #${blockToCheck.blockNum}. Expecting actuallyUnreserved: ${expectedAmount} to equal expectedAmount: ${expectedAmount}`
                            ).toEqual(expectedAmount);
                        }
                    }
                }
            },
        });

        it({
            id: "C03",
            title: "Set SubIdentity transaction holds the deposit",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const subIdentitySetEvents = filterAndApply(
                            events,
                            "identity",
                            ["SubIdentitiesSet"],
                            ({ event }: EventRecord) => event.data.toHuman() as unknown as { main: string }
                        );

                        if (!subIdentitySetEvents.length) {
                            continue;
                        }

                        for (const subIdentitySetEvent of subIdentitySetEvents) {
                            log(`Found "SubIdentitiesSet" event for block: ${blockToCheck.blockNum}. Checking...`);

                            const prevBlockHash = await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1);
                            const prevApiAtBlock = await api.at(prevBlockHash);
                            const apiAtBlock = await api.at(blockToCheck.blockHash);

                            const prevSubs = await prevApiAtBlock.query.identity.subsOf(subIdentitySetEvent.main);
                            const subs = await apiAtBlock.query.identity.subsOf(subIdentitySetEvent.main);

                            const diff = subs.toJSON()[1].length - prevSubs.toJSON()[1].length;
                            const diffAbs = Math.abs(diff);

                            const expectedAmount = calculateSubIdentityDeposit(api, diffAbs);

                            if (diff > 0) {
                                const reserved = filterAndApply(
                                    events,
                                    "balances",
                                    ["Reserved"],
                                    ({ event }: EventRecord) => event.data as unknown as unknown as { amount: u128 }
                                );

                                expect(
                                    expectedAmount,
                                    `Block #${blockToCheck.blockNum}. Expecting expectedAmount: ${expectedAmount} to equal reserved: ${reserved}`
                                ).toEqual(expectedAmount);
                            } else {
                                const unreserved = filterAndApply(
                                    events,
                                    "balances",
                                    ["Unreserved"],
                                    ({ event }: EventRecord) => event.data as unknown as unknown as { amount: u128 }
                                );

                                expect(
                                    expectedAmount,
                                    `Block #${blockToCheck.blockNum}. Expecting expectedAmount: ${expectedAmount} to equal unreserved: ${unreserved}`
                                ).toEqual(expectedAmount);
                            }
                        }
                    }
                }
            },
        });

        it({
            id: "C04",
            title: "Check if all the identities have reserved deposits",
            test: async () => {
                const limit = 1000;
                let last_key = "";
                let count = 0;

                const identityPerAccount: Map<string, { deposit: bigint }> = new Map();
                for (;;) {
                    const query = await api.query.identity.identityOf.entriesPaged({
                        args: [],
                        pageSize: limit,
                        startKey: last_key,
                    });

                    if (query.length === 0) {
                        break;
                    }
                    count += query.length;

                    for (const [storageKey, optIdentity] of query) {
                        last_key = storageKey.toString();

                        if (optIdentity.isSome) {
                            const identity = optIdentity.unwrap() as unknown as { deposit: u128 };
                            const accountId = storageKey.args[0].toString();

                            identityPerAccount.set(accountId, {
                                deposit: identity.deposit.toBigInt(),
                            });
                        }
                    }

                    if (count % (10 * limit) === 0) {
                        log(`Retrieved ${count} subs of identity`);
                    }
                }

                for (const accountId of identityPerAccount.keys()) {
                    const accountData = await api.query.system.account(accountId);
                    const reserved = accountData.data.reserved.toBigInt();

                    expect(
                        reserved,
                        `Reserved balance: ${reserved} for account ${accountId} should be more or equal to deposit: ${identityPerAccount.get(accountId).deposit}`
                    ).toBeGreaterThanOrEqual(identityPerAccount.get(accountId).deposit);
                }
            },
        });
    },
});
