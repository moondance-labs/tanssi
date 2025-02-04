import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import { jumpSessions } from "util/block";
import { paraIdTank } from "util/payment";

describeSuite({
    id: "CT0102",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        const paraId2001 = 2001n;
        const costPerBlock = 1_000_000n;
        let refundAddress;
        let balanceTankBefore;
        let purchasedCredits;
        let registerAlias;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            registerAlias = runtimeName.includes("light") ? polkadotJs.tx.containerRegistrar : polkadotJs.tx.registrar;

            refundAddress = generateKeyringPair("sr25519");
            const tx2001OneSession = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0);
            await context.createBlock([await polkadotJs.tx.sudo.sudo(tx2001OneSession).signAsync(alice)]);
            const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
            // Now, buy some credits for container chain 2001
            purchasedCredits = blocksPerSession * costPerBlock + existentialDeposit;
            const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, purchasedCredits);
            await context.createBlock([await tx.signAsync(alice)]);
            balanceTankBefore = (await polkadotJs.query.system.account(paraIdTank(paraId2001))).data.free.toBigInt();
            expect(balanceTankBefore, `Tank should have been filled`).toBe(purchasedCredits);
        });
        it({
            id: "E01",
            title: "Sudo can set refund address",
            test: async () => {
                // We deregister the chain
                const setRefundAddress = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPayment.setRefundAddress(paraId2001, refundAddress.address)
                );
                await context.createBlock([await setRefundAddress.signAsync(alice)]);
                // Check that we can fetch the address
                const refundAddressOnChain = await polkadotJs.query.servicesPayment.refundAddress(paraId2001);
                expect(refundAddressOnChain.toString(), `Refund address should be set`).toBe(refundAddress.address);
            },
        });
        it({
            id: "E02",
            title: "On deregistration we refund the address",
            test: async () => {
                // We deregister the chain
                const deregister2001 = polkadotJs.tx.sudo.sudo(registerAlias.deregister(paraId2001));
                await context.createBlock([await deregister2001.signAsync(alice)]);
                // Check that after 2 sessions, tank is empty and chain is deregistered
                await jumpSessions(context, 2);
                const balanceTank = (
                    await polkadotJs.query.system.account(paraIdTank(paraId2001))
                ).data.free.toBigInt();
                expect(balanceTank, `Tank should have been removed`).toBe(0n);

                const balanceRefundAddress = (
                    await polkadotJs.query.system.account(refundAddress.address)
                ).data.free.toBigInt();

                expect(balanceRefundAddress).toBe(purchasedCredits);
            },
        });
    },
});
