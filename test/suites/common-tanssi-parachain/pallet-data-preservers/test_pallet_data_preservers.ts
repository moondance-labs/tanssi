import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { generateEmptyGenesisData } from "utils";

describeSuite({
    id: "COMMO0401",
    title: "Data preservers pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let general_user_bob: KeyringPair;
        let profileId = 0;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            general_user_bob = context.keyring.charlie;
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
                await context.createBlock(); // session boundary block cannot contain tx
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

                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const registerTx = polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
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
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const registerTx = polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
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
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const registerTx = polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
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
                await context.createBlock();
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
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const registerTx = polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
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
            title: "Container will be unassigned on deregister",
            test: async () => {
                const paraId = 2006;
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const registerTx = polkadotJs.tx.registrar.register(paraId, containerChainGenesisData, null);
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
                const deregisterTx = polkadotJs.tx.registrar.deregister(paraId);
                await context.createBlock();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(deregisterTx).signAsync(sudo_alice)]);

                expect((await polkadotJs.query.dataPreservers.assignments(paraId)).toJSON()).to.deep.equal([]);
            },
        });
    },
});
