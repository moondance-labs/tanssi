import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";
import { paraIdTank } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "COMM0202",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        const paraId2001 = 2001;
        const costPerBlock = 1_000_000n;
        let refundAddress: KeyringPair;
        let balanceTankBefore: bigint;
        let purchasedCredits: bigint;
        let registerAlias: any;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightSP: boolean;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            registerAlias = runtimeName.includes("light") ? polkadotJs.tx.containerRegistrar : polkadotJs.tx.registrar;

            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSP = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT.includes(specVersion);

            refundAddress = generateKeyringPair("sr25519");
            const tx2001OneSession = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0);

            if (shouldSkipStarlightSP) {
                console.log(`Services payment tests for Starlight version ${specVersion}`);

                // We check that the call (without sudo) is filtered.
                await checkCallIsFiltered(context, polkadotJs, await tx2001OneSession.signAsync(alice));

                // Purchase credits should be filtered too
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, 100n);
                await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                return;
            }

            const sudoSignedTx = await polkadotJs.tx.sudo.sudo(tx2001OneSession).signAsync(alice);
            await context.createBlock([sudoSignedTx]);
            const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
            // Now, buy some credits for container chain 2001
            purchasedCredits = blocksPerSession * costPerBlock + existentialDeposit;
            const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, purchasedCredits);
            await context.createBlock([await tx.signAsync(alice)]);
            balanceTankBefore = (await polkadotJs.query.system.account(paraIdTank(paraId2001))).data.free.toBigInt();
            expect(balanceTankBefore, "Tank should have been filled").toBe(purchasedCredits);
        });
        it({
            id: "E01",
            title: "Sudo can set refund address",
            test: async () => {
                // We deregister the chain

                const setRefundAddress = polkadotJs.tx.servicesPayment.setRefundAddress(paraId2001, refundAddress.address);
                
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await setRefundAddress.signAsync(alice));
                    return;
                }

                const setRefundAddressSudo = polkadotJs.tx.sudo.sudo(setRefundAddress);
                await context.createBlock([await setRefundAddressSudo.signAsync(alice)]);
                // Check that we can fetch the address
                const refundAddressOnChain = await polkadotJs.query.servicesPayment.refundAddress(paraId2001);
                expect(refundAddressOnChain.toString(), "Refund address should be set").toBe(refundAddress.address);
            },
        });
        it({
            id: "E02",
            title: "On deregistration we refund the address",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    return;
                }
                // We deregister the chain
                const deregister2001 = polkadotJs.tx.sudo.sudo(registerAlias.deregister(paraId2001));
                await context.createBlock([await deregister2001.signAsync(alice)]);
                // Check that after 2 sessions, tank is empty and chain is deregistered
                await jumpSessions(context, 2);
                const balanceTank = (
                    await polkadotJs.query.system.account(paraIdTank(paraId2001))
                ).data.free.toBigInt();
                expect(balanceTank, "Tank should have been removed").toBe(0n);

                const balanceRefundAddress = (
                    await polkadotJs.query.system.account(refundAddress.address)
                ).data.free.toBigInt();

                expect(balanceRefundAddress).toBe(purchasedCredits);
            },
        });
    },
});
