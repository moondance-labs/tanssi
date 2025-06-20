import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { PalletDataPreserversRegisteredProfile } from "@polkadot/types/lookup";

describeSuite({
    id: "S04",
    title: "Verify data preservers consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let paraApi: ApiPromise;
        let registeredProfiles: PalletDataPreserversRegisteredProfile[];

        beforeAll(async () => {
            paraApi = context.polkadotJs("para");
            registeredProfiles = (await paraApi.query.dataPreservers.profiles.entries())
                .filter(([, entry]) => entry.isSome)
                .map(([, entry]) => entry.unwrap());
        });

        it({
            id: "C01",
            title: "all profiles should have a deposit of either 0 or value fixed in the runtime",
            test: async () => {
                const byteFee = 100n * 1_000_000n * 100n; // 10_000_000_000
                const baseFee = 100n * 1_000_000_000n * 100n; // 10_000_000_000_000

                // New deposit has been decreased to 100 times, but we need to check both
                const oldToNewRatio = 100n;

                const calculatedFee = (encodedLength: number) => baseFee + byteFee * BigInt(encodedLength);

                const failures = registeredProfiles.filter(({ deposit, profile }) => {
                    const feeOld = calculatedFee(profile.encodedLength);
                    const feeNew = feeOld / oldToNewRatio;

                    return (
                      deposit.toBigInt() !== feeOld &&
                      deposit.toBigInt() !== feeNew &&
                      deposit.toBigInt() !== 0n
                    );
                });

                for (const { deposit, account } of failures) {
                    log(`Invalid deposit ${deposit.toNumber()} for account ${account.toHuman()} `);
                }
                expect(failures.length, `${failures.length} invalid deposits registered`).toBe(0);
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
                const failures = registeredProfiles.filter(
                    ({ profile }) => !isValidEndpointUrl(profile.url.toHuman().toString())
                );
                for (const { profile } of failures) {
                    log(`Invalid URL ${profile.url.toHuman()}`);
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
