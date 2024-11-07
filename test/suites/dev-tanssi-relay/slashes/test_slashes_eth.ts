import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { jumpToSession } from "../../../util/block";

describeSuite({
    id: "DTR1801",
    title: "Babe offences should trigger a slash",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Babe offences trigger a slash",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const msgId = "0x0000000000000000000000000000000000000000000000000000000000000001";
                const h256 = "0x0000000000000000000000000000000000000000000000000000000000000002";
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(msgId, h256))
                    .signAsync(alice);
                await context.createBlock([removeAliceFromInvulnerables]);

                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x["other"]);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0]["other"];

                // Also a MessagesCommitted event with the same hash as the digest log
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "MessagesCommitted";
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
