import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { generateEmptyGenesisData, jumpSessions } from "utils";

describeSuite({
    id: "DEVT1601",
    title: "ContainerRegistrar <> relay Registrar",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let emptyGenesisData: any;

        beforeAll(() => {
            alice = context.keyring.alice;
            charlie = context.keyring.alice;
            polkadotJs = context.pjsApi;
            emptyGenesisData = () => {
                const g = polkadotJs.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
                    // Code key: 0x3a636f6465 or [58, 99, 111, 100, 101]
                    storage: [
                        {
                            // ":code" key
                            key: "0x3a636f6465",
                            // code value (must be at least 9 bytes length)
                            value: "0x0102030405060708091011",
                        },
                    ],
                    name: "0x436f6e7461696e657220436861696e2032303030",
                    id: "0x636f6e7461696e65722d636861696e2d32303030",
                    forkId: null,
                    extensions: "0x",
                    properties: {
                        tokenMetadata: {
                            tokenSymbol: "0x61626364",
                            ss58Format: 42,
                            tokenDecimals: 12,
                        },
                        isEthereum: false,
                    },
                });
                return g;
            };
        });

        it({
            id: "E01",
            title: "should be able to register paraId",
            test: async () => {
                await context.createBlock();
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(alice)]);
                const tx = await polkadotJs.tx.containerRegistrar
                    .register(2002, containerChainGenesisData, "0x111")
                    .signAsync(alice);

                await context.createBlock([tx], { allowFailures: false });

                await jumpSessions(context, 1);

                // Para should be onboarding now
                const isOnboarding = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isOnboarding.toString()).to.eq("Onboarding");

                // Accept validation code so that para is onboarded after 2 sessions
                const tx2 = polkadotJs.tx.paras.addTrustedValidationCode("0x0102030405060708091011");
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2).signAsync(alice)], {
                    allowFailures: false,
                });

                await jumpSessions(context, 2);

                // Para should be a parathread now
                const isParathread = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isParathread.toString()).to.eq("Parathread");

                // Check that the on chain genesis data is set correctly
                const onChainGenesisData = await polkadotJs.query.containerRegistrar.paraGenesisData(2002);
                expect(containerChainGenesisData.toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const profileTx = polkadotJs.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx3 = polkadotJs.tx.dataPreservers.startAssignment(profileId, 2002, "Free");

                // Mark the paraId valid for collating
                const tx4 = polkadotJs.tx.containerRegistrar.markValidForCollating(2002);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock(
                    [
                        await profileTx.signAsync(charlie),
                        await tx3.signAsync(alice, { nonce: nonce.addn(1) }),
                        await polkadotJs.tx.sudo.sudo(tx4).signAsync(alice, { nonce: nonce.addn(2) }),
                    ],
                    {
                        allowFailures: false,
                    }
                );

                await jumpSessions(context, 2);

                // Para should be a parachain now
                const isParachain = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isParachain.toString()).to.eq("Parachain");

                // Expect all paraIds to be registered (genesis ones + 2002)
                const parasRegistered = await polkadotJs.query.containerRegistrar.registeredParaIds();
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2002]);
            },
        });

        it({
            id: "E02",
            title: "should not be able to register paraId twice",
            test: async () => {
                // Check we can't register via relay Registrar
                const tx2 = polkadotJs.tx.containerRegistrar
                    .register(2002, generateEmptyGenesisData(context.pjsApi), "0x0102030405060708091011")
                    .signAsync(alice);
                const { result: result2 } = await context.createBlock([tx2]);
                expect(result2[0].successful).to.be.false;
                expect(result2[0].error.section).to.eq("containerRegistrar");
                expect(result2[0].error.name).to.eq("ParaIdAlreadyRegistered");
            },
        });

        it({
            id: "E03",
            title: "ContainerRegistrar::deregister should offboard the paraId",
            test: async () => {
                // Para should still be a parachain
                const isParachain = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isParachain.toString()).to.eq("Parachain");

                const tx = polkadotJs.tx.containerRegistrar.deregister(2002);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)], {
                    allowFailures: false,
                });

                await jumpSessions(context, 2);

                // Check that the on chain genesis data is now empty
                const onChainGenesisDataAfter = await polkadotJs.query.containerRegistrar.paraGenesisData(2002);
                expect(onChainGenesisDataAfter.toHuman()).to.be.null;

                await context.createBlock();
                // Para should be offboarding
                const isOffboarding = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isOffboarding.toString()).to.eq("OffboardingParathread");
            },
        });

        it({
            id: "E04",
            title: "should not be able to register through relay",
            test: async () => {
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const tx = polkadotJs.tx.registrar
                    .register(4000, containerChainGenesisData, "0x0102030405060708091011")
                    .signAsync(alice);

                const { result } = await context.createBlock([tx]);
                expect(result[0].successful).to.be.false;
                expect(result[0].error.section).to.eq("system");
                expect(result[0].error.name).to.eq("CallFiltered");
            },
        });

        it({
            id: "E05",
            title: "should not be able to deregister through relay",
            test: async () => {
                const tx = polkadotJs.tx.registrar.deregister(4000).signAsync(alice);

                const { result } = await context.createBlock([tx]);
                expect(result[0].successful).to.be.false;
                expect(result[0].error.section).to.eq("system");
                expect(result[0].error.name).to.eq("CallFiltered");
            },
        });
    },
});
