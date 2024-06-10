import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "CT1001",
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
                    deposit: 10_210_000_000_000,
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
                await context.createBlock(); // session boundary block cannot contain tx
                await context.createBlock([signedTx2]);

                const storedProfile2 = await polkadotJs.query.dataPreservers.profiles(profileId);
                expect(storedProfile2.toJSON()).to.be.equal(null);
            },
        });
    },
});
