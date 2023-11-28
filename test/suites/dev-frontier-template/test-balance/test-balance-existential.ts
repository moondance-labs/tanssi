import { TransactionTypes, beforeEach, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, MIN_GAS_PRICE, createRawTransfer } from "@moonwall/util";
import { PrivateKeyAccount } from "viem";
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";

describeSuite({
    id: "DF0101",
    title: "Existential Deposit disabled",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let randomAccount: PrivateKeyAccount;
        let privateKey: `0x${string}`;

        beforeEach(async function () {
            privateKey = generatePrivateKey();
            randomAccount = privateKeyToAccount(privateKey);
            const { result } = await context.createBlock(
                context.polkadotJs().tx.balances.transferAllowDeath(randomAccount.address, 10_000_000_000_000_000_000n)
            );
            expect(result!.successful, result!.error?.name).to.be.true;
        });

        for (const txnType of TransactionTypes) {
            it({
                id: `T0${TransactionTypes.indexOf(txnType) + 1}`,
                title: `full ${txnType} transfer should not reap on 0 account balance`,
                test: async function () {
                    const gasPrice = (await context.polkadotJs().rpc.eth.gasPrice()).toBigInt();
                    const raw = await createRawTransfer(
                        context,
                        ALITH_ADDRESS,
                        10_000_000_000_000_000_000n - 21000n * gasPrice,
                        {
                            privateKey,
                            type: txnType,
                            gasPrice: gasPrice,
                            gas: 21000n,
                            maxFeePerGas: gasPrice,
                        }
                    );
                    const { result } = await context.createBlock(raw);

                    expect(result!.successful, result!.error?.name).toBe(true);

                    expect(await context.viem("public").getBalance({ address: randomAccount.address })).toBe(0n);
                },
            });
        }

        it({
            id: "T04",
            title: "should not reap on tiny balance",
            test: async function () {
                const randomAccountBalance = await context
                    .viem("public")
                    .getBalance({ address: randomAccount.address });
                const rawTxn = await context.createTxn!({
                    to: BALTATHAR_ADDRESS,
                    privateKey,
                    txnType: "legacy",
                    value: randomAccountBalance - 1n - 21000n * MIN_GAS_PRICE,
                    gasLimit: 21000n,
                    gasPrice: MIN_GAS_PRICE,
                });

                await context.createBlock(rawTxn);
                expect(await context.viem("public").getBalance({ address: randomAccount.address })).toBe(1n);
                expect(await context.viem("public").getTransactionCount({ address: randomAccount.address })).toBe(1);
            },
        });

        it({
            id: "T05",
            title: "runtime constant should be set to zero",
            test: async function () {
                const existentialDeposit = context.polkadotJs().consts.balances.existentialDeposit.toBigInt();
                expect(existentialDeposit).toBe(0n);
            },
        });
    },
});
