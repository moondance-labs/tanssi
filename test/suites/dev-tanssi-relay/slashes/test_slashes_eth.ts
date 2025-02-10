import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { jumpToSession } from "../../../util/block";
import { expectEventCount } from "../../../helpers/events";

describeSuite({
    id: "DTR1801",
    title: "Test slashes are being sent to ethereum using bridge",
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
            title: "Test using rootTestSendMsgToEth",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const nonce = "0x0000000000000000000000000000000000000000000000000000000000000000";
                const numMsg = 1;
                const msgSize = 32;
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(nonce, numMsg, msgSize))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x["other"]);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0]["other"];

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                });

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

        it({
            id: "E02",
            title: "Test using ethereumSystem.upgrade",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const implAddress = "0x0000000000000000000000000000000000000001";
                const implCodeHash = "0x0000000000000000000000000000000000000000000000000000000000000002";
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.upgrade(implAddress, implCodeHash, null))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x["other"]);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0]["other"];

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                });

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

        it({
            id: "E03",
            title: "Send too big message using rootTestSendMsgToEth",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const nonce = "0x0000000000000000000000000000000000000000000000000000000000000000";
                const numMsg = 1;
                // TODO: the limit should be 2048 bytes, not 1921
                const msgSize = 1921;
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(nonce, numMsg, msgSize))
                    .signAsync(alice);
                await context.createBlock([tx]);

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 0,
                    MessageAccepted: 0,
                    Processed: 0,
                    MessageQueued: 0,
                });

                // Also a MessagesCommitted event with the same hash as the digest log
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "Sudid";
                });
                expect(ev1.length).to.be.equal(1);
                const ev1Data = ev1[0].event.data[0].toJSON();
                expect(ev1Data["err"]).toBeTruthy();
            },
        });

        it({
            id: "E04",
            title: "Send message of max size using rootTestSendMsgToEth",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const nonce = "0x0000000000000000000000000000000000000000000000000000000000000000";
                const numMsg = 1;
                const msgSize = 1920;
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(nonce, numMsg, msgSize))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x["other"]);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0]["other"];

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                });

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

        it({
            id: "E05",
            title: "Send 100 messages using rootTestSendMsgToEth",
            test: async function () {
                await jumpToSession(context, 1);

                // Send test message to ethereum
                const nonce = "0x0000000000000000000000000000000000000000000000000000000000000000";
                const numMsg = 100;
                const msgSize = 32;
                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(nonce, numMsg, msgSize))
                    .signAsync(alice);
                await context.createBlock([tx]);

                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x["other"]);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0]["other"];

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 32,
                    Processed: 32,
                    MessageQueued: 100,
                });

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

                // Next block will have 32 events more
                await context.createBlock();
                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 32,
                    Processed: 32,
                    MessageQueued: 0,
                });

                // Total so far: 64
                await context.createBlock();
                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 32,
                    Processed: 32,
                    MessageQueued: 0,
                });

                // Total so far: 96, missing last 4
                await context.createBlock();
                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 4,
                    Processed: 4,
                    MessageQueued: 0,
                });
            },
        });
    },
});
