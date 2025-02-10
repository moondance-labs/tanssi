import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getBlockArray } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { GenericExtrinsic, U32 } from "@polkadot/types";
import type { FrameSystemEventRecord } from "@polkadot/types/lookup";
import type { AnyTuple } from "@polkadot/types/types";
import { hexToU8a, stringToHex } from "@polkadot/util";
import { sr25519Verify } from "@polkadot/wasm-crypto";
import Bottleneck from "bottleneck";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);
const hours = (timePeriod / (1000 * 60 * 60)).toFixed(2);

type BlockFilteredRecord = {
    blockNum: number;
    blockHash;
    header;
    preHash;
    extrinsics: GenericExtrinsic<AnyTuple>[];
    events: FrameSystemEventRecord[];
    logs;
    authorities;
    accountsWithBabeKeys;
};

describeSuite({
    id: "SMOK02",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let blockData: BlockFilteredRecord[];

        beforeAll(async () => {
            api = context.polkadotJs();

            const blockNumArray = await getBlockArray(api, timePeriod);

            log(`Collecting ${hours} hours worth of authors`);

            const getBlockData = async (blockNum: number) => {
                const blockHash = await api.rpc.chain.getBlockHash(blockNum);
                const signedBlock = await api.rpc.chain.getBlock(blockHash);
                const header = signedBlock.block.header;
                const apiAt = await api.at(blockHash);
                const preHash = getPreHash(api, header);

                const authorities = await apiAt.query.session.validators();
                const babeAuthorities = await apiAt.query.babe.authorities();

                const blockBabeEpochStart = (await apiAt.query.babe.epochStart()).toJSON()[0];
                const apiAtSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockBabeEpochStart));

                // If there has been a recent change in session keys,
                // it might happen that BABE keys from pallet babe haven't changed yet,
                // but new (next) keys are already stored in pallet session.
                //
                // So we need to check the keyOwner storage at the previous epoch change to check
                // the actual BABE keys that were used to seal the block and match the validator
                // accounts at that point.
                //
                // If we were to use the session pallet to retrieve the validators (and keys),
                // we would get the new set of keys wich are not being applied yet.
                const accountsWithBabeKeys = await keyOwners(babeAuthorities, apiAtSessionChange);

                return {
                    blockNum: blockNum,
                    preHash,
                    extrinsics: signedBlock.block.extrinsics,
                    events: await apiAt.query.system.events(),
                    logs: signedBlock.block.header.digest.logs,
                    authorities,
                    accountsWithBabeKeys,
                };
            };
            const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });
            blockData = await Promise.all(blockNumArray.map((num) => limiter.schedule(() => getBlockData(num))));
        }, timeout);

        it({
            id: "C01",
            title: "BABE keys are set and validators from logs match validators from pallet",
            test: async () => {
                // Check the previous epoch digest.
                // The [0] index indicates the block number in which the previous session started.
                // The [1] index indicates the block number in which the current session started.
                const blockToCheck = ((await api.query.babe.epochStart()) as unknown as [U32, U32])[0];
                const apiAtSessionChange = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));

                const digestsInSessionChange = (await apiAtSessionChange.query.system.digest()).logs;
                const filteredDigests = digestsInSessionChange.filter(
                    (log) => log.isConsensus === true && log.asConsensus[0].toHex() === stringToHex("BABE")
                );
                expect(filteredDigests.length).to.eq(1);

                // 0x01 corresponds to ConsensusLog::NextEpochData enum variant.
                expect(filteredDigests[0].asConsensus[1].toHex().startsWith("0x01")).to.be.true;

                // Assert that authorities from log === authorities from pallet
                const babeAuthoritiesFromPallet = await api.query.babe.authorities();
                const babeConsensusLog = api.registry.createType(
                    "(u8, Vec<(SpConsensusBabeAppPublic, u64)>, [u8; 32])",
                    filteredDigests[0].asConsensus[1].toHex()
                );

                expect(babeConsensusLog[1]).to.deep.equal(babeAuthoritiesFromPallet);

                const keyOwnersArray = [];
                const keyOwnersInPalletSession = await keyOwners(babeAuthoritiesFromPallet, api);

                for (const keyOwner of keyOwnersInPalletSession) {
                    keyOwnersArray.push(keyOwner[0]);
                }

                // Get validators from pallet session
                const sessionValidators = await api.query.session.validators();

                keyOwnersArray.sort();
                sessionValidators.sort();

                // Check that key owners found are the same validators in pallet session
                expect(keyOwnersArray).to.deep.equal(sessionValidators.toJSON());
            },
        });

        it({
            id: "C02",
            title: "BABE author signature valid",
            test: async () => {
                const failures = blockData
                    .map(({ blockNum, preHash, logs, authorities, accountsWithBabeKeys }) => {
                        const babeLogs = logs.filter(
                            (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() === stringToHex("BABE")
                        );
                        expect(babeLogs.length).to.eq(1);

                        const babeLogEnum = api.registry.createType(
                            "SpConsensusBabeDigestsPreDigest",
                            babeLogs[0].asPreRuntime[1].toHex()
                        );

                        expect(babeLogEnum.isSecondaryVRF || babeLogEnum.isPrimary).toBeTruthy();
                        const babeLog = babeLogEnum.isSecondaryVRF ? babeLogEnum.asSecondaryVRF : babeLogEnum.asPrimary;

                        // Get expected author from BABE log and on chain authorities
                        const authorityIndex = babeLog.authorityIndex;
                        const orchestratorAuthorities = authorities.toJSON();
                        const expectedAuthor = orchestratorAuthorities[authorityIndex.toNumber()];

                        // Get block author signature from seal log
                        const sealLogs = logs.filter(
                            (log) => log.isSeal === true && log.asSeal[0].toHex() === stringToHex("BABE")
                        );

                        expect(sealLogs.length).to.eq(1);
                        const sealLog = api.registry.createType(
                            "PolkadotPrimitivesV7ValidatorAppSignature",
                            sealLogs[0].asSeal[1].toHex()
                        );

                        // Verify seal signature
                        const message = hexToU8a(preHash);
                        const signature = hexToU8a(sealLog.toHex());
                        const authorBabe = accountsWithBabeKeys.find((acc) => acc[0] === expectedAuthor);
                        expect(authorBabe, `Missing babe key for block author: ${expectedAuthor}`).toBeTruthy();
                        const pubKey = hexToU8a(authorBabe[1]);

                        const authorValid = sr25519Verify(signature, message, pubKey);

                        return { blockNum, expectedAuthor, authorValid };
                    })
                    .filter(({ authorValid }) => authorValid === false);

                failures.forEach(({ blockNum, expectedAuthor }) => {
                    log(
                        `Author at block #${blockNum} should have been #${expectedAuthor.toString()}, but seal signature does not match`
                    );
                });

                expect(
                    failures.length,
                    `Please investigate blocks ${failures.map((a) => a.blockNum).join(", ")}; authors  `
                ).to.equal(0);
            },
        });
    },
});

