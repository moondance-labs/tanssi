import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { getAuthorFromDigest } from "util/author";
import { fetchIssuance, filterRewardFromOrchestrator, fetchRewardAuthorContainers, fetchWithdrawnAmount, fetchDepositedAmount } from "util/block";
import { PARACHAIN_BOND } from "util/constants";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 2 * 60 * 60 * 1000;
const hours = (timePeriod / (1000 * 60 * 60)).toFixed(2);
const atBlock = process.env.AT_BLOCK ? Number(process.env.AT_BLOCK) : -1;

describeSuite({
    id: "S08",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
            const getBlockData = async (blockNum: number) => {
                log(`Collecting ${hours} hours worth of block data`);
                const blockHash = await api.rpc.chain.getBlockHash(blockNum);
                const signedBlock = await api.rpc.chain.getBlock(blockHash);
                const apiAt = await api.at(blockHash);
                const weights = await apiAt.query.system.blockWeight();
                const receipts = (await apiAt.query.ethereum.currentReceipts()).unwrapOr([]);
                const events = await apiAt.query.system.events();
                return {
                    blockNum: blockNum,
                    nextFeeMultiplier: await apiAt.query.transactionPayment.nextFeeMultiplier(),
                    extrinsics: signedBlock.block.extrinsics,
                    weights,
                    receipts,
                    events,
                };
            };
        });

        it({
            id: "C03",
            title: "Supply variance is correct",
            test: async function () {
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const withdrawnAmount = await fetchWithdrawnAmount(events);
                const depositAmount = await fetchDepositedAmount(events);

                const supplyAfter = (await apiAtIssuanceAfter.query.balances.totalIssuance()).toBigInt();
                // we know there might be rounding errors, so we always check it is in the range +-1
                expect(supplyAfter).to.equal(supplyBefore + depositAmount - withdrawnAmount);
            },
        });
    },
});
