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
            id: "E02",
            title: "Create proposal locks minimum bond from proposer",
            test: async function () {
                // Gets the initial reserved amount from the proposer
                const proposer_initial_balance = await polkadotJs.query.system.account(user_charlie.address);
                const proposer_initial_reserved_balance = proposer_initial_balance.data.reserved.toBigInt();

                // minimum configured bond > 5% of the proposal
                const tx = polkadotJs.tx.treasury.proposeSpend(1, user_dave.address);
                const signedTx = await tx.signAsync(user_charlie);
                await context.createBlock([signedTx]);

                const proposer_new_balance = await polkadotJs.query.system.account(user_charlie.address);
                const proposer_new_reserved_balance = proposer_new_balance.data.reserved.toBigInt();

                // reserved value should be the minimum bond
                expect(proposer_new_reserved_balance).to.be.equal(
                    proposer_initial_reserved_balance + 1_000_000_000_000n * 100n
                );
            },
        });

        it({
            id: "E03",
            title: "Create proposal locks 5% of the proposal from proposer's account",
            test: async function () {
                // Gets the initial reserved amount from the proposer
                const proposer_initial_balance = await polkadotJs.query.system.account(user_dave.address);
                const proposer_initial_reserved_balance = proposer_initial_balance.data.reserved.toBigInt();

                // minimum configured bond > 5% of the proposal
                const proposal_value = 1_000_000_000_000_000_000n;
                const tx = polkadotJs.tx.treasury.proposeSpend(proposal_value, user_charlie.address);
                const signedTx = await tx.signAsync(user_dave);
                await context.createBlock([signedTx]);

                const proposer_new_balance = await polkadotJs.query.system.account(user_dave.address);
                const proposer_new_reserved_balance = proposer_new_balance.data.reserved.toBigInt();

                // reserved value should be 5% from the total amount requested in the proposal
                expect(proposer_new_reserved_balance).to.be.equal(
                    proposer_initial_reserved_balance + (proposal_value * 5n) / 100n
                );
            },
        });

        it({
            id: "E04",
            title: "Bond goes to treasury upon proposal rejection",
            test: async function () {
                // Gets the initial pot deposit value
                const initial_pot = await polkadotJs.query.system.account(treasury_address);
                const initial_free_pot = initial_pot.data.free.toBigInt();

                // Creates a proposal
                const proposal_value = 1_000_000_000_000_000_000n;
                const tx = polkadotJs.tx.treasury.proposeSpend(proposal_value, user_dave.address);
                const signedTx = await tx.signAsync(user_bob);
                await context.createBlock([signedTx]);

                // Proposal is rejected
                const tx_rejection = polkadotJs.tx.treasury.rejectProposal(2);
                const signedTx_rejection = await polkadotJs.tx.sudo.sudo(tx_rejection).signAsync(sudo_alice);
                await context.createBlock([signedTx_rejection]);

                // Gets the after rejection pot deposit value
                const new_pot = await polkadotJs.query.system.account(treasury_address);
                const new_free_pot = new_pot.data.free.toBigInt();

                // Pot value should be >= the initial value + reserved proposal bond
                expect(new_free_pot).toBeGreaterThan(initial_free_pot + (proposal_value * 5n) / 100n);
            },
        });

        it({
            id: "E05",
            title: "Proposal is approved",
            test: async function () {
                // initial approvals count
                const initial_approvals_count = await context.polkadotJs().query.treasury.approvals();

                // Creates a proposal
                const proposal_value = 100n;
                const tx = polkadotJs.tx.treasury.proposeSpend(proposal_value, user_dave.address);
                const signedTx = await tx.signAsync(user_bob);
                await context.createBlock([signedTx]);

                // Proposal is approved
                const tx_approval = polkadotJs.tx.treasury.approveProposal(3);
                const signedTx_approval = await polkadotJs.tx.sudo.sudo(tx_approval).signAsync(sudo_alice);
                await context.createBlock([signedTx_approval]);

                // New approvals count
                const new_approvals_count = await context.polkadotJs().query.treasury.approvals();

                // There should be 1 new approval
                expect(new_approvals_count.length).to.be.equal(initial_approvals_count.length + 1);
            },
        });

        it({
            id: "E06",
            title: "Non root can not approve proposals",
            test: async function () {
                // initial approvals count
                const initial_approvals_count = await context.polkadotJs().query.treasury.approvals();

                // Creates a proposal
                const proposal_value = 100n;
                const tx = polkadotJs.tx.treasury.proposeSpend(proposal_value, user_dave.address);
                const signedTx = await tx.signAsync(user_bob);
                await context.createBlock([signedTx]);

                // Proposal is approved
                const tx_approval = polkadotJs.tx.treasury.approveProposal(4);
                const signedTx_approval = await tx_approval.signAsync(user_charlie);
                await context.createBlock([signedTx_approval]);

                // New approvals count
                const new_approvals_count = await context.polkadotJs().query.treasury.approvals();

                // There should be no new approvals
                expect(new_approvals_count.length).to.be.equal(initial_approvals_count.length);
            },
        });
    },
});
