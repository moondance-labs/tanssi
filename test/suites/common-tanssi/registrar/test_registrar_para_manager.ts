import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0502",
    title: "Registrar para manager",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        const paraId = 2002;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that registering paraIds is possible",
            test: async function () {
                await context.createBlock();

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const emptyGenesisData = () => {
                    const g = polkadotJs.createType("TpContainerChainGenesisDataContainerChainGenesisData", {
                        storage: [
                            {
                                key: "0x636f6465",
                                value: "0x010203040506",
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
                const bootNodes = [
                    "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                ];

                await context.createBlock([
                    await polkadotJs.tx.registrar.register(paraId, containerChainGenesisData).signAsync(alice),
                ]);

                // Bob still not a manager, extrinsic requiring ManagerOrigin should fail
                const { result } = await context.createBlock(
                    await polkadotJs.tx.servicesPayment.setRefundAddress(paraId, bob.address).signAsync(bob),
                );
                expect(result.successful).to.be.false;

                // Set bob as manager
                await context.createBlock([
                    await polkadotJs.tx.registrar.setParaManager(paraId, bob.address).signAsync(alice),
                ]);

                // Extrinsic should succeed now
                const { result: result2 } = await context.createBlock(
                    await polkadotJs.tx.servicesPayment.setRefundAddress(paraId, bob.address).signAsync(bob),
                );

                expect(result2.successful).to.be.true;

            },
        });
    },
});
