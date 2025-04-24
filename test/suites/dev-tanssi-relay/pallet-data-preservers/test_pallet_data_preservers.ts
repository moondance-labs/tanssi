import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { generateEmptyGenesisData, initializeCustomCreateBlock } from "utils";
import type { DpContainerChainGenesisDataContainerChainGenesisData } from "@polkadot/types/lookup";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_DATA_PRESERVERS, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT0901",
    title: "Data preservers pallet relay test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let general_user_bob: KeyringPair;
        let profileId = 0;
        let containerChainGenesisData: DpContainerChainGenesisDataContainerChainGenesisData;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightDP: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            general_user_bob = context.keyring.charlie;
            initializeCustomCreateBlock(context);
            containerChainGenesisData = generateEmptyGenesisData(context.pjsApi, true);

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightDP = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_DATA_PRESERVERS.includes(specVersion);
        });

        it({
            id: "E01",
            title: "User can create profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const tx = polkadotJs.tx.dataPreservers.createProfile(profile);
                const signedTx = await tx.signAsync(general_user_bob);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_200_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E02",
            title: "User can update profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                };

                const tx = polkadotJs.tx.dataPreservers.createProfile(profile);
                const signedTx = await tx.signAsync(general_user_bob);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);

                    // Update profile should be filtered too
                    const tx2 = polkadotJs.tx.dataPreservers.updateProfile(profileId, profile);
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(general_user_bob));
                    return;
                }

                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(++profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_200_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });

                const profile2 = {
                    url: "exemple2",
                    paraIds: { whitelist: [42, 43] },
                    mode: { Rpc: { supportsEthereumRpcs: false } },
                };

                const tx2 = polkadotJs.tx.dataPreservers.updateProfile(profileId, profile2);
                const signedTx2 = await tx2.signAsync(general_user_bob);
                await context.createBlock([signedTx2]);

                const storedProfile2 = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile2.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_220_000_000_000,
                    profile: {
                        url: "0x6578656d706c6532",
                        paraIds: { whitelist: [42, 43] },
                        mode: { rpc: { supportsEthereumRpcs: false } },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E03",
            title: "User can delete profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                };

                const tx = polkadotJs.tx.dataPreservers.createProfile(profile);
                const signedTx = await tx.signAsync(general_user_bob);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);

                    // Delete profile should be filtered too
                    const tx2 = polkadotJs.tx.dataPreservers.deleteProfile(profileId);
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(general_user_bob));
                    return;
                }

                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(++profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_200_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });

                const tx2 = polkadotJs.tx.dataPreservers.deleteProfile(profileId);
                const signedTx2 = await tx2.signAsync(general_user_bob);
                await context.createBlock([signedTx2]);

                const storedProfile2 = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile2.toJSON()).to.be.equal(null);
            },
        });

        it({
            id: "E04",
            title: "Root can force create profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                };

                const tx = polkadotJs.tx.dataPreservers.forceCreateProfile(profile, general_user_bob.address);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E04 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudo_alice));
                    return;
                }

                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);
                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(++profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 0,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E05",
            title: "Root can force update profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                };

                const tx = polkadotJs.tx.dataPreservers.createProfile(profile);
                const signedTx = await tx.signAsync(general_user_bob);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E05 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);

                    // Force update profile should be filtered too
                    const tx2 = polkadotJs.tx.dataPreservers.forceUpdateProfile(profileId, profile);
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(sudo_alice));
                    return;
                }
                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(++profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_200_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });

                const profile2 = {
                    url: "exemple2",
                    paraIds: { whitelist: [42, 43] },
                    mode: { Rpc: { supportsEthereumRpcs: false } },
                };

                const tx2 = polkadotJs.tx.dataPreservers.forceUpdateProfile(profileId, profile2);
                const signedTx2 = await polkadotJs.tx.sudo.sudo(tx2).signAsync(sudo_alice);
                await context.createBlock([signedTx2]);

                const storedProfile2 = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile2.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 0,
                    profile: {
                        url: "0x6578656d706c6532",
                        paraIds: { whitelist: [42, 43] },
                        mode: { rpc: { supportsEthereumRpcs: false } },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E06",
            title: "Root can force delete profile",
            test: async () => {
                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [42, 43] },
                    mode: "Bootnode",
                };

                const tx = polkadotJs.tx.dataPreservers.createProfile(profile);
                const signedTx = await tx.signAsync(general_user_bob);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E06 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);

                    // Force delete profile should be filtered too
                    const tx2 = polkadotJs.tx.dataPreservers.forceDeleteProfile(profileId);
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(sudo_alice));
                    return;
                }
                await context.createBlock([signedTx]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(++profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_200_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [42, 43] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });

                const tx2 = polkadotJs.tx.dataPreservers.forceDeleteProfile(profileId);
                const signedTx2 = await polkadotJs.tx.sudo.sudo(tx2).signAsync(sudo_alice);
                await context.createBlock([signedTx2]);

                const storedProfile2 = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile2.toJSON()).to.be.equal(null);
            },
        });

        it({
            id: "E07",
            title: "Profile can be assigned",
            test: async () => {
                const paraId = 2002;
                await context.createBlock([]);

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E07 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free").signAsync(sudo_alice));  
                    return;
                }
                
                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(sudo_alice)]);
                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    containerChainGenesisData.storage[0].value
                );
                await context.createBlock([await registerTx.signAsync(sudo_alice)]);

                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [paraId] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                await context.createBlock([await profileTx.signAsync(general_user_bob)]);
                profileId++;

                const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                await context.createBlock([await assignTx.signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([profileId]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_160_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [paraId] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: [paraId, { free: null }],
                });
            },
        });

        it({
            id: "E08",
            title: "Profile can be force assigned",
            test: async () => {
                const paraId = 2003;
                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E08 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.dataPreservers.forceStartAssignment(profileId, paraId, "Free").signAsync(sudo_alice));  
                    return;
                }

                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(sudo_alice)]);
                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    containerChainGenesisData.storage[0].value
                );
                await context.createBlock([await registerTx.signAsync(sudo_alice)]);

                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [paraId] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                await context.createBlock([await profileTx.signAsync(general_user_bob)]);
                ++profileId;

                const assignTx = polkadotJs.tx.dataPreservers.forceStartAssignment(profileId, paraId, "Free");
                await context.createBlock([await polkadotJs.tx.sudo.sudo(assignTx).signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([profileId]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_160_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [paraId] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: [paraId, { free: null }],
                });
            },
        });

        it({
            id: "E09",
            title: "Profile can be unassigned",
            test: async () => {
                const paraId = 2004;

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E09 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.dataPreservers.stopAssignment(profileId, paraId).signAsync(sudo_alice));  
                    return;
                }

                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(sudo_alice)]);
                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    containerChainGenesisData.storage[0].value
                );
                await context.createBlock([]);
                await context.createBlock([await registerTx.signAsync(sudo_alice)]);

                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [paraId] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                await context.createBlock([await profileTx.signAsync(general_user_bob)]);
                profileId++;

                const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                const unassignTx = polkadotJs.tx.dataPreservers.stopAssignment(profileId, paraId);
                await context.createBlock([await assignTx.signAsync(sudo_alice)]);
                await context.createBlock([await unassignTx.signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_160_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [paraId] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E10",
            title: "Profile can be force unassigned",
            test: async () => {
                const paraId = 2005;
                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E10 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.dataPreservers.stopAssignment(profileId, paraId).signAsync(sudo_alice));  
                    return;
                }
                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(sudo_alice)]);
                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    containerChainGenesisData.storage[0].value
                );
                await context.createBlock([await registerTx.signAsync(sudo_alice)]);

                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [paraId] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                await context.createBlock([await profileTx.signAsync(general_user_bob)]);
                profileId++;

                const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                const unassignTx = polkadotJs.tx.dataPreservers.stopAssignment(profileId, paraId);
                await context.createBlock([await assignTx.signAsync(sudo_alice)]);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(unassignTx).signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([]);

                const storedProfile = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile.toJSON()).to.be.deep.equal({
                    account: general_user_bob.address,
                    deposit: 10_160_000_000_000,
                    profile: {
                        url: "0x6578656d706c65",
                        paraIds: { whitelist: [paraId] },
                        mode: { bootnode: null },
                        assignmentRequest: { free: null },
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E11",
            title: "Profile will be unassigned on container deregister",
            test: async () => {
                const paraId = 2006;

                if (shouldSkipStarlightDP) {
                    console.log(`Skipping E11 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await polkadotJs.tx.dataPreservers.stopAssignment(profileId, paraId).signAsync(sudo_alice));  
                    return;
                }

                await context.createBlock([]);
                await context.createBlock([await polkadotJs.tx.registrar.reserve().signAsync(sudo_alice)]);
                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    containerChainGenesisData.storage[0].value
                );
                await context.createBlock([await registerTx.signAsync(sudo_alice)]);

                const profile = {
                    url: "exemple",
                    paraIds: { whitelist: [paraId] },
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                };

                const profileTx = polkadotJs.tx.dataPreservers.createProfile(profile);
                await context.createBlock([await profileTx.signAsync(general_user_bob)]);
                profileId++;

                const assignTx = polkadotJs.tx.dataPreservers.startAssignment(profileId, paraId, "Free");
                await context.createBlock([await assignTx.signAsync(sudo_alice)]);

                // Deregistering the container will remove the assignment
                const deregisterTx = polkadotJs.tx.containerRegistrar.deregister(paraId);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(deregisterTx).signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([]);
            },
        });
    },
});
