import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { BN } from "@polkadot/util";
import { getTreasuryAddress } from "../../utils";

const RUNTIME_VERSION_THRESHOLD = 1300;
const BLOCKS_AMOUNT_TO_CHECK = 100;

describeSuite({
    id: "S07",
    title: "Check if fees go to treasury for all runtimes",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Check if fees go to treasury",
            test: async () => {
                const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                const treasuryAddress = getTreasuryAddress(api);

                for (let i = 1; i <= BLOCKS_AMOUNT_TO_CHECK; i++) {
                    const blockNumber = currentBlock - i;
                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const block = await api.rpc.chain.getBlock(blockHash);
                    const extrinsics = block.block.extrinsics;

                    if (extrinsics.length === 0) {
                        log(`No extrinsics for block ${blockNumber}, skipping...`);
                        continue;
                    }
                    const apiAtBlock = await api.at(blockHash);
                    const specVersion = apiAtBlock.consts.system.version.specVersion.toNumber();

                    if (specVersion < RUNTIME_VERSION_THRESHOLD) {
                        log(
                            `Skip tests for runtimes before ${RUNTIME_VERSION_THRESHOLD}. Current runtime: ${specVersion}`
                        );
                        return;
                    }

                    const prevBlockApi = await api.at(await api.rpc.chain.getBlockHash(blockNumber - 1));
                    const treasureBalanceBefore = (
                        await prevBlockApi.query.system.account(treasuryAddress)
                    ).data.free.toBn();
                    const treasureBalanceAfter = (
                        await apiAtBlock.query.system.account(treasuryAddress)
                    ).data.free.toBn();

                    // Expected treasury deposit for the current block
                    const treasuryDeposit = treasureBalanceAfter.sub(treasureBalanceBefore);
                    // Accumulated fees and tips for the current block
                    let totalFee = new BN(0);

                    const events = await apiAtBlock.query.system.events();

                    for (const [index, extrinsic] of extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        for (const { event, phase } of events) {
                            if (phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index)) {
                                if (event.section === "transactionPayment" && event.method === "TransactionFeePaid") {
                                    const [_, actualFee, tip] = event.data;
                                    const fee = (actualFee as any).toBn();
                                    const tipBn = (tip as any).toBn();
                                    totalFee = totalFee.add(fee).add(tipBn);
                                }
                            }
                        }
                    }

                    if (!totalFee.isZero() || !treasuryDeposit.isZero()) {
                        expect(
                            totalFee,
                            `Total fee (${totalFee.toString()}) should equal Treasury Deposit (${treasuryDeposit.toString()}) for block: ${blockNumber}`
                        ).toEqual(treasuryDeposit);
                    } else {
                        log(`Skip for block number: ${blockNumber} as it has no fees`);
                    }
                }
            },
        });
    },
});
