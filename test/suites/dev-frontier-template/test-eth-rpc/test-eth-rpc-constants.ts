import { describeSuite, expect } from "@moonwall/cli";
import { customWeb3Request } from "@moonwall/util";

describeSuite({
    id: "DF0601",
    title: "RPC Constants",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        it({
            id: "T01",
            title: "should have 0 hashrate",
            test: async function () {
                expect(BigInt((await customWeb3Request(context.web3(), "eth_hashrate", [])).result)).toBe(0n);
            },
        });

        it({
            id: "T02",
            title: "should have chainId 1281",
            test: async function () {
                expect(BigInt((await customWeb3Request(context.web3(), "eth_chainId", [])).result)).toBe(1281n);
            },
        });

        it({
            id: "T03",
            title: "should have no accounts",
            test: async function () {
                expect((await customWeb3Request(context.web3(), "eth_accounts", [])).result).toStrictEqual([]);
            },
        });

        it({
            id: "T04",
            title: "block author should be 0x0000000000000000000000000000000000000000",
            test: async function () {
                expect((await customWeb3Request(context.web3(), "eth_coinbase", [])).result).toBe(
                    "0x0000000000000000000000000000000000000000"
                );
            },
        });
    },
});
