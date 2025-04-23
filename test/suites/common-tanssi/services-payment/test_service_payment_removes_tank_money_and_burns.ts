import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { fetchIssuance, jumpSessions } from "utils";
import { paraIdTank } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SERVICES_PAYMENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "COMM0201",
    title: "Services payment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        const blocksPerSession = 10n;
        const paraId2001 = 2001;
        const costPerBlock = 1_000_000n;
        let balanceTankBefore: bigint;
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

            const tx2000OneSession = polkadotJs.tx.servicesPayment.setBlockProductionCredits(paraId2001, 0);
            if (shouldSkipStarlightSP) {
                console.log(`Skipping services payment tests for Starlight version ${specVersion}`);

                // We check that the call (without sudo) is filtered. 
                await checkCallIsFiltered(context, polkadotJs, await tx2000OneSession.signAsync(alice));

                // Purchase credits should be filtered too
                const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, 100n);
                await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                return;
            }

            const sudoSignedTx = await polkadotJs.tx.sudo.sudo(tx2000OneSession).signAsync(alice);
            await context.createBlock([sudoSignedTx]);
            const existentialDeposit = await polkadotJs.consts.balances.existentialDeposit.toBigInt();
            // Now, buy some credits for container chain 2001
            const purchasedCredits = blocksPerSession * costPerBlock + existentialDeposit;
            const tx = polkadotJs.tx.servicesPayment.purchaseCredits(paraId2001, purchasedCredits);
            await context.createBlock([await tx.signAsync(alice)]);
            balanceTankBefore = (await polkadotJs.query.system.account(paraIdTank(paraId2001))).data.free.toBigInt();
            expect(balanceTankBefore, "Tank should have been filled").toBe(purchasedCredits);
        });
        it({
            id: "E01",
            title: "We deregister 2000, check the issuance drops",
            test: async () => {
                if (shouldSkipStarlightSP) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
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

                const blockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toNumber();
                const apiAtBlockBefore = await polkadotJs.at(await polkadotJs.rpc.chain.getBlockHash(blockNumber - 1));
                const supplyBefore = (await apiAtBlockBefore.query.balances.totalIssuance()).toBigInt();
                const supplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                const blockIssuance = await fetchIssuance(await polkadotJs.query.system.events());

                const issuanceDiff = supplyAfter - supplyBefore;
                expect(issuanceDiff, "Tank should have been removed").toBe(
                    blockIssuance.amount.toBigInt() - balanceTankBefore
                );
            },
        });
    },
});
