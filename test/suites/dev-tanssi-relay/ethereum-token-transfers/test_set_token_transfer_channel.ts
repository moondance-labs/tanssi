import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
    id: "DTR1701",
    title: "EthereumTokenTransfers tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "setTokenTransferChannel should update channel details",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const currentChannelInfo = (
                    await polkadotJs.query.ethereumTokenTransfers.currentChannelInfo()
                ).toJSON();
                expect(currentChannelInfo).to.be.null;

                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx], { allowFailures: false });

                const currentChannelInfoAfter = (
                    await polkadotJs.query.ethereumTokenTransfers.currentChannelInfo()
                ).unwrap();

                expect(currentChannelInfoAfter.channelId.toHex()).to.eq(newChannelId);
                expect(currentChannelInfoAfter.paraId.toNumber()).to.eq(newParaId);
                expect(currentChannelInfoAfter.agentId.toHex()).to.eq(newAgentId);
            },
        });
    },
});
