import "@tanssi/api-augment";
import { deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import {
    ALITH_ADDRESS,
    ALITH_PRIVATE_KEY,
    BALTATHAR_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
    alith,
    createViemTransaction,
    sendRawTransaction,
} from "@moonwall/util";
import { encodeFunctionData } from "viem";

describeSuite({
    id: "DE0201",
    title: "Test Contract - Deployment Filter",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "Any account can deploy (CREATE) in default mode",
            test: async () => {
                const {
                    abi: fooAlithAbi,
                    contractAddress: contractAddressAlith,
                    hash: contractHashAlith,
                    status: contractStatusAlith,
                } = await deployCreateCompiledContract(context, "Foo", { privateKey: ALITH_PRIVATE_KEY });

                expect(fooAlithAbi).toBeTruthy();
                expect(contractAddressAlith).toBeTruthy();
                expect(contractHashAlith).toBeTruthy();
                expect(contractStatusAlith).to.eq("success");

                const {
                    abi: fooBaltatharAbi,
                    contractAddress: contractAddressBaltathar,
                    hash: contractHashBaltathar,
                    status: contractStatusBaltathar,
                } = await deployCreateCompiledContract(context, "Foo", { privateKey: BALTATHAR_PRIVATE_KEY });

                expect(fooBaltatharAbi).toBeTruthy();
                expect(contractAddressBaltathar).toBeTruthy();
                expect(contractAddressBaltathar).to.not.eq(contractAddressAlith);
                expect(contractHashBaltathar).toBeTruthy();
                expect(contractStatusBaltathar).to.eq("success");
            },
        });

        it({
            id: "T02",
            title: "Only allowed address can deploy (CREATE) after changing parameters",
            test: async () => {
                const deployFilter = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeDeployFilter", { Whitelisted: [ALITH_ADDRESS] });
                const allowedAddressesToCreate = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeDynamicParamsContractDeployFilterParameters", {
                        AllowedAddressesToCreate: [null, deployFilter],
                    });
                const runtimeParameters = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeRuntimeParameters", {
                        ContractDeployFilter: allowedAddressesToCreate,
                    });

                // parameters.setParameter() call to allow Alith (0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac)
                // to deploy contracts via CREATE.
                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(context.polkadotJs().tx.parameters.setParameter(runtimeParameters.toU8a()))
                        .signAsync(alith),
                    { allowFailures: false }
                );

                // Alith can deploy "Foo".
                const {
                    abi: fooAlithAbi,
                    contractAddress: contractAddressAlith,
                    hash: contractHashAlith,
                    status: contractStatusAlith,
                } = await deployCreateCompiledContract(context, "Foo", { privateKey: ALITH_PRIVATE_KEY });

                expect(fooAlithAbi).toBeTruthy();
                expect(contractAddressAlith).toBeTruthy();
                expect(contractHashAlith).toBeTruthy();
                expect(contractStatusAlith).to.eq("success");

                // Baltathar is forbidden to deploy after changing the configuration params.
                try {
                    await deployCreateCompiledContract(context, "Foo", { privateKey: BALTATHAR_PRIVATE_KEY });
                } catch (error) {
                    return expect(error.details).to.be.eq(
                        // pallet-evm (index 61): CreateOriginNotAllowed error (index 13)
                        "execution fatal: Module(ModuleError { index: 61, error: [13, 0, 0, 0], message: None })"
                    );
                }

                expect.fail("Expected the previous contract deployment to fail");
            },
        });

        it({
            id: "T03",
            title: "Any account can deploy CALL(CREATE) in default mode",
            test: async () => {
                // First Alith deploys "Foo", which then will be used to deploy
                // the inner contract "Bar".
                const {
                    abi: fooAbi,
                    contractAddress: contractFooAddress,
                    hash: contractFooHash,
                    status: contractFooStatus,
                } = await deployCreateCompiledContract(context, "Foo", { privateKey: ALITH_PRIVATE_KEY });

                expect(fooAbi).toBeTruthy();
                expect(contractFooAddress).toBeTruthy();
                expect(contractFooHash).toBeTruthy();
                expect(contractFooStatus).to.eq("success");

                let aliceNonce = (await context.polkadotJs().query.system.account(ALITH_ADDRESS)).nonce.toNumber();

                // Alith can perform inner deployments.
                const alithInnerDeployTx = await createViemTransaction(context, {
                    to: contractFooAddress,
                    nonce: aliceNonce++,
                    data: encodeFunctionData({
                        abi: fooAbi,
                        functionName: "newBar",
                        args: [],
                    }),
                    privateKey: ALITH_PRIVATE_KEY,
                });

                const alithTxResult = await sendRawTransaction(context, alithInnerDeployTx);
                await context.createBlock();

                const alithTxReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: alithTxResult as `0x${string}` });
                expect(alithTxReceipt.status).to.eq("success");

                // Baltathar can also perform inner deployments.
                let baltatharNonce = (
                    await context.polkadotJs().query.system.account(BALTATHAR_ADDRESS)
                ).nonce.toNumber();
                const baltatharInnerDeployTx = await createViemTransaction(context, {
                    to: contractFooAddress,
                    nonce: baltatharNonce++,
                    data: encodeFunctionData({
                        abi: fooAbi,
                        functionName: "newBar",
                        args: [],
                    }),
                    privateKey: BALTATHAR_PRIVATE_KEY,
                });

                const baltatharTxResult = await sendRawTransaction(context, baltatharInnerDeployTx);
                await context.createBlock();

                const baltatharTxReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: baltatharTxResult as `0x${string}` });
                expect(baltatharTxReceipt.status).to.eq("success");
            },
        });

        it({
            id: "T04",
            title: "Only allowed address can deploy CALL(CREATE) after changing parameters",
            test: async () => {
                const deployFilter = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeDeployFilter", { Whitelisted: [ALITH_ADDRESS] });
                const allowedAddressesToCreateInner = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeDynamicParamsContractDeployFilterParameters", {
                        AllowedAddressesToCreateInner: [null, deployFilter],
                    });
                const runtimeParameters = context
                    .polkadotJs()
                    .createType("ContainerChainTemplateFrontierRuntimeRuntimeParameters", {
                        ContractDeployFilter: allowedAddressesToCreateInner,
                    });

                // parameters.setParameter() call to allow Alith (0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac)
                // to deploy inner contracts via CALL(CREATE).
                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(context.polkadotJs().tx.parameters.setParameter(runtimeParameters.toU8a()))
                        .signAsync(alith),
                    { allowFailures: false }
                );

                // First Alith deploys "Foo", which then will be used to deploy
                // the inner contract "Bar".
                const {
                    abi: fooAbi,
                    contractAddress: contractFooAddress,
                    hash: contractFooHash,
                    status: contractFooStatus,
                } = await deployCreateCompiledContract(context, "Foo", { privateKey: ALITH_PRIVATE_KEY });

                expect(contractFooHash).toBeTruthy();
                expect(contractFooStatus).to.eq("success");

                let aliceNonce = (await context.polkadotJs().query.system.account(ALITH_ADDRESS)).nonce.toNumber();

                // Alith can perform inner deployments.
                const alithInnerDeployTx = await createViemTransaction(context, {
                    to: contractFooAddress,
                    nonce: aliceNonce++,
                    data: encodeFunctionData({
                        abi: fooAbi,
                        functionName: "newBar",
                        args: [],
                    }),
                    privateKey: ALITH_PRIVATE_KEY,
                });

                const alithTxResult = await sendRawTransaction(context, alithInnerDeployTx);
                await context.createBlock();

                const alithTxReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: alithTxResult as `0x${string}` });
                expect(alithTxReceipt.status).to.eq("success");

                // Baltathar can't perform inner deployments anymore.
                let baltatharNonce = (
                    await context.polkadotJs().query.system.account(BALTATHAR_ADDRESS)
                ).nonce.toNumber();
                try {
                    await createViemTransaction(context, {
                        to: contractFooAddress,
                        nonce: baltatharNonce++,
                        data: encodeFunctionData({
                            abi: fooAbi,
                            functionName: "newBar",
                            args: [],
                        }),
                        privateKey: BALTATHAR_PRIVATE_KEY,
                    });
                } catch (error) {
                    return expect(error.details).to.be.eq("VM Exception while processing transaction: revert");
                }

                expect.fail("Expected the previous contract deployment to fail");
            },
        });
    },
});
