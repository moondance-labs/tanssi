import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { u8aToHex } from "@polkadot/util";
import { jumpToSession } from "../../../util/block";
import { generateBabeEquivocationProof } from "../../../util/slashes";

describeSuite({
    id: "DTR1801",
    title: "Babe offences should trigger a slash",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceBabePair: KeyringPair;
        let aliceStash: KeyringPair;
        beforeAll(async () => {
            const keyringBabe = new Keyring({ type: "sr25519" });
            aliceBabePair = keyringBabe.addFromUri("//Alice");
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            aliceStash = keyringBabe.addFromUri("//Alice//stash");
        });
        it({
            id: "E01",
            title: "Babe offences trigger a slash",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // Remove alice from invulnerables (just for the slash)
                const msgId = "0x0000000000000000000000000000000000000000000000000000000000000001";
                const h256 = "0x0000000000000000000000000000000000000000000000000000000000000002";
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.rootTestSendMsgToEth(msgId, h256))
                    .signAsync(alice);
                await context.createBlock([removeAliceFromInvulnerables]);

                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                console.log(baseHeader.toJSON());
            },
        });
    },
});
