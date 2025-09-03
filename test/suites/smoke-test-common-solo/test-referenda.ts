import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { BN } from "@polkadot/util";

const AMOUNT_RECENT_PROPOSALS_FOR_CHECKING = 10;

/*
 * The goal of this smoke test is to ensure that the decision deposit for referenda proposals is set.
 * If the deposit is not set, the proposal will be in Preparing state until UndecidingTimeout passed (14 days).
 * Then it will be rejected with event: referenda.TimedOut.
 */
describeSuite({
    id: "SMOK17",
    title: "Smoke tests for referenda",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Referenda deposit does exist for recent proposals",
            test: async () => {
                const referendumCount = (await api.query.referenda.referendumCount()).toNumber();
                const startIndex = Math.max(0, referendumCount - AMOUNT_RECENT_PROPOSALS_FOR_CHECKING);

                for (let proposalIndex = startIndex; proposalIndex < referendumCount; proposalIndex++) {
                    const infoOpt = await api.query.referenda.referendumInfoFor(proposalIndex);
                    if (infoOpt.isSome) {
                        const info = infoOpt.unwrap();
                        if (info.isOngoing) {
                            const decisionDeposit = info.asOngoing.decisionDeposit;
                            const unwrappedDecisionDeposit = decisionDeposit.unwrapOr(null);

                            expect(
                                unwrappedDecisionDeposit,
                                `Expecting decision deposit is not null for proposal with index ${proposalIndex}`
                            ).not.toBeNull();
                        } else if (info.isApproved) {
                            const atBlock = info.asApproved[0].toNumber();
                            await checkDecisionDepositAtBlock(api, proposalIndex, atBlock - 1);
                        } else if (info.isRejected) {
                            const atBlock = info.asRejected[0].toNumber();
                            await checkDecisionDepositAtBlock(api, proposalIndex, atBlock - 1);
                        }
                    }
                }
            },
        });
    },
});

export async function checkDecisionDepositAtBlock(
    api: ApiPromise,
    referendaIndex: number,
    atBlock: number
): Promise<void> {
    const blockHash = await api.rpc.chain.getBlockHash(atBlock);
    const apiAt = await api.at(blockHash);

    const optInfo = await apiAt.query.referenda.referendumInfoFor(referendaIndex);
    const info = optInfo.unwrap();

    const trackId = info.asOngoing.track.toString();
    expect(info.isOngoing).toEqual(true);
    expect(info.asOngoing.decisionDeposit.isSome).toEqual(true);
    expect(trackId).toEqual("0");

    const trackInfo = apiAt.consts.referenda.tracks;
    const trackInfoDetails = trackInfo.at(Number.parseInt(trackId)).toJSON()[1] as {
        decisionDeposit: number;
    };

    const depositInfo = info.asOngoing.decisionDeposit.unwrap().toJSON() as { who: string; amount: number };
    expect(new BN(depositInfo.amount.toString())).toEqual(new BN(trackInfoDetails.decisionDeposit.toString()));
}
