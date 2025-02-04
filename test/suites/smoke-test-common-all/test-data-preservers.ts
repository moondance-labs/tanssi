import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S16",
    title: "Verify data preservers consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it }) => {
        let paraApi: ApiPromise;

        beforeAll(async () => {
            paraApi = context.polkadotJs("para");
        });

        it({
            id: "C01",
            title: "all profiles should have a deposit of either 0 or value fixed in the runtime",
            test: async () => {
                // Add more if we change ProfileDeposit value. Keep previous values for profiles
                // created before the change.
                const validDeposits = [0, 11330000000000];

                const entries = await paraApi.query.dataPreservers.profiles.entries();

                for (const [, entry] of entries) {
                    expect(validDeposits.includes(entry.deposit));
                }
            },
        });

        it({
            id: "C02",
            title: "all assigned profile have assignement witness corresponding to request and whished para id",
            test: async () => {
                const entries = await paraApi.query.dataPreservers.profiles.entries();

                for (const [, entry] of entries) {
                    if (entry.assignment === null) {
                        continue;
                    }

                    const [para_id, witness] = entry.assignment;

                    if (entry.profile.paraIds.whitelist != null) {
                        expect(entry.profile.paraIds.whitelist.includes(para_id));
                    } else if (entry.profile.paraIds.blacklist != null) {
                        expect(!entry.profile.paraIds.blacklist.includes(para_id));
                    }

                    if (entry.profile.assignmentRequest === "Free") {
                        expect(witness).to.be.eq("Free");
                    } else if (entry.profile.assignmentRequest.streamPayment != null) {
                        expect(witness.streamPayment).to.not.be.undefined();
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
                const entries = await paraApi.query.dataPreservers.profiles.entries();

                for (const [, entry] of entries) {
                    const profile = entry.unwrap().profile;
                    expect(isValidEndpointUrl(profile.url.toHuman()), `Invalid URL {profile.url}`);
                }
            },
        });
    },
});

function isValidEndpointUrl(string) {
    const prefixes = ["/dns4/", "https://", "http://", "wss://", "ws://"];

    return prefixes.some((prefix) => string.startsWith(prefix));
}
