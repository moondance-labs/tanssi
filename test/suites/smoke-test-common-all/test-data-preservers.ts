import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { PalletDataPreserversRegisteredProfile } from "@polkadot/types/lookup";
import { type BlockData, getBlocksDataForPeriodMs } from "../../utils";
import { filterAndApply } from "@moonwall/util";
import type { EventRecord } from "@polkadot/types/interfaces";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 1 * 60 * 1000; // 1 hours in ms
const timeout = Math.max(Math.floor(timePeriod / 12), 15000);

describeSuite({
    id: "S04",
    title: "Verify data preservers consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let paraApi: ApiPromise;
        let registeredProfiles: PalletDataPreserversRegisteredProfile[];
        let runtimeVersion: number;
        let blocksData: BlockData[];

        beforeAll(async () => {
            paraApi = context.polkadotJs("para");
            registeredProfiles = (await paraApi.query.dataPreservers.profiles.entries())
                .filter(([, entry]) => entry.isSome)
                .map(([, entry]) => entry.unwrap());
            runtimeVersion = paraApi.runtimeVersion.specVersion.toNumber();
            blocksData = await getBlocksDataForPeriodMs(paraApi, timePeriod);
        }, timeout);

        it({
            id: "C01",
            title: "all profiles should have a deposit of either 0 or value fixed in the runtime",
            test: async () => {
                const byteFee = 100n * 1_000_000n * 100n; // 10_000_000_000
                const baseFee = 100n * 1_000_000_000n * 100n; // 10_000_000_000_000

                // New deposit has been decreased to 100 times, but we need to check both
                const oldToNewRatio = 100n;

                const calculatedFee = (encodedLength: number) => baseFee + byteFee * BigInt(encodedLength);

                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const profileCreatedEvents = filterAndApply(
                            events,
                            "dataPreservers",
                            ["ProfileCreated"],
                            ({ event }: EventRecord) => event.data.toHuman() as unknown as { profileId: number }
                        );

                        if (!profileCreatedEvents.length) {
                            continue;
                        }

                        for (const profileCreatedEvent of profileCreatedEvents) {
                            log(`Found "ProfileCreated" event for block: ${blockToCheck.blockNum}. Checking...`);

                            const apiAtBlock = await paraApi.at(blockToCheck.blockHash);
                            const profileRecord = await apiAtBlock.query.dataPreservers.profiles(
                                profileCreatedEvent.profileId
                            );
                            const { profile, deposit } = profileRecord.unwrap();

                            const feeOld = calculatedFee(profile.encodedLength);
                            const feeNew = feeOld / oldToNewRatio;

                            expect(
                                deposit.toBigInt() !== feeOld &&
                                    deposit.toBigInt() !== feeNew &&
                                    deposit.toBigInt() !== 0n,
                                `Invalid deposit registered for profile: ${JSON.stringify(profile.toHuman())}`
                            ).toEqual(false);
                        }
                    }
                }
            },
        });

        it({
            id: "C02",
            title: "all assigned profile have assignement witness corresponding to request and whished para id",
            test: async () => {
                for (const { profile, assignment } of registeredProfiles.filter(
                    ({ assignment }) => assignment.isSome
                )) {
                    const [para_id, witness] = assignment.unwrap();

                    if (profile.paraIds.isWhitelist) {
                        expect(profile.paraIds.asWhitelist.has(para_id));
                    } else if (profile.paraIds.isBlacklist) {
                        expect(!profile.paraIds.asBlacklist.has(para_id));
                    }

                    if (profile.assignmentRequest.toString() === "Free") {
                        expect(witness.toString()).to.be.eq("Free");
                    } else if (profile.assignmentRequest.isStreamPayment) {
                        expect(witness.asStreamPayment).not.toBeUndefined();
                    } else {
                        // Make test fail on unknown assignment modes.
                        // This force use to update this test when we add new modes.
                        expect.fail("unknown assignment mode");
                    }
                }
            },
        });

        it({
            id: "C03",
            title: "all profiles should have valid url",
            test: async () => {
                let failures = [];
                // the property has been renamed here: https://github.com/moondance-labs/tanssi/pull/1304
                if (runtimeVersion < 1600) {
                    failures = registeredProfiles.filter(
                        // The type has changed for the current runtime, we ignore it
                        // @ts-ignore
                        ({ profile }) => !isValidEndpointUrl(profile.url.toHuman().toString())
                    );
                } else {
                    type ProfileType = {
                        bootnodeUrl: string | null;
                        directRpcUrls: string[];
                        proxyRpcUrls: string[];
                    };
                    failures = registeredProfiles.filter(({ profile }) => {
                        const profileDecoded = profile.toHuman() as ProfileType;
                        // collect any possible URL-s
                        const urls = [
                            profileDecoded.bootnodeUrl?.toString() || "",
                            ...profileDecoded.directRpcUrls,
                            ...profileDecoded.proxyRpcUrls,
                        ].filter(Boolean);
                        return !!urls.map((url: string) => isValidEndpointUrl(url)).includes(false);
                    });
                }

                for (const { profile } of failures) {
                    log(`Found invalid URL for profile: ${JSON.stringify(profile.toHuman())}`);
                }
                expect(failures.length, `${failures.length} invalid endpoint urls registered`).toBe(0);
            },
        });
    },
});

function isValidEndpointUrl(endpoint: string) {
    const prefixes = ["/dns4/", "https://", "http://", "wss://", "ws://"];
    return prefixes.some((prefix) => endpoint.startsWith(prefix));
}
