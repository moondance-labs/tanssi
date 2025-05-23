import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "utils";
import { checkCallIsFiltered } from "helpers";
import { DANCE } from "utils";

describeSuite({
    id: "C0401",
    title: "Maintenance mode test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let randomAccount: KeyringPair;
        let chain: string;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            polkadotJs = context.pjsApi;
            chain = polkadotJs.consts.system.version.specName.toString();
            alice = context.keyring.alice;
            randomAccount = chain === "frontier-template" ? generateKeyringPair() : generateKeyringPair("sr25519");
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
        });

        it({
            id: "E01",
            title: "No maintenance mode at genesis",
            test: async () => {
                await context.createBlock();
                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.false;
            },
        });

        it({
            id: "E02",
            title: "Signed origin cannot enable maintenance mode",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev1.length).to.be.equal(1);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.false;
            },
        });

        it({
            id: "E03",
            title: "Root origin can enable maintenance mode",
            test: async () => {
                await context.createBlock();
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "EnteredMaintenanceMode";
                });
                expect(ev1.length).to.be.equal(1);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;
            },
        });

        it({
            id: "E04",
            title: "No transfers allowed in maintenance mode",
            test: async () => {
                await context.createBlock();
                await context.createBlock();

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

                const balanceBefore = (await polkadotJs.query.system.account(randomAccount.address)).data.free;

                const tx = polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, DANCE);

                if (chain === "frontier-template") {
                    expect(await context.createBlock([await tx.signAsync(alice)]).catch((e) => e.toString())).to.equal(
                        "RpcError: 1010: Invalid Transaction: Transaction call is not expected"
                    );
                } else {
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                }

                const balanceAfter = (await polkadotJs.query.system.account(randomAccount.address)).data.free;

                expect(balanceBefore.eq(balanceAfter)).to.be.true;
            },
        });

        it({
            id: "E05",
            title: "Transfer with sudo allowed in maintenance mode",
            test: async () => {
                await context.createBlock();
                await context.createBlock();

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;

                const balanceBefore = (await polkadotJs.query.system.account(randomAccount.address)).data.free;
                expect(balanceBefore.toBigInt()).to.be.eq(0n);

                // We need to use forceTransfer because transfer doesn't work with sudo
                const tx = polkadotJs.tx.balances.forceTransfer(alice.address, randomAccount.address, DANCE);

                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);
                const balanceAfter = (await polkadotJs.query.system.account(randomAccount.address)).data.free;

                expect(balanceBefore.lt(balanceAfter)).to.be.true;
            },
        });

        it({
            id: "E06",
            title: "Signed origin cannot disable maintenance mode",
            test: async () => {
                await context.createBlock();
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.resumeNormalOperation();
                await context.createBlock([await tx.signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev1.length).to.be.equal(1);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.true;
            },
        });

        it({
            id: "E07",
            title: "Root origin can disable maintenance mode",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.maintenanceMode.resumeNormalOperation();
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "NormalOperationResumed";
                });
                expect(ev1.length).to.be.equal(1);

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.false;
            },
        });

        it({
            id: "E08",
            title: "Transfers allowed again after disabling maintenance mode",
            test: async () => {
                await context.createBlock();

                const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
                expect(enabled).to.be.false;

                const balanceBefore = (await polkadotJs.query.system.account(randomAccount.address)).data.free;
                const tx = polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, DANCE);

                await context.createBlock([await tx.signAsync(alice)]);
                const balanceAfter = (await polkadotJs.query.system.account(randomAccount.address)).data.free;
                expect(balanceBefore.lt(balanceAfter)).to.be.true;
            },
        });
    },
});
