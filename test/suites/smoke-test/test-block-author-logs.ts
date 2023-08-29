import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getBlockArray } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { GenericExtrinsic } from "@polkadot/types";
import { FrameSystemEventRecord } from "@polkadot/types/lookup";
import { AnyTuple } from "@polkadot/types/types";
import { hexToNumber, stringToHex } from "@polkadot/util";
import Bottleneck from "bottleneck";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);
const hours = (timePeriod / (1000 * 60 * 60)).toFixed(2);

type BlockFilteredRecord = {
    blockNum: number;
    extrinsics: GenericExtrinsic<AnyTuple>[];
    events: FrameSystemEventRecord[];
    logs;
    authorities;
};

describeSuite({
    id: "S02",
    title: `Authors in the last ${hours} should match the slot number provided`,
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let blockData: BlockFilteredRecord[];

        beforeAll(async function () {
            api = context.polkadotJs();
            const blockNumArray = await getBlockArray(api, timePeriod);
            log(`Collecting ${hours} hours worth of authors`);

            const getBlockData = async (blockNum: number) => {
                const blockHash = await api.rpc.chain.getBlockHash(blockNum);
                const signedBlock = await api.rpc.chain.getBlock(blockHash);
                const apiAt = await api.at(blockHash);

                return {
                    blockNum: blockNum,
                    extrinsics: signedBlock.block.extrinsics,
                    events: await apiAt.query.system.events(),
                    logs: signedBlock.block.header.digest.logs,
                    authorities: await apiAt.query.authorityAssignment.collatorContainerChain(
                        await apiAt.query.session.currentIndex()
                    ),
                };
            };
            const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });
            blockData = await Promise.all(blockNumArray.map((num) => limiter.schedule(() => getBlockData(num))));
        }, timeout);

        it({
            id: "C01",
            title: "Author should be correctly set",
            test: async function () {
                const failures = blockData
                    .map(({ blockNum, logs, authorities }) => {
                        const nimbusLog = logs.filter(
                            (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex("nmbs")
                        );
                        // nimbus log has to exist
                        const author = nimbusLog[0].asPreRuntime[1].toHex();

                        // aura log has to exist
                        const slotLog = logs.filter(
                            (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex("aura")
                        );
                        const slot = slotLog[0].asPreRuntime[1].reverse().toHex();

                        const orchestratorAuthorities = authorities.toJSON()["orchestratorChain"];
                        const expectedAuthor =
                            orchestratorAuthorities[hexToNumber(slot) % orchestratorAuthorities.length];

                        return { blockNum, author, expectedAuthor };
                    })
                    .filter(({ expectedAuthor, author }) => expectedAuthor.toString() != author.toString());

                failures.forEach(({ blockNum, author, expectedAuthor }) => {
                    log(
                        `Author at block #${blockNum} was #${author.toString()}` +
                            `but should have been #${expectedAuthor.toString()}`
                    );
                });

                expect(
                    failures.length,
                    `Please investigate blocks ${failures.map((a) => a.blockNum).join(`, `)}; authors  `
                ).to.equal(0);
            },
        });
    },
});
