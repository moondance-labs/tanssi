import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DTR1701",
    title: "EthereumTokenTransfers tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlighETT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlighETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_TOKEN_TRANSFERS.includes(specVersion);
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

                const tx = polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                    newChannelId,
                    newAgentId,
                    newParaId
                );

                if (shouldSkipStarlighETT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                const sudoSignedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);

                await context.createBlock([sudoSignedTx], { allowFailures: false });

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
