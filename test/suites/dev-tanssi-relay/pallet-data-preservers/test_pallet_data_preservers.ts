import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR0601",
    title: "Data preservers pallet relay test suite",
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
            test: async function () {
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E02",
            title: "User can update profile",
            test: async function () {
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
                        assignmentRequest: "Free",
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E03",
            title: "User can delete profile",
            test: async function () {
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
                        assignmentRequest: "Free",
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
            test: async function () {
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E05",
            title: "Root can force update profile",
            test: async function () {
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
                        assignmentRequest: "Free",
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E06",
            title: "Root can force delete profile",
            test: async function () {
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
                        assignmentRequest: "Free",
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
            test: async function () {
                const paraId = 2002;
                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
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

                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    "0x010203"
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
                        assignmentRequest: "Free",
                    },
                    assignment: [paraId, "Free"],
                });
            },
        });

        it({
            id: "E08",
            title: "Profile can be force assigned",
            test: async function () {
                const paraId = 2003;
                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
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

                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    "0x010203"
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
                        assignmentRequest: "Free",
                    },
                    assignment: [paraId, "Free"],
                });
            },
        });

        it({
            id: "E09",
            title: "Profile can be unassigned",
            test: async function () {
                const paraId = 2004;
                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
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

                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    "0x010203"
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E10",
            title: "Profile can be force unassigned",
            test: async function () {
                const paraId = 2005;
                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
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

                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    "0x010203"
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
                        assignmentRequest: "Free",
                    },
                    assignment: null,
                });
            },
        });

        it({
            id: "E11",
            title: "Profile will be unassigned on container deregister",
            test: async function () {
                const paraId = 2006;
                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
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

                const registerTx = polkadotJs.tx.containerRegistrar.register(
                    paraId,
                    containerChainGenesisData,
                    "0x010203"
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
