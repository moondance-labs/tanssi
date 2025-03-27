import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMDR01",
    title: "HRMP Validation Test Suite",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "HRMP channels are only opened between active parachains or parathreads",
            test: async () => {
                // Get all active parachains and parathreads
                const activeParaSet = new Set(
                    (await api.query.paras.paraLifecycles.entries())
                        .map(([key]) => key.args[0].toString())
                );

                // Get all HRMP channels
                const hrmpChannels = await api.query.hrmp.hrmpChannels.entries();

                // Verify each channel is between active paras
                for (const [key, _] of hrmpChannels) {
                    const { sender, recipient } = key.args[0];

                    // Check if both sender and recipient are valid
                    const senderId = sender.toString();
                    const recipientId = recipient.toString();

                    expect(
                        activeParaSet.has(senderId),
                        `HRMP channel sender ${senderId} is not an active parachain or parathread`
                    ).to.be.true;

                    expect(
                        activeParaSet.has(recipientId),
                        `HRMP channel recipient ${recipientId} is not an active parachain or parathread`
                    ).to.be.true;
                }
            },
        });

        it({
            id: "C02",
            title: "Proper deposits are made for HRMP channels",
            test: async () => {
                const config = await api.query.configuration.activeConfig();
                const hrmpSenderDeposit = config.hrmpSenderDeposit.toBigInt();
                const hrmpRecipientDeposit = config.hrmpRecipientDeposit.toBigInt();

                const hrmpChannels = await api.query.hrmp.hrmpChannels.entries();

                for (const [key, channelInfo] of hrmpChannels) {
                    const { sender, recipient } = key.args[0];

                    const senderId = sender.toString();
                    const recipientId = recipient.toString();

                    // Check sender deposit is correct
                    const senderDeposit = channelInfo.unwrap().senderDeposit.toBigInt();
                    expect(senderDeposit).to.equal(
                        hrmpSenderDeposit,
                        `Incorrect sender deposit for channel ${senderId}->${recipientId}`
                    );

                    // Check recipient deposit is correct
                    const recipientDeposit = channelInfo.unwrap().recipientDeposit.toBigInt();
                    expect(recipientDeposit).to.equal(
                        hrmpRecipientDeposit,
                        `Incorrect recipient deposit for channel ${senderId}->${recipientId}`
                    );
                }
            },
        });

        it({
            id: "C03",
            title: "HRMP channels don't exceed the maximum limit per parachain",
            test: async () => {
                const config = await api.query.configuration.activeConfig();
                const maxHrmpChannelsPerParachain = config.hrmpMaxParachainOutboundChannels.toNumber();

                const hrmpChannels = await api.query.hrmp.hrmpChannels.entries();

                // Map to count the number of outbound channels per parachain
                const outboundChannelCounts = new Map();

                // Traverse all HRMP channels and count the number of outbound channels per parachain
                for (const [key, _] of hrmpChannels) {
                    const { sender } = key.args[0];
                    const senderId = sender.toString();

                    const currentCount = outboundChannelCounts.get(senderId) || 0;
                    outboundChannelCounts.set(senderId, currentCount + 1);
                }

                // Verify no parachain exceeds the limit
                for (const [paraId, count] of outboundChannelCounts.entries()) {
                    expect(count).to.be.lessThanOrEqual(
                        maxHrmpChannelsPerParachain,
                        `Parachain ${paraId} has ${count} outbound HRMP channels, exceeding the maximum of ${maxHrmpChannelsPerParachain}`
                    );
                }
            },
        });
    },
});
