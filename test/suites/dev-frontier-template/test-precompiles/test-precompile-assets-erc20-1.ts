import "@moonbeam-network/api-augment";
import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, alith } from "@moonwall/util";
import { u16 } from "@polkadot/types-codec";
import { Abi } from "viem";
import { mockAssetCreation, relayAssetMetadata } from "../../../helpers/assets.ts";
import { RELAY_SOURCE_LOCATION } from "../../../util/constants.ts";

describeSuite({
    id: "DF1101",
    title: "Precompiles - Assets-ERC20",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let erc20Abi: Abi;
        let assetId: u16;
        let contractInstanceAddress: `0x${string}`;

        const ADDRESS_ERC20 = "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF000f";
        const ASSET_ID = 15n;

        beforeAll(async () => {
            assetId = context.polkadotJs().createType("u16", ASSET_ID);

            await mockAssetCreation(
                context,
                alith,
                assetId,
                ALITH_ADDRESS,
                RELAY_SOURCE_LOCATION,
                relayAssetMetadata,
                true
            );

            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, 2000000000000000000000n)
            );

            const { abi, contractAddress } = await deployCreateCompiledContract(context, "ERC20Instance");
            erc20Abi = abi;
            contractInstanceAddress = contractAddress;
        });

        it({
            id: "T01",
            title: "allows to call name",
            test: async function () {
                const name = await context.viem().readContract({
                    address: ADDRESS_ERC20,
                    abi: erc20Abi,
                    functionName: "name",
                });

                expect(name).equals("DOT");
            },
        });

        it({
            id: "T02",
            title: "allows to call name via wrapper",
            test: async function () {
                const name = await context.viem().readContract({
                    address: contractInstanceAddress,
                    abi: erc20Abi,
                    functionName: "name",
                });

                expect(name).equals("DOT");
            },
        });

        it({
            id: "T03",
            title: "allows to call symbol",
            test: async function () {
                const symbol = await context.viem().readContract({
                    address: ADDRESS_ERC20,
                    abi: erc20Abi,
                    functionName: "symbol",
                });
                expect(symbol).equals("DOT");
            },
        });

        it({
            id: "T04",
            title: "allows to call symbol via wrapper",
            test: async function () {
                const symbol = await context.viem().readContract({
                    address: contractInstanceAddress,
                    abi: erc20Abi,
                    functionName: "symbol",
                });
                expect(symbol).equals("DOT");
            },
        });

        it({
            id: "T05",
            title: "allows to call decimals",
            test: async function () {
                const decimals = await context.viem().readContract({
                    address: ADDRESS_ERC20,
                    abi: erc20Abi,
                    functionName: "decimals",
                });
                expect(decimals).equals(12);
            },
        });

        it({
            id: "T06",
            title: "allows to call decimals via wrapper",
            test: async function () {
                const decimals = await context.viem().readContract({
                    address: contractInstanceAddress,
                    abi: erc20Abi,
                    functionName: "decimals",
                });
                expect(decimals).equals(12);
            },
        });

        it({
            id: "T07",
            title: "allows to call getBalance",
            test: async function () {
                const data = await context.viem().readContract({
                    address: ADDRESS_ERC20,
                    abi: erc20Abi,
                    functionName: "balanceOf",
                    args: [ALITH_ADDRESS],
                });
                expect(data).equals(2000000000000000000000n);
            },
        });

        it({
            id: "T08",
            title: "allows to call getBalance via wrapper",
            test: async function () {
                const data = await context.viem().readContract({
                    address: contractInstanceAddress,
                    abi: erc20Abi,
                    functionName: "balanceOf",
                    args: [ALITH_ADDRESS],
                });
                expect(data).equals(2000000000000000000000n);
            },
        });

        it({
            id: "T09",
            title: "allows to call totalSupply",
            test: async function () {
                const data = await context.viem().readContract({
                    address: ADDRESS_ERC20,
                    abi: erc20Abi,
                    functionName: "totalSupply",
                });
                expect(data).equals(2000000000000000000000n);
            },
        });

        it({
            id: "T10",
            title: "allows to call totalSupply via wrapper",
            test: async function () {
                const data = await context.viem().readContract({
                    address: contractInstanceAddress,
                    abi: erc20Abi,
                    functionName: "totalSupply",
                });
                expect(data).equals(2000000000000000000000n);
            },
        });
    },
});
