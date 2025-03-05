import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { PRIMARY_GOVERNANCE_CHANNEL_ID, getCurrentEraStartBlock, getPastEraStartBlock } from "utils";

describeSuite({
    id: "SMOK09",
    title: "Smoke tests for external validators slashes pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Number of bonded eras does not exceed bonding duration",
            test: async () => {
                const bondedEras = await api.query.externalValidatorSlashes.bondedEras();
                const bondedDuration = await api.consts.externalValidatorSlashes.bondingDuration;
                const currentEra = (await api.query.externalValidators.activeEra()).unwrap();

                if (currentEra.index < bondedDuration) {
                    expect(bondedEras.length).to.be.lessThanOrEqual(bondedDuration.toNumber() + 1);
                } else {
                    expect(bondedEras.length).to.be.eq(bondedDuration.toNumber() + 1);
                }
            },
        });

        it({
            id: "C02",
            title: "Slash message should increase nonce",
            timeout: 600000,
            test: async () => {
                const bondedDuration = await api.consts.externalValidatorSlashes.bondingDuration;
                const currentEra = (await api.query.externalValidators.activeEra()).unwrap();

                let eraToBeAnalyzed = currentEra.index;
                const firstBlockNumber = await getCurrentEraStartBlock(api);
                let blockNumberCheckpointStartEra = firstBlockNumber;

                log(`Analizing backwards ${bondedDuration} eras`);

                // Go backwards bondedDuration eras
                // We will analyze all the bounded eras and not just the last
                while (
                    eraToBeAnalyzed.toNumber() > currentEra.index.toNumber() - bondedDuration.toNumber() &&
                    eraToBeAnalyzed.toNumber() > 0
                ) {
                    const blockNumberCheckpointPreviousEra = blockNumberCheckpointStartEra - 1;

                    blockNumberCheckpointStartEra = await getPastEraStartBlock(api, blockNumberCheckpointPreviousEra);
                    const apiAtCheckpointStartEra = await api.at(
                        await api.rpc.chain.getBlockHash(blockNumberCheckpointStartEra)
                    );

                    // We dont have an event, so we will check if we have unreported slashes
                    const apitAtCheckpointPreviousEraEnd = await api.at(
                        await api.rpc.chain.getBlockHash(blockNumberCheckpointStartEra - 1)
                    );
                    const unreportedSlashes =
                        await apitAtCheckpointPreviousEraEnd.query.externalValidatorSlashes.unreportedSlashesQueue();
                    eraToBeAnalyzed = (
                        await apitAtCheckpointPreviousEraEnd.query.externalValidators.activeEra()
                    ).unwrap().index;

                    if (unreportedSlashes.length > 0) {
                        // at the beginning of the next era, we should have sent a message
                        // therefore the nonce has increased
                        const nonceBefore =
                            await apitAtCheckpointPreviousEraEnd.query.ethereumOutboundQueue.nonce(
                                PRIMARY_GOVERNANCE_CHANNEL_ID
                            );
                        const nonceAfter =
                            await apiAtCheckpointStartEra.query.ethereumOutboundQueue.nonce(
                                PRIMARY_GOVERNANCE_CHANNEL_ID
                            );
                        expect(
                            nonceAfter.toNumber(),
                            `Slash message for era ${eraToBeAnalyzed} was not sent even if we have  ${unreportedSlashes.length} to be reported in block ${blockNumberCheckpointStartEra}`
                        ).toBe(nonceBefore.toNumber() + 1);
                    }
                }
            },
        });
    },
});
