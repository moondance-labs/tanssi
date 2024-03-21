import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { jumpSessions, fetchIssuance } from "util/block";
import { paraIdTank } from "util/payment";

describeSuite({
    id: "CT0604",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        const paraId2001 = 2001n;
        const costPerBlock = 1_000_000n;
        let balanceTankBefore;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const tx2000OneSession = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0);
            await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2000OneSession).signAsync(alice)]);
            const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
            // Now, buy some credits for container chain 2001
            const purchasedCredits = blocksPerSession * costPerBlock + existentialDeposit;
            const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, purchasedCredits);
            await context.createBlock([await tx.signAsync(alice)]);
            balanceTankBefore = (await polkadotJs.query.system.account(paraIdTank(paraId2001))).data.free.toBigInt();
            expect(balanceTankBefore, `Tank should have been filled`).toBe(purchasedCredits);
        });
        it({
            id: "E01",
            title: "We deregister 2000, check the issuance drops",
            test: async function () {
                // We deregister the chain
                const deregister2001 = polkadotJs.tx.sudo.sudo(polkadotJs.tx.registrar.deregister(paraId2001));
                await context.createBlock([await deregister2001.signAsync(alice)]);
                // Check that after 2 sessions, tank is empty and chain is deregistered
                await jumpSessions(context, 2);
                const balanceTank = (
                    await polkadotJs.query.system.account(paraIdTank(paraId2001))
                ).data.free.toBigInt();
                expect(balanceTank, `Tank should have been removed`).toBe(0n);

                const blockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toNumber();
                const apiAtBlockBefore = await polkadotJs.at(await polkadotJs.rpc.chain.getBlockHash(blockNumber - 1));
                const supplyBefore = (await apiAtBlockBefore.query.balances.totalIssuance()).toBigInt();
                const supplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                const blockIssuance = await fetchIssuance(await polkadotJs.query.system.events());
                const issuanceDiff = supplyAfter - supplyBefore;
                expect(issuanceDiff, `Tank should have been removed`).toBe(
                    blockIssuance.amount.toBigInt() - balanceTankBefore
                );
            },
        });
    },
});
