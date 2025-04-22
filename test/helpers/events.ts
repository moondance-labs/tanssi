import { expect } from "@moonwall/cli";
import { type ApiPromise } from "@polkadot/api";
import type { SpRuntimeDispatchError } from "@polkadot/types/lookup";

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

export async function checkCallIsFiltered(context: any, polkadotJs: ApiPromise, tx: any) {
    try {
        await context.createBlock(await tx, { allowFailures: false });
        expect.fail("Expected call to be filtered, but it was not.");
    } catch {
        const events = await polkadotJs.query.system.events();
        const errors = events
            .filter(({ event }) => polkadotJs.events.system.ExtrinsicFailed.is(event))
            .map(
                ({
                    event: {
                        data: [error],
                    },
                }) => {
                    const dispatchError = error as SpRuntimeDispatchError;
                    if (dispatchError.isModule) {
                        const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                        const { method } = decoded;

                        return `${method}`;
                    }
                    return error.toString();
                }
            );

        expect(errors.length).to.be.eq(1);
        expect(errors[0]).to.be.eq("CallFiltered");
    }
}