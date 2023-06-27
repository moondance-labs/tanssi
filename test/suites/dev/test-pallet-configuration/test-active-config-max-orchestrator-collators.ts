import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block.js";

describeSuite({
  id: "D0517",
  title: "Configuration - ActiveConfig - MaxOrchestratorCollators",
  foundationMethods: "dev",
  testCases: ({ context, log, it }) => {
    let alice;
    beforeAll(async function () {
      const keyring = new Keyring({ type: "sr25519" });
      alice = keyring.addFromUri("//Alice", { name: "Alice  default" });

      const config = await context
        .polkadotJs()
        .query.configuration.activeConfig();
      expect(config["maxOrchestratorCollators"].toString()).toBe("1");

      const { result } = await context.createBlock(
        context
          .polkadotJs()
          .tx.sudo.sudo(
            context.polkadotJs().tx.configuration.setMaxOrchestratorCollators(2)
          )
          .signAsync(alice)
      );
      expect(result!.successful, result!.error?.name).to.be.true;

      await jumpSessions(context, 2);
    });

    it({
      id: "T01",
      title: "should set max orchestrator collators after 2 sessions",
      test: async function () {
        const config = await context
          .polkadotJs()
          .query.configuration.activeConfig();
        expect(config["maxOrchestratorCollators"].toString()).toBe("2");
      },
    });
  },
});
