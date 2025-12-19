import { type DevModeContext, fastFowardToNextEvent } from "@moonwall/cli";
import type { KeyringPair } from "@polkadot/keyring/types";
import { BN } from "@polkadot/util";

// Maximizes conviction voting of some voters
// with respect to an ongoing referenda
// Their whole free balance will be used to vote
export const maximizeConvictionVotingOf = async (context: DevModeContext, voters: KeyringPair[], refIndex: number) => {
    // We need to have enough to pay for fee
    const fee = (
        await context
            .polkadotJs()
            .tx.convictionVoting.vote(refIndex, {
                Standard: {
                    vote: { aye: true, conviction: "Locked6x" },
                    balance: ((await context.polkadotJs().query.system.account(voters[0].address)) as any).data.free,
                },
            })
            .paymentInfo(voters[0])
    ).partialFee;

    // We vote with everything but fee
    // We leave 10 times the fee in case we need to vote more than once
    await context.createBlock(
        voters.map(async (voter) =>
            context
                .polkadotJs()
                .tx.convictionVoting.vote(refIndex, {
                    Standard: {
                        vote: { aye: true, conviction: "Locked6x" },
                        balance: await (
                            (await context.polkadotJs().query.system.account(voter.address)) as any
                        ).data.free.sub(fee.mul(new BN(10))),
                    },
                })
                .signAsync(voter)
        )
    );
};

// Fast forward until the proper error is thrown
export const fastForwardUntilNoMoreEvents = async (context: DevModeContext) => {
    while (true) {
        try {
            await fastFowardToNextEvent(context);
        } catch (err: any) {
            if (err.message === "entry is not iterable") {
                return;
            }
            throw err; // unexpected error
        }
    }
};
