import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
  id: "D0517",
  title: "Configuration - ActiveConfig - CollatorsPerContainer",
  foundationMethods: "dev",
  testCases: ({ context, log, it }) => {
    let alice;
    beforeAll(async function () {
      const keyring = new Keyring({ type: "sr25519" });
      alice = keyring.addFromUri("//Alice", { name: "Alice  default" });

      const config = await context
        .polkadotJs()
        .query.configuration.activeConfig();
      expect(config["collatorsPerContainer"].toString()).toBe("2");

      const { result } = await context.createBlock(
        context
          .polkadotJs()
          .tx.sudo.sudo(
            context.polkadotJs().tx.configuration.setCollatorsPerContainer(5)
          )
          .signAsync(alice)
      );
      expect(result!.successful, result!.error?.name).to.be.true;

      await jumpSessions(context, 2);
    });

    it({
      id: "T01",
      title: "should set collators per container after 2 sessions",
      test: async function () {
        const config = await context
          .polkadotJs()
          .query.configuration.activeConfig();
        expect(config["collatorsPerContainer"].toString()).toBe("5");
      },
    });
  },
});
