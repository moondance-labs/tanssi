import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { stringToHex, u8aToHex } from "@polkadot/util";
import { jumpToSession } from "utils";

describeSuite({
    id: "DEVT1401",
    title: "BEEFY - Test Digests",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let aliceStash: KeyringPair;
        let firstMmrRootDigest: `0x${string}`;
        let secondMmrRootDigest: `0x${string}`;
        beforeAll(() => {
            const keyring = new Keyring({ type: "sr25519" });
            aliceStash = keyring.addFromUri("//Alice//stash");
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Session change should update BEEFY authorities and digest",
            test: async () => {
                await jumpToSession(context, 1);

                // Take Alice's BEEFY key corresponding to next session.
                const aliceNextKeys = await polkadotJs.query.session.nextKeys(aliceStash.address);
                const aliceNextBeefyKey = u8aToHex(aliceNextKeys.unwrap().beefy);

                await jumpToSession(context, 2);
                const digests = (await polkadotJs.query.system.digest()).logs;
                const filteredDigests = digests.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BEEF")
                );

                // As session changed, it should contain two BEEFY digests: AuthoritiesChange and MmrRoot.
                expect(filteredDigests.length).to.eq(2);

                // 0x01 corresponds to ConsensusLog::AuthoritiesChange enum variant.
                expect(filteredDigests[0].asConsensus[1].toHex().startsWith("0x01")).to.be.true;

                // First BEEF log should contain Alice's BEEFY key
                expect(filteredDigests[0].asConsensus[1].toHex().includes(aliceNextBeefyKey.slice(2))).to.be.true;

                firstMmrRootDigest = filteredDigests[1].asConsensus[1].toHex();

                // 0x03 corresponds to ConsensusLog::MmrRoot enum variant.
                expect(firstMmrRootDigest.startsWith("0x03")).to.be.true;

                // Second BEEF log should contain the MMR root.
                // Length should be 68 (0x03 + 32 bytes MMR root).
                expect(firstMmrRootDigest.length).to.eq(68);
            },
        });

        it({
            id: "E02",
            title: "Should also update MMR root digest when creating a new block",
            test: async () => {
                await context.createBlock();
                const digests = (await polkadotJs.query.system.digest()).logs;
                const filteredDigests = digests.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BEEF")
                );

                // Now we should only have the MmrRoot BEEFY digest (as session didn't change yet).
                expect(filteredDigests.length).to.eq(1);

                secondMmrRootDigest = filteredDigests[0].asConsensus[1].toHex();

                // New MmrRoot digest should be different than the first one found.
                expect(secondMmrRootDigest).to.not.eq(firstMmrRootDigest);

                // 0x03 corresponds to ConsensusLog::MmrRoot enum variant.
                expect(secondMmrRootDigest.startsWith("0x03")).to.be.true;

                // Second BEEF log should contain the MMR root.
                // Length should be 68 (0x03 + 32 bytes MMR root).
                expect(secondMmrRootDigest.length).to.eq(68);
            },
        });
    },
});
