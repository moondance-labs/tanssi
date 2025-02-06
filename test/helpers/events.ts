import { expect } from "@moonwall/cli";
export async function expectEventCount(polkadotJs, eventCounts: Record<string, number>): Promise<void> {
    const events = await polkadotJs.query.system.events();

    for (const [eventMethod, expectedCount] of Object.entries(eventCounts)) {
        const matchingEvents = events.filter(({ event }) => event.method === eventMethod);

        expect(
            matchingEvents.length,
            `Expected ${expectedCount} occurrences of event '${eventMethod}', but found ${matchingEvents.length}`
        ).to.equal(expectedCount);
    }
}
