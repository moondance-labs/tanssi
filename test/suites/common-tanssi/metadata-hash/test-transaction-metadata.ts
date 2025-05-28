import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { SignerOptions } from "@polkadot/api/types";
import { merkleizeMetadata } from "@polkadot-api/merkleize-metadata";
import { u8aToHex } from "@polkadot/util";
import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@moonwall/util";
import { isDancebox } from "../../../utils/runtime.ts";

async function getMetadataHash(api: ApiPromise) {
    const metadata = await api.call.metadata.metadataAtVersion(15);
    const { specName, specVersion } = api.runtimeVersion;

    const hash = merkleizeMetadata(metadata.toHex(), {
        base58Prefix: api.consts.system.ss58Prefix.toNumber(),
        decimals: api.registry.chainDecimals[0],
        specName: specName.toString(),
        specVersion: specVersion.toNumber(),
        tokenSymbol: api.registry.chainTokens[0],
    });

    return u8aToHex(hash.digest());
}

describeSuite({
    id: "COMM0401",
    title: "Test transaction with metadata hash",
    foundationMethods: "dev",
    testCases: ({ context, it, log }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "T01",
            title: "Should fail with an invalid metadata hash",
            test: async ({ skip }) => {
                if (isDancebox(polkadotJs)) {
                    skip();
                }

                const withMetadataOpts: Partial<SignerOptions> = {
                    mode: 1,
                    metadataHash: `0x${"00".repeat(32)}`,
                };

                await context.createBlock();

                let errorMsg = "";
                try {
                    await polkadotJs.tx.system.remark("0x00").signAndSend(alice, withMetadataOpts);
                    await context.createBlock();
                } catch (e) {
                    errorMsg = e.message;
                }

                expect(errorMsg).to.be.eq("1010: Invalid Transaction: Transaction has a bad signature");
            },
        });

        it({
            id: "T02",
            title: "Should succeed with a valid metadata hash",
            test: async ({ skip }) => {
                if (isDancebox(polkadotJs)) {
                    skip();
                }

                await context.createBlock();

                const withMetadataOpts = {
                    mode: 1,
                    metadataHash: await getMetadataHash(polkadotJs),
                };

                await polkadotJs.tx.system.remark("0x00").signAndSend(alice, withMetadataOpts);
                await context.createBlock();
            },
        });
    },
});
