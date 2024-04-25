import { describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import { ALITH_ADDRESS, ALITH_PRIVATE_KEY, customWeb3Request, generateKeyringPair } from "@moonwall/util";

describeSuite({
    id: "DF0301",
    title: "Ethereum Block - Pending",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        const TEST_ACCOUNT = "0x1111111111111111111111111111111111111111";

        it({
            id: "T01",
            title: "should return pending block",
            test: async function () {
                let nonce = 0;
                const sendTransaction = async () => {
                    const gasPrice = (await context.polkadotJs().rpc.eth.gasPrice()).toBigInt();
                    const tx = await context.web3().eth.accounts.signTransaction(
                        {
                            from: ALITH_ADDRESS,
                            to: TEST_ACCOUNT,
                            value: "0x200", // Must be higher than ExistentialDeposit
                            gasPrice: gasPrice,
                            gas: "0x100000",
                            nonce: nonce,
                        },
                        ALITH_PRIVATE_KEY
                    );
                    nonce = nonce + 1;
                    return (await customWeb3Request(context.web3(), "eth_sendRawTransaction", [tx.rawTransaction]))
                        .result;
                };

                // block 1 send 5 transactions
                const expectedXtsNumber = 5;
                // eslint-disable-next-line @typescript-eslint/no-unused-vars
                for (const _ of Array(expectedXtsNumber)) {
                    await sendTransaction();
                }

                // test still invalid future transactions can be safely applied (they are applied, just not overlayed)
                nonce = nonce + 100;
                await sendTransaction();

                // do not seal, get pendign block
                let pending_transactions = [];
                {
                    const pending = (
                        await customWeb3Request(context.web3(), "eth_getBlockByNumber", ["pending", false])
                    ).result;
                    expect(pending.hash).to.be.null;
                    expect(pending.miner).to.be.null;
                    expect(pending.nonce).to.be.null;
                    expect(pending.totalDifficulty).to.be.null;
                    pending_transactions = pending.transactions;
                    expect(pending_transactions.length).to.be.eq(expectedXtsNumber);
                }

                // seal and compare latest blocks transactions with the previously pending
                await context.createBlock();
                const latest_block = await context.web3().eth.getBlock("latest", false);
                expect(pending_transactions).to.be.deep.eq(latest_block.transactions);
            },
        });

        it({
            id: "T02",
            title: "should be able to estimate gas with pending block with transfers",
            test: async function () {
                const randomAccount = generateKeyringPair();
                const randomAddress = randomAccount.address as `0x${string}`;
                const estimatedGas = await context.viem().estimateGas({
                    account: ALITH_ADDRESS,
                    value: 10_000_000_000_000_000_000n,
                    to: randomAddress,
                    blockTag: "pending",
                });
                expect(estimatedGas, "Estimated bal transfer incorrect").toBe(21000n);
            },
        });

        it({
            id: "T03",
            title: "should be able to estimate gas with pending block with contract creators",
            test: async function () {
                const { bytecode } = fetchCompiledContract("MultiplyBy7");
                expect(
                    await context.viem().estimateGas({
                        account: ALITH_ADDRESS,
                        data: bytecode,
                        blockTag: "pending",
                    })
                ).to.toBeGreaterThan(21000n);
            },
        });
    },
});
