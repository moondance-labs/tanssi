import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { stringToHex } from "@polkadot/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S19",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "BABE keys are set and validators from logs match validators from pallet",
            test: async function () {
                const blockToCheck = (await api.query.babe.epochStart()).toJSON()[1];

                const apiAtSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));

                const digestsInSessionChange = (await apiAtSessionChange.query.system.digest()).logs;
                const filteredDigests = digestsInSessionChange.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() == stringToHex("BABE")
                );
                expect(filteredDigests.length).to.eq(1);

                // 0x01 corresponds to ConsensusLog::NextEpochData enum variant.
                expect(filteredDigests[0].asConsensus[1].toHex().startsWith("0x01")).to.be.true;

                // Assert that authorities from log == authorities from pallet
                const babeAuthoritiesFromPallet = await api.query.babe.authorities();
                const asdas = api.registry.createType(
                    "(u8, Vec<(SpConsensusBabeAppPublic, u64)>, [u8; 32])",
                    filteredDigests[0].asConsensus[1].toHex()
                );

                expect(asdas[1]).to.deep.equal(babeAuthoritiesFromPallet);

                // Get babe keys from pallet session
                const sessionValidators = await api.query.session.validators();

                const babeKeysInPalletSession = [];

                for (const account of sessionValidators) {
                    const accountKeys = await api.query.session.nextKeys(account);
                    expect(accountKeys.isSome, `Missing babe key for validator ${account.toJSON()}`).toBeTruthy();
                    babeKeysInPalletSession.push(accountKeys.unwrap().babe.toHex());
                }

                // Assert that all validators have babe keys
                const babeAuthoritiesSorted = babeAuthoritiesFromPallet.map((x) => x[0].toHex());
                babeAuthoritiesSorted.sort();
                babeKeysInPalletSession.sort();
                expect(babeKeysInPalletSession).to.deep.equal(babeAuthoritiesSorted);
            },
        });
    },
});
