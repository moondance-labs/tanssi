import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "DT0107",
    title: "ContainerRegistrar <> relay Registrar",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            polkadotJs = context.pjsApi;
        });

        it({
            id: "E01",
            title: "should be able to register paraId",
            test: async function () {
                await context.createBlock();

                // Code key: 0x3a636f6465 or [58, 99, 111, 100, 101]
                const emptyGenesisData = () => {
                    const g = polkadotJs.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
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
                const containerChainGenesisData = emptyGenesisData();

                const tx = await polkadotJs.tx.containerRegistrar
                    .register(2002, containerChainGenesisData)
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
                expect(emptyGenesisData().toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                // Mark the paraId valid for collating
                const tx3 = polkadotJs.tx.containerRegistrar.markValidForCollating(2002);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx3).signAsync(alice)], {
                    allowFailures: false,
                });

                await jumpSessions(context, 2);

                // Para should be a parathread now
                const isParachain = await polkadotJs.query.paras.paraLifecycles(2002);
                expect(isParachain.toString()).to.eq("Parachain");

                // Expect all paraIds to be registered (genesis ones + 2002)
                const parasRegistered = await polkadotJs.query.containerRegistrar.registeredParaIds();
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2002]);
            },
        });
    },
});
