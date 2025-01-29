import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";

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
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const currentChannelId = (await polkadotJs.query.ethereumTokenTransfers.currentChannelId()).toJSON();
                const currentParalId = (await polkadotJs.query.ethereumTokenTransfers.currentParaId()).toJSON();
                const currentAgentId = (await polkadotJs.query.ethereumTokenTransfers.currentAgentId()).toJSON();

                expect(currentChannelId).to.be.null;
                expect(currentParalId).to.be.null;
                expect(currentAgentId).to.be.null;

                let newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                let newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                let newParaId = 500;

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

                const currentChannelIdAfter = (
                    await polkadotJs.query.ethereumTokenTransfers.currentChannelId()
                ).toJSON();
                const currentParalIdAfter = (await polkadotJs.query.ethereumTokenTransfers.currentParaId()).toJSON();
                const currentAgentIdAfter = (await polkadotJs.query.ethereumTokenTransfers.currentAgentId()).toJSON();

                expect(currentChannelIdAfter).to.eq(newChannelId);
                expect(currentParalIdAfter).to.eq(newParaId);
                expect(currentAgentIdAfter).to.eq(newAgentId);
            },
        });
    },
});
