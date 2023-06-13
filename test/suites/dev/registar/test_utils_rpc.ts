import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

import "@polkadot/api-augment";

describeSuite({
  id: "D07",
  title: "Test utils RPC",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob;
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Checking that fetching registered paraIds is possible",
        test: async function () {
            const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

            // These are registered in genesis
            expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);
        },
      });
    },
});
