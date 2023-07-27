import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import "@polkadot/api-augment";
import { initializeCustomCreateBlock } from "../../../util/block";

describeSuite({
  id: "D10",
  title: "Maintenance mode test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    let alice, bob, charlie, dave;
    initializeCustomCreateBlock(context);

    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      charlie = keyring.addFromUri('//Charlie', { name: 'Charlie default' });
      dave = keyring.addFromUri('//Dave', { name: 'Dave default' });
      polkadotJs = context.polkadotJs();
    });

    it({
      id: "E01",
      title: "No maintenance mode at genesis",
      test: async function () {
        await context.createBlock();
        const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
        expect(enabled).to.be.false;
      },
    });

    it({
      id: "E02",
      title: "Signed origin cannot enable maintenance mode",
      test: async function () {
        await context.createBlock();

        const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
        await context.createBlock([
          await tx.signAsync(alice),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ExtrinsicFailed";
        });
        expect(ev1.length).to.be.equal(1);

        const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
        expect(enabled).to.be.false;
      },
    });

    it({
      id: "E03",
      title: "Root origin can enable maintenance mode",
      test: async function () {
        await context.createBlock();
        await context.createBlock();

        const tx = polkadotJs.tx.maintenanceMode.enterMaintenanceMode();
        await context.createBlock([
          await polkadotJs.tx.sudo.sudo(tx).signAsync(alice),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "EnteredMaintenanceMode";
        });
        expect(ev1.length).to.be.equal(1);

        const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
        expect(enabled).to.be.true;
      },
    });

    it({
      id: "E04",
      title: "No transfers allowed in maintenance mode",
      test: async function () {
        await context.createBlock();

        const enabled = (await polkadotJs.query.maintenanceMode.maintenanceMode()).toJSON();
        expect(enabled).to.be.true;

        const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;

        const tx = polkadotJs.tx.balances
          .transfer(bob.address, 1000);

        await context.createBlock([
            await tx.signAsync(alice),
        ]);
        const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;

        console.log("Before, after", balanceBefore, " / ", balanceAfter);

        expect(balanceBefore.eq(balanceAfter)).to.be.true;
      },
    });
    },
});
