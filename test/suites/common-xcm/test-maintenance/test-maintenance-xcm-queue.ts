import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith, generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import {
    RawXcmMessage,
    XcmFragment,
    descendSiblingOriginFromAddress20,
    descendSiblingOriginFromAddress32,
    injectHrmpMessageAndSeal,
    sovereignAccountOfSiblingForAddress20,
    sovereignAccountOfSiblingForAddress32,
} from "../../../util/xcm.ts";

describeSuite({
    id: "CX0104",
    title: "Maintenance mode - XCM queue",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let transferredBalance: bigint;
        let sendingAddress: `0x${string}`;
        let alice: KeyringPair;
        let chain: string;
        let random: KeyringPair;
        let xcmMessage: XcmFragment;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();

            // Generate the proper Keyring for the current type of chain
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

            const descendFunction =
                chain == "frontier-template" ? descendSiblingOriginFromAddress20 : descendSiblingOriginFromAddress32;
            const sovereignFunction =
                chain == "frontier-template"
                    ? sovereignAccountOfSiblingForAddress20
                    : sovereignAccountOfSiblingForAddress32;

            // Generate the sibling sovereign and derivative accounts
            const { originAddress, descendOriginAddress } = descendFunction(context);
            const sovereign = sovereignFunction(context, 1);
            sendingAddress = originAddress;

            // Transfer some tokens to sovereign and derivative accounts for execution costs
            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;
            polkadotJs = context.polkadotJs();

            const txSigned = polkadotJs.tx.balances.transferAllowDeath(descendOriginAddress, transferredBalance);
            const txRoot = polkadotJs.tx.balances.transferAllowDeath(sovereign, transferredBalance);

            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });
            await context.createBlock(await txRoot.signAsync(alice, { nonce: aliceNonce++ }), { allowFailures: false });
            const balanceSigned = (await polkadotJs.query.system.account(descendOriginAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);
            const balanceRoot = (await polkadotJs.query.system.account(sovereign)).data.free.toBigInt();
            expect(balanceRoot).to.eq(transferredBalance);

            // Now let's start building the message
            // Generate random receiver address
            random = chain == "frontier-template" ? generateKeyringPair() : generateKeyringPair("sr25519");

            // Get Pallet balances index
            const metadata = await polkadotJs.rpc.state.getMetadata();
            const balancesPalletIndex = metadata.asLatest.pallets
                .find(({ name }) => name.toString() == "Balances")!
                .index.toNumber();

            const transferCall = polkadotJs.tx.balances.transferAllowDeath(random.address, transferredBalance / 10n);
            const transferCallEncoded = transferCall?.method.toHex();

            // Build the XCM message
            xcmMessage = new XcmFragment({
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
        });

        it({
            id: "T01",
            title: "Should queue XCM execution during maintenance mode (HRMP)",
            test: async function () {
                // Enter maintenance mode with sudo
                const maintenanceTx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(maintenanceTx).signAsync(alice)]);

                // Ensure we are in maintenance mode
                let maintenanceOn = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(maintenanceOn).to.be.true;

                // This XCM message coming by HRMP should not be executed since we are in maintenance mode
                await injectHrmpMessageAndSeal(context, 1, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Make sure the random address has zero balance
                const balance = (await polkadotJs.query.system.account(random.address)).data.free.toBigInt();
                expect(balance).to.eq(0n);

                // ---- Now let's disable maintenance mode ----

                // Disable maintenance mode with sudo
                const resumeTx = polkadotJs.tx.maintenanceMode.resumeNormalOperation();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(resumeTx).signAsync(alice)]);

                // Create a block in which the XCM message will be executed
                // MessageQueue takes two blocks to resume execution
                await context.createBlock();
                await context.createBlock();

                // Ensure we are NOT in maintenance mode
                maintenanceOn = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(maintenanceOn).to.be.false;

                // Make sure the random address has received the tokens
                const balanceAfter = (await polkadotJs.query.system.account(random.address)).data.free.toBigInt();
                expect(balanceAfter).to.eq(transferredBalance / 10n);
            },
        });
    },
});
