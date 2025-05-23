import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { u32 } from "@polkadot/types";
import { initializeCustomCreateBlock, jumpSessions } from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY,
    checkCallIsFiltered,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR,
} from "helpers";

describeSuite({
    id: "DEVT1501",
    title: "Proxy test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudoAlice: KeyringPair;
        let delegateBob: KeyringPair;
        let charlie: KeyringPair;
        const REGISTRAR_PROXY_INDEX = 7;
        let genesisData: any;
        const VALIDATION_CODE = "0x546865205761736d20436f6465";
        const GENESIS_HEAD = "0x6865616465722064617461";
        const FORCED_PARA_ID = 5555;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightProxy: boolean;
        let shouldSkipStarlightRegistrar: boolean;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            sudoAlice = context.keyring.alice;
            delegateBob = context.keyring.bob;
            charlie = context.keyring.charlie;

            polkadotJs = context.polkadotJs();

            genesisData = polkadotJs.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
                storage: [
                    {
                        key: "0x3a636f6465",
                        value: VALIDATION_CODE,
                    },
                ],
                name: "0x54657374696e672070726f78696573",
                id: "0x54657374696e672070726f78696573",
                forkId: null,
                extensions: "0x",
                properties: {
                    tokenMetadata: {
                        tokenSymbol: "0x50524f5859",
                        ss58Format: 42,
                        tokenDecimals: 12,
                    },
                    isEthereum: false,
                },
            });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightProxy = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY.includes(specVersion);
            shouldSkipStarlightRegistrar =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Can add proxy",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.addProxy(delegateBob.address, REGISTRAR_PROXY_INDEX, 0);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudoAlice));
                    return;
                }

                await context.createBlock([await tx.signAsync(sudoAlice)]);

                const proxies = await polkadotJs.query.proxy.proxies(sudoAlice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate: delegateBob.address,
                        proxyType: "SudoRegistrar",
                        delay: 0,
                    },
                ]);
            },
        });

        it({
            id: "E02",
            title: "Delegated account can sudo txs in paras_registrar",
            test: async () => {
                const txReserve = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.registrar.forceRegister(
                            delegateBob.address,
                            50,
                            FORCED_PARA_ID,
                            GENESIS_HEAD,
                            VALIDATION_CODE
                        )
                    )
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await txReserve.signAsync(delegateBob));
                    return;
                }

                await context.createBlock([await txReserve.signAsync(delegateBob)]);

                const registrar_info = await polkadotJs.query.registrar.paras(FORCED_PARA_ID);
                expect(registrar_info.toJSON()).not.toBeNull();
            },
        });

        it({
            id: "E03",
            title: "Delegated account can sudo txs in data preservers, paras, paraSudoWrapper, and registrar",
            test: async () => {
                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.proxy.proxy(sudoAlice.address, null, "0x").signAsync(delegateBob)
                    );
                    return;
                }

                if (shouldSkipStarlightRegistrar) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.registrar.reserve().signAsync(charlie)
                    );
                    return;
                }

                // A regular user registers a new avs

                const txReserve = polkadotJs.tx.registrar.reserve();
                await context.createBlock([await txReserve.signAsync(charlie)]);

                let events = await polkadotJs.query.system.events();
                const reservedEvent = events.filter(
                    ({ event }) => event.method === "Reserved" && event.data[1].toString() === charlie.address
                );
                const reservedParaId = (reservedEvent[0].event.data[0] as u32).toNumber();
                console.log(reservedParaId);

                const txRegisterRelay = polkadotJs.tx.registrar.register(reservedParaId, GENESIS_HEAD, VALIDATION_CODE);
                await context.createBlock([await txRegisterRelay.signAsync(charlie)]);

                const txRegisterTanssi = polkadotJs.tx.containerRegistrar.register(
                    reservedParaId,
                    genesisData,
                    GENESIS_HEAD
                );
                await context.createBlock([await txRegisterTanssi.signAsync(charlie)]);

                await jumpSessions(context, 1);

                // Proxy can add validation code
                const txAddsCode = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.paras.addTrustedValidationCode(VALIDATION_CODE))
                );

                await context.createBlock([await txAddsCode.signAsync(delegateBob)]);
                await jumpSessions(context, 2);

                // Proxy creates a data preserver. "The URL" translates to 0x5468652055524c when scale encoded

                const profile = {
                    url: "The URL",
                    paraIds: { whitelist: [reservedParaId] },
                    mode: "Bootnode",
                };

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const txProfile = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.dataPreservers.forceCreateProfile(profile, delegateBob.address)
                    )
                );
                await context.createBlock([await txProfile.signAsync(delegateBob)]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: delegateBob.address,
                    deposit: 0,
                    profile: {
                        url: "0x5468652055524c",
                        paraIds: { whitelist: [reservedParaId] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });

                // Data preservers need to be assigned before collating

                const txAssignBootnode = polkadotJs.tx.dataPreservers.startAssignment(
                    profileId,
                    reservedParaId,
                    "Free"
                );
                await context.createBlock([await txAssignBootnode.signAsync(charlie)]);

                // Proxy can mark as valid for collating

                const txStartCollating = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.containerRegistrar.markValidForCollating(reservedParaId))
                );
                await context.createBlock([await txStartCollating.signAsync(delegateBob)]);

                events = await polkadotJs.query.system.events();
                const startCollatingEvent = events.filter(
                    ({ event }) =>
                        event.method === "ParaIdValidForCollating" &&
                        event.data[0].toString() === reservedParaId.toString()
                );

                expect(startCollatingEvent.length).eq(1);

                // Proxy can call paraSudoWrapper
                const txCreateChannel = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.parasSudoWrapper.sudoEstablishHrmpChannel(FORCED_PARA_ID, reservedParaId, 1, 1)
                    )
                );
                await context.createBlock([await txCreateChannel.signAsync(delegateBob)]);
                await jumpSessions(context, 1);

                const hrmpChannels = await polkadotJs.query.hrmp.hrmpChannels([FORCED_PARA_ID, reservedParaId]);
                expect(hrmpChannels.toJSON()).not.toBeNull();
            },
        });

        it({
            id: "E04",
            title: "Unauthorized account cannot sudo calls",
            test: async () => {
                // Call adding validation code
                const VALIDATION_CODE_NOT_INCLUDED = "0x4e6f7420676f6e6e61206d616b65206974";

                const txAddsCode = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.paras.addTrustedValidationCode(VALIDATION_CODE_NOT_INCLUDED))
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await txAddsCode.signAsync(charlie));
                    return;
                }
                if (shouldSkipStarlightRegistrar) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(
                        context,
                        polkadotJs,
                        await polkadotJs.tx.registrar.reserve().signAsync(charlie)
                    );
                    return;
                }

                await context.createBlock([await txAddsCode.signAsync(charlie)]);
                await jumpSessions(context, 2);

                const trustedCodes = await polkadotJs.query.paras.codeByHash.entries();
                const noCodeMatching = trustedCodes.filter((code) => {
                    return code[1].toString() === VALIDATION_CODE_NOT_INCLUDED;
                });

                expect(noCodeMatching.length).eq(0);

                // Call upgrading a parathread

                const txUpgrade = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.parasSudoWrapper.sudoScheduleParathreadUpgrade(FORCED_PARA_ID)
                    )
                );
                await context.createBlock([await txUpgrade.signAsync(charlie)]);
                await jumpSessions(context, 1);

                const stillParathread = await polkadotJs.query.paras.paraLifecycles(FORCED_PARA_ID);
                expect(stillParathread.toString()).eq("Parathread");

                // Call registering a para

                const PARA_ID = 5556;
                const txReserve = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.registrar.forceRegister(
                            charlie.address,
                            50,
                            PARA_ID,
                            GENESIS_HEAD,
                            VALIDATION_CODE
                        )
                    )
                );
                await context.createBlock([await txReserve.signAsync(charlie)]);

                const registrar_info = await polkadotJs.query.registrar.paras(PARA_ID);
                expect(registrar_info.toJSON()).toBeNull();

                // registering a profile

                const profile = {
                    url: "The URL",
                    paraIds: { whitelist: [PARA_ID] },
                    mode: "Bootnode",
                };

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const txProfile = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.dataPreservers.forceCreateProfile(profile, charlie.address))
                );
                await context.createBlock([await txProfile.signAsync(charlie)]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.isEmpty).to.be.true;

                // Deregistering a chain

                const txStartCollating = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.containerRegistrar.deregister(2002))
                );
                await context.createBlock([await txStartCollating.signAsync(charlie)]);

                const events = await polkadotJs.query.system.events();
                const startCollatingEvent = events.filter(
                    ({ event }) => event.method === "ParaIdValidForCollating" && event.data[0].toString() === "2002"
                );

                expect(startCollatingEvent.length).eq(0);
            },
        });
    },
});
