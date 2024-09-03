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
    },
});
