import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { extractFeeAuthor } from "util/block";

describeSuite({
    id: "CT0901",
    title: "Treasury pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let user_charlie: KeyringPair;
        let user_dave: KeyringPair;
        let user_bob: KeyringPair;
        // From Pallet Id "tns/tsry" -> Account
        const treasury_address = "5EYCAe5jXiVvytpxmBupXPCNE9Vduq7gPeTwy9xMgQtKWMnR";

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            user_charlie = context.keyring.charlie;
            user_dave = context.keyring.dave;
            user_bob = context.keyring.bob;
        });

        it({
            id: "E01",
            title: "20% of fees & tips go for treasury account",
            test: async function () {
                // Gets the initial pot deposit value
                const initial_pot = await polkadotJs.query.system.account(treasury_address);
                const initial_free_pot = initial_pot.data.free.toBigInt();

                // Executes a tx adding an additional tip
                const tx = polkadotJs.tx.balances.transferAllowDeath(user_charlie.address, 200_000);
                const signedTx = await tx.signAsync(user_dave, { tip: 100_000 });
                await context.createBlock([signedTx]);
                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, user_dave.address).amount.toBigInt();

                // Gets the new pot deposit value
                const new_pot = await polkadotJs.query.system.account(treasury_address);
                const new_free_pot = new_pot.data.free.toBigInt();

                // Division operation rounding
                const rounding = fee % 5n > 0 ? 1n : 0n;

                // Treasury pot should increase by 20% of the paid fee & tip
                expect(new_free_pot).to.be.equal(initial_free_pot + fee / 5n + rounding);
            },
        });

        it({
            id: "E06",
            title: "Non root can not spend from treasury",
            test: async function () {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);

                // Creates a proposal
                const proposal_value = 1000000000n;
                const assetKind = null;
                const tx = polkadotJs.tx.treasury.spend(assetKind, proposal_value, user_dave.address, null);
                const signedTx = await tx.signAsync(user_bob);
                await context.createBlock([signedTx]);

                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
            },
        });

        it({
            id: "E07",
            title: "Root can spend from treasury",
            test: async function () {
                expect((await polkadotJs.query.treasury.spendCount()).toNumber()).to.equal(0);
                const balanceBefore = (await polkadotJs.query.system.account(user_dave.address)).data.free.toBigInt();

                // Creates a proposal
                // Value needs to be higher than the transaction fee paid by dave, but lower than the total treasury pot
                const proposal_value = 1000000000n;
                const assetKind = null;
                const tx = polkadotJs.tx.treasury.spend(assetKind, proposal_value, user_dave.address, null);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);
                await context.createBlock([signedTx]);

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