// Given a block header, returns its preHash. This is the hash of the header before adding the seal.
// The hash of the block header after adding the seal is the block hash.
function getPreHash(api, header) {
    const logsNoSeal = header.digest.logs.filter((log) => !log.isSeal);
    const headerWithoutSeal = api.registry.createType("Header", {
        parentHash: header.parentHash,
        number: header.number,
        stateRoot: header.stateRoot,
        extrinsicsRoot: header.extrinsicsRoot,
        digest: {
            logs: logsNoSeal,
        },
    });
    return headerWithoutSeal.hash.toHex();
}

// Helper function to retrieve the validators from pallet session given
// the babe authorities (babeKeys) and a specific api instance.
async function keyOwners(babeAuthoritiesFromPallet, api) {
    const keyOwnersInPalletSession = [];

    for (const authority of babeAuthoritiesFromPallet) {
        const param = api.registry.createType("(SpCoreCryptoKeyTypeId, Bytes)", [
            stringToHex("babe"),
            authority[0].toHex(),
        ]);
        const keyOwner = await api.query.session.keyOwner(param);

        expect(
            keyOwner.isSome,
            `Validator with babe key ${authority[0].toJSON()} not found in pallet session!`
        ).toBeTruthy();
        keyOwnersInPalletSession.push([keyOwner.toJSON(), authority[0].toJSON()]);
    }

    return keyOwnersInPalletSession;
}
