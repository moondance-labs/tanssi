import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { MultiLocation } from "utils";
import { expectEventCount } from "../../../helpers/events";

describeSuite({
    id: "DTR1702",
    title: "EthereumTokenTransfers tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "transferNativeToken should send message to Ethereum",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                // Set channel info on EthereumTokenTransfers pallet.
                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                const tokenLocation: MultiLocation = {
                    parents: 0,
                    interior: "Here",
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "dance",
                    symbol: "dance",
                    decimals: 12,
                };

                // Register token on EthereumSystem.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                const recipient = "0x0000000000000000000000000000000000000007";
                const amount = 1000;

                // Finally call transferNativeToken extrinsic.
                const tx3 = await polkadotJs.tx.ethereumTokenTransfers
                    .transferNativeToken(amount, recipient)
                    .signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check events and digest were emitted correctly.
                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x.other);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0].other;

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                    NativeTokenTransferred: 1,
                });

                // Also a MessagesCommitted event with the same hash as the digest log
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "MessagesCommitted";
                });
                expect(ev1.length).to.be.equal(1);
                const ev1Data = ev1[0].event.data[0].toJSON();

                // logHex == 0x00 + ev1Data
                // Example:
                // logHex: 0x0064cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                // ev1Data:  0x64cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                const prefixedEv1Data = `0x00${ev1Data.slice(2)}`;
                expect(prefixedEv1Data).to.be.equal(logHex);
            },
        });
    },
});
