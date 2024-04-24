import { beforeEach, describeSuite, expect } from "@moonwall/cli";
import {
    ALITH_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
    CHARLETH_ADDRESS,
    baltathar,
    ETHAN_ADDRESS,
    BALTATHAR_ADDRESS,
} from "@moonwall/util";
import { expectEVMResult } from "helpers";
import { PrivateKeyAccount, keccak256, pad, parseEther, toBytes, toHex } from "viem";
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";

const IERC20_ADDRESS = "0x0000000000000000000000000000000000000800";

describeSuite({
    id: "DF1111",
    title: "Precompiles - ERC20 Native",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let randomAccount: PrivateKeyAccount;

        beforeEach(async () => {
            randomAccount = privateKeyToAccount(generatePrivateKey());
        });

        it({
            id: "T01",
            title: "allows to call balanceOf",
            test: async function () {
                const transferAmount = 1000n;
                const signedTx = context
                    .polkadotJs()
                    .tx.balances.transferAllowDeath(ETHAN_ADDRESS, transferAmount)
                    .signAsync(baltathar);
                await context.createBlock(signedTx);

                const balance = await context.readContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "balanceOf",
                    args: [ETHAN_ADDRESS],
                });

                expect(balance).equals(transferAmount);
            },
        });

        it({
            id: "T02",
            title: "allows to call totalSupply",
            test: async function () {
                const totalSupply = await context.readContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "totalSupply",
                });

                const totalIssuance = (await context.polkadotJs().query.balances.totalIssuance()).toBigInt();
                expect(totalSupply).toBe(totalIssuance);
            },
        });

        it({
            id: "T03",
            title: "allows to approve transfers, and allowance matches",
            test: async function () {
                const allowanceBefore = (await context.readContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "allowance",
                    args: [ALITH_ADDRESS, BALTATHAR_ADDRESS],
                })) as bigint;

                const amount = parseEther("10");

                const rawTx = await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "approve",
                    args: [BALTATHAR_ADDRESS, amount],
                    rawTxOnly: true,
                });
                const { result } = await context.createBlock(rawTx);

                const allowanceAfter = (await context.readContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "allowance",
                    args: [ALITH_ADDRESS, BALTATHAR_ADDRESS],
                })) as bigint;

                expect(allowanceAfter - allowanceBefore).equals(BigInt(amount));

                const { status, logs } = await context
                    .viem()
                    .getTransactionReceipt({ hash: result?.hash as `0x${string}` });

                expect(status).to.equal("success");
                expect(logs.length).to.eq(1);
                expect(logs[0].topics[0]).toBe(keccak256(toBytes("Approval(address,address,uint256)")));
                expect(logs[0].topics[1]?.toLowerCase()).toBe(pad(ALITH_ADDRESS.toLowerCase() as `0x${string}`));
                expect(logs[0].topics[2]?.toLowerCase()).toBe(pad(BALTATHAR_ADDRESS.toLowerCase() as `0x${string}`));
            },
        });

        it({
            id: "T04",
            title: "allows to call transfer",
            test: async function () {
                expect(
                    await context.readContract!({
                        contractAddress: IERC20_ADDRESS,
                        contractName: "IERC20",
                        functionName: "balanceOf",
                        args: [randomAccount.address],
                    })
                ).equals(0n);

                const balanceBefore = await context.viem().getBalance({ address: BALTATHAR_ADDRESS });

                const rawTx = await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "transfer",
                    args: [randomAccount.address, parseEther("3")],
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    rawTxOnly: true,
                });
                const { result } = await context.createBlock(rawTx);
                const { status, gasUsed } = await context
                    .viem()
                    .getTransactionReceipt({ hash: result?.hash as `0x${string}` });
                expect(status).to.equal("success");

                const balanceAfter = await context.viem().getBalance({ address: BALTATHAR_ADDRESS });
                const block = await context.viem().getBlock();
                const fees = ((gasUsed as bigint) * block.baseFeePerGas!) as bigint;
                expect(balanceAfter).toBeLessThanOrEqual(balanceBefore - parseEther("3") - fees);
                expect(await context.viem().getBalance({ address: randomAccount.address })).to.equal(parseEther("3"));
            },
        });

        it({
            id: "T05",
            title: "allows to approve transfer and use transferFrom",
            test: async function () {
                const allowedAmount = parseEther("10");
                const transferAmount = parseEther("5");

                await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "approve",
                    args: [BALTATHAR_ADDRESS, allowedAmount],
                });
                await context.createBlock();

                const fromBalBefore = (
                    await context.polkadotJs().query.system.account(ALITH_ADDRESS)
                ).data.free.toBigInt();
                const toBalBefore = await context.viem().getBalance({ address: CHARLETH_ADDRESS });

                const rawTx = await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "transferFrom",
                    args: [ALITH_ADDRESS, CHARLETH_ADDRESS, transferAmount],
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    rawTxOnly: true,
                });

                const { result } = await context.createBlock(rawTx);
                const { logs, status } = await context
                    .viem()
                    .getTransactionReceipt({ hash: result?.hash as `0x${string}` });

                const fromBalAfter = (
                    await context.polkadotJs().query.system.account(ALITH_ADDRESS)
                ).data.free.toBigInt();

                const toBalAfter = await context.viem().getBalance({ address: CHARLETH_ADDRESS });
                expect(logs.length).to.eq(1);
                expect(logs[0].address).to.eq(IERC20_ADDRESS);
                expect(logs[0].data).to.eq(pad(toHex(transferAmount)));
                expect(logs[0].topics.length).to.eq(3);
                expect(logs[0].topics[0]).toBe(keccak256(toBytes("Transfer(address,address,uint256)")));
                expect(logs[0].topics[1]?.toLowerCase()).toBe(pad(ALITH_ADDRESS.toLowerCase() as `0x${string}`));
                expect(logs[0].topics[2]?.toLowerCase()).toBe(pad(CHARLETH_ADDRESS.toLowerCase() as `0x${string}`));
                expect(status).to.equal("success");
                expect(toBalAfter).toBe(toBalBefore + transferAmount);
                expect(fromBalAfter).toBe(fromBalBefore - transferAmount);
                const newAllowedAmount = allowedAmount - transferAmount;
                expect(
                    await context.readContract!({
                        contractAddress: IERC20_ADDRESS,
                        contractName: "IERC20",
                        functionName: "allowance",
                        args: [ALITH_ADDRESS, BALTATHAR_ADDRESS],
                    })
                ).toBe(newAllowedAmount);
            },
        });

        it({
            id: "T06",
            title: "refuses to transferFrom more than allowed",
            test: async function () {
                const allowedAmount = parseEther("10");
                const transferAmount = parseEther("15");

                await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "approve",
                    args: [BALTATHAR_ADDRESS, allowedAmount],
                });
                await context.createBlock();

                const fromBalBefore = (
                    await context.polkadotJs().query.system.account(ALITH_ADDRESS)
                ).data.free.toBigInt();
                const toBalBefore = await context.viem().getBalance({ address: CHARLETH_ADDRESS });

                const rawTxn = await context.writeContract!({
                    contractAddress: IERC20_ADDRESS,
                    contractName: "IERC20",
                    functionName: "transferFrom",
                    args: [ALITH_ADDRESS, CHARLETH_ADDRESS, transferAmount],
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    rawTxOnly: true,
                    gas: 200_000n,
                    web3Library: "ethers",
                });

                const { result } = await context.createBlock(rawTxn);
                expectEVMResult(result!.events, "Revert");

                const fromBalAfter = (
                    await context.polkadotJs().query.system.account(ALITH_ADDRESS)
                ).data.free.toBigInt();

                const toBalAfter = await context.viem().getBalance({ address: CHARLETH_ADDRESS });
                expect(toBalAfter).toBe(toBalBefore);
                expect(fromBalAfter).toBe(fromBalBefore);
                expect(
                    await context.readContract!({
                        contractAddress: IERC20_ADDRESS,
                        contractName: "IERC20",
                        functionName: "allowance",
                        args: [ALITH_ADDRESS, BALTATHAR_ADDRESS],
                    })
                ).toBe(allowedAmount);
            },
        });
    },
});
