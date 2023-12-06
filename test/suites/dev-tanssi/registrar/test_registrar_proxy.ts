import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { stringToHex } from "@polkadot/util";

describeSuite({
    id: "DT0605",
    title: "Registrar test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Can add registrar proxy and use it",
            test: async function () {
                // Setup proxy
                const delegate = charlie.address;
                const registrar_proxy = 6;
                const delay = 0;
                const tx = polkadotJs.tx.proxy.addProxy(delegate, registrar_proxy, delay);
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "ProxyAdded";
                });
                expect(ev1.length).to.be.equal(1);

                const proxies = await polkadotJs.query.proxy.proxies(alice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate: charlie.address,
                        proxyType: "Registrar",
                        delay: 0,
                    },
                ]);

                // Use proxy
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

                // assert we can inject on chain data with proxy
                const tx2 = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.registrar.register(2002, containerChainGenesisData)
                );
                await context.createBlock([await tx2.signAsync(charlie)]);
                // Check that the on chain genesis data is set correctly
                const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2002);
                // TODO: fix once we have types
                expect(emptyGenesisData().toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                // assert we can inject bootnodes with proxy
                const tx3 = polkadotJs.tx.proxy.proxy(
                    alice.address,
                    null,
                    polkadotJs.tx.registrar.setBootNodes(2002, ["dummy"])
                );
                await context.createBlock([await tx3.signAsync(charlie)]);

                // Check that the on chain genesis data is set correctly
                const onChainBootnodes = await polkadotJs.query.registrar.bootNodes(2002);
                // TODO: fix once we have types
                expect(onChainBootnodes.toHuman()).to.deep.equal(["dummy"]);
            },
        });
    },
});
