import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import {
    RawXcmMessage,
    XcmFragment,
    descendParentOriginForAddress20,
    descendParentOriginFromAddress32,
    injectDmpMessageAndSeal,
} from "../../../util/xcm.ts";

describeSuite({
    id: "CX0201",
    title: "Mock XCM - Succeeds using sovereign accounts",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let transferredBalance;
        let sendingAddress;
        let alice: KeyringPair;
        let chain;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            const descendFunction =
                chain == "frontier-template" ? descendParentOriginForAddress20 : descendParentOriginFromAddress32;
            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();
            const { originAddress, descendOriginAddress } = descendFunction(context);

            sendingAddress = originAddress;
            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;

            const txSigned = polkadotJs.tx.balances.transferAllowDeath(descendOriginAddress, transferredBalance);

            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(descendOriginAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);
        });

        it({
            id: "T01",
            title: "Should succeed using sovereign account from signed origin",
            test: async function () {
                // Generate random receiver address
                const random = chain == "frontier-template" ? generateKeyringPair() : generateKeyringPair("sr25519");
                // Get Pallet balances index
                const metadata = await polkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() == "Balances")!
                    .index.toNumber();

                const transferCall = polkadotJs.tx.balances.transferAllowDeath(
                    random.address,
                    transferredBalance / 10n
                );
                const transferCallEncoded = transferCall?.method.toHex();

                // We are going to test that we can receive a transact operation from parachain 1
                // using descendOrigin first
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 0,
                                interior: {
                                    X1: { PalletInstance: balancesPalletIndex },
                                },
                            },
                            fungible: transferredBalance / 4n,
                        },
                    ],
                    descend_origin: sendingAddress,
                })
                    .descend_origin()
                    .withdraw_asset()
                    .buy_execution()
                    .push_any({
                        Transact: {
                            originKind: "SovereignAccount",
                            requireWeightAtMost: {
                                refTime: 1000000000,
                                proofSize: 32000,
                            },
                            call: {
                                encoded: transferCallEncoded,
                            },
                        },
                    })
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();

                // Make sure the state has ALITH's foreign parachain tokens
                const testAccountBalance = (await polkadotJs.query.system.account(random.address)).data.free.toBigInt();

                expect(testAccountBalance).to.eq(transferredBalance / 10n);
            },
        });
    },
});
