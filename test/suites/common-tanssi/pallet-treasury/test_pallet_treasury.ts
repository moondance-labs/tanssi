import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpBlocks } from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_TREASURY,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES,
    checkCallIsFiltered,
} from "helpers";

describeSuite({
    id: "COMM0101",
    title: "Treasury pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let user_dave: KeyringPair;
        let user_bob: KeyringPair;
        let chain: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightBalances: boolean;
        let shouldSkipStarlightTreasury: boolean;
        // From Pallet Id "py/trsry" -> Account if relay chain
        // From Pallet Id "tns/tsry" -> Account if parachain

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            user_dave = context.keyring.dave;
            user_bob = context.keyring.bob;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            const treasuryAddress = runtimeName.includes("light")
                ? "5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z"
                : "5EYCAe5jXiVvytpxmBupXPCNE9Vduq7gPeTwy9xMgQtKWMnR";

            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightBalances =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES.includes(specVersion);
            shouldSkipStarlightTreasury =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_TREASURY.includes(specVersion);

            const tx = polkadotJs.tx.balances.transferAllowDeath(treasuryAddress, 1_000_000_000_000);

            const signedTx = await tx.signAsync(sudo_alice);

            if (shouldSkipStarlightBalances) {
                console.log(`Skipping balance transfer to Treasury account: Starlight version ${specVersion}`);
                await checkCallIsFiltered(context, polkadotJs, signedTx);
                return;
            }

            // Fund treasury account
            await context.createBlock([signedTx]);
        });

        it({
            id: "E01",
            title: "Non root can not spend from treasury (Local)",
            test: async () => {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);

                // Creates a proposal
                const proposal_value = 1000000000n;
                const tx = polkadotJs.tx.treasury.spendLocal(proposal_value, user_dave.address);
                const signedTx = await tx.signAsync(user_bob);

                if (shouldSkipStarlightTreasury) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
            },
        });

        it({
            id: "E02",
            title: "Root can spend from treasury (Local)",
            test: async () => {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
                const balanceBefore = (await polkadotJs.query.system.account(user_dave.address)).data.free.toBigInt();

                // Creates a proposal
                // Value needs to be higher than the transaction fee paid by dave, but lower than the total treasury pot
                const proposal_value = 1000000000n;
                const tx = polkadotJs.tx.treasury.spendLocal(proposal_value, user_dave.address);
                const sudoSignedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);

                if (shouldSkipStarlightTreasury) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);

                    // We check that the call (without sudo pallet) is filtered.
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudo_alice));
                    return;
                }

                await context.createBlock([sudoSignedTx]);

                // Local spends dont upadte the spend count
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);

                // After the payout is approved, we need to wait SpendPeriod for the payout to happen
                const spendPeriod = polkadotJs.consts.treasury.spendPeriod;

                // Now we just wait the spendPeriod, no need for payout calls in local spends
                await jumpBlocks(context, spendPeriod.toNumber());

                const balanceAfter = (await polkadotJs.query.system.account(user_dave.address)).data.free.toBigInt();
                expect(balanceAfter).toBeGreaterThan(balanceBefore);
            },
        });

        it({
            id: "E03",
            title: "Non root can not spend from treasury (Non-local)",
            test: async () => {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);

                // Creates a proposal
                const proposal_value = 1000000000n;
                const assetKind = null;
                const tx = polkadotJs.tx.treasury.spend(assetKind, proposal_value, user_dave.address, null);
                const signedTx = await tx.signAsync(user_bob);

                if (shouldSkipStarlightTreasury) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
            },
        });

        it({
            id: "E04",
            title: "Root can spend from treasury (Non-local)",
            test: async () => {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
                const balanceBefore = (await polkadotJs.query.system.account(user_dave.address)).data.free.toBigInt();

                // Creates a proposal
                // Value needs to be higher than the transaction fee paid by dave, but lower than the total treasury pot
                const proposal_value = 1000000000n;
                const assetKind = null;
                const tx = polkadotJs.tx.treasury.spend(assetKind, proposal_value, user_dave.address, null);
                const sudoSignedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);

                if (shouldSkipStarlightTreasury) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);

                    // We check that the call (without sudo pallet) is filtered.
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudo_alice));

                    // Payouts also filtered
                    const tx2 = polkadotJs.tx.treasury.payout(0);
                    const signedTx2 = await tx2.signAsync(user_dave);
                    await checkCallIsFiltered(context, polkadotJs, signedTx2);

                    return;
                }

                await context.createBlock([sudoSignedTx]);

                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(1);

                // Dave needs to claim payout
                const tx2 = polkadotJs.tx.treasury.payout(0);
                const signedTx2 = await tx2.signAsync(user_dave);
                await context.createBlock([signedTx2]);

                const balanceAfter = (await polkadotJs.query.system.account(user_dave.address)).data.free.toBigInt();
                expect(balanceAfter).toBeGreaterThan(balanceBefore);
            },
        });
    },
});
