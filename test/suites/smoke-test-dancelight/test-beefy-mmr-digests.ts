import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { stringToHex } from "@polkadot/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK03",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Session change block should update BEEFY and MMR root digests properly",
            test: async () => {
                const blockToCheck = (await api.query.babe.epochStart()).toJSON()[1];

                const apiAtBeforeSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 5));
                const beefyNextAuthorities = await apiAtBeforeSessionChange.query.beefy.nextAuthorities();

                const apiAtSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));

                const digestsInSessionChange = (await apiAtSessionChange.query.system.digest()).logs;
                const filteredDigests = digestsInSessionChange.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BEEF")
                );

                // As session changed, it should contain two BEEFY digests: AuthoritiesChange and MmrRoot.
                expect(filteredDigests.length).to.eq(2);

                // 0x01 corresponds to ConsensusLog::AuthoritiesChange enum variant.
                expect(filteredDigests[0].asConsensus[1].toHex().startsWith("0x01")).to.be.true;

                // Check if each authority is included in the BEEFY digest
                for (const authority of Object.values(beefyNextAuthorities.toJSON())) {
                    expect(filteredDigests[0].asConsensus[1].toHex().includes(authority.slice(2))).to.be.true;
                }

                const firstMmrRootDigest = filteredDigests[1].asConsensus[1].toHex();

                // 0x03 corresponds to ConsensusLog::MmrRoot enum variant.
                expect(firstMmrRootDigest.startsWith("0x03")).to.be.true;

                // Second BEEF log should contain the MMR root.
                // Length should be 68 (0x03 + 32 bytes MMR root).
                expect(firstMmrRootDigest.length).to.eq(68);

                // Now let's check just after session change
                const apiAtAfterSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockToCheck + 1));
                const digestsAfterSessionChange = (await apiAtAfterSessionChange.query.system.digest()).logs;
                const filteredDigestsAfterSessionChange = digestsAfterSessionChange.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BEEF")
                );

                // Now we should only have the MmrRoot BEEFY digest (as session didn't change yet).
                expect(filteredDigestsAfterSessionChange.length).to.eq(1);

                const secondMmrRootDigest = filteredDigestsAfterSessionChange[0].asConsensus[1].toHex();

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
