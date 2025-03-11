import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { type RawXcmMessage, XcmFragment, injectUmpMessageAndSeal, jumpToSession } from "utils";

describeSuite({
    id: "DEVT1903",
    title: "XCM - Succeeds sending XCM",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let random: KeyringPair;
        let transferredBalance: bigint;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

            random = generateKeyringPair("sr25519");

            transferredBalance = 100_000_000_000_000_000n;

            const location = {
                V3: {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                },
            };

            const locationToAccountResult = await polkadotJs.call.locationToAccountApi.convertLocation(location);
            expect(locationToAccountResult.isOk);

            const convertedAddress = locationToAccountResult.asOk.toJSON();

            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

            // Send some tokens to the sovereign account of para 2000
            const txSigned = polkadotJs.tx.balances.transferAllowDeath(convertedAddress, transferredBalance);
            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(convertedAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);
        });

        it({
            id: "T01",
            title: "Should succeed receiving tokens",
            test: async () => {
                const balanceRandomBefore = (
                    await polkadotJs.query.system.account(random.address)
                ).data.free.toBigInt();
                expect(balanceRandomBefore).to.eq(0n);

                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: { Here: null },
                            },
                            fungible: transferredBalance / 10n,
                        },
                    ],
                    beneficiary: u8aToHex(random.addressRaw),
                })
                    .withdraw_asset()
                    .buy_execution()
                    .deposit_asset_v3()
                    .as_v3();

                // Enable para inherent to process xcm message
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Wait until message is processed
                await jumpToSession(context, 3);
                await context.createBlock();

                const balanceRandomAfter = (await polkadotJs.query.system.account(random.address)).data.free.toBigInt();
                expect(Number(balanceRandomAfter)).to.be.greaterThan(0);
            },
        });
    },
});
